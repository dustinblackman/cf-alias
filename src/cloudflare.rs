use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::config;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub enabled: bool,
    pub name: String,
    pub actions: Vec<Action>,
    pub matchers: Vec<Matcher>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesListResponse {
    pub result: Vec<RoutesListResult>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesListResult {
    pub tag: String,
    pub name: String,
    pub matchers: Vec<Matcher>,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub priority: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matcher {
    #[serde(rename = "type")]
    pub type_field: String,
    pub field: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub value: Vec<String>,
}

pub struct Email {
    pub email: String,
    pub forwarding_email: String,
    pub enabled: bool,
}

pub async fn list_routes() -> Result<Vec<Email>> {
    let cf_config = config::load_config()?;
    let token = cf_config.cloudflare_token;
    let zone = cf_config.cloudflare_zone;

    let routes_list = reqwest::Client::new()
        .get(format!(
            "https://api.cloudflare.com/client/v4/zones/{zone}/email/routing/rules"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<RoutesListResponse>()
        .await?;

    let mut emails = routes_list
        .result
        .iter()
        .map(|e| {
            let mut forwading_email = "".to_string();
            if !e.actions[0].value.is_empty() {
                forwading_email = e.actions[0].value[0].to_owned();
            }
            return Email {
                email: e.matchers[0].value.to_owned().unwrap_or_default(),
                forwarding_email: forwading_email,
                enabled: e.enabled,
            };
        })
        .filter(|e| return !e.email.is_empty())
        .collect::<Vec<Email>>();
    emails.sort_by_key(|e| return e.email.to_owned());

    return Ok(emails);
}

pub async fn create_route(email: String) -> Result<()> {
    let cf_config = config::load_config()?;
    let forward_email = cf_config.cloudflare_forward_email;
    let token = cf_config.cloudflare_token;
    let zone = cf_config.cloudflare_zone;

    let now = chrono::offset::Utc::now().to_string();
    let body = CreateRequest {
        enabled: true,
        name: format!("Rule created at {now}"),
        actions: vec![Action {
            type_field: "forward".to_string(),
            value: vec![forward_email],
        }],
        matchers: vec![Matcher {
            type_field: "literal".to_string(),
            field: Some("to".to_string()),
            value: Some(email),
        }],
    };

    reqwest::Client::new()
        .post(format!(
            "https://api.cloudflare.com/client/v4/zones/{zone}/email/routing/rules"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    return Ok(());
}
