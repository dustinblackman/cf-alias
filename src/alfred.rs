use crate::{cloudflare, config, utils};
use anyhow::Result;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use fstrings::*;
use notify_rust::Notification;
use serde::Serialize;
use std::{thread, time};

#[derive(Default, Debug, Clone, Serialize)]
struct Item {
    title: String,
    arg: String,
    subtitle: String,
}

#[derive(Default, Debug, Clone, Serialize)]
struct Items {
    items: Vec<Item>,
}

pub async fn create(email: String) -> Result<()> {
    copy_to_clopboard(email.to_owned());
    cloudflare::create_route(email.to_owned()).await?;

    Notification::new()
        .summary("cf-alias")
        .body(&f!("{email} has been successfully created"))
        .icon("email")
        .show()?;

    thread::sleep(time::Duration::from_secs(5));

    return Ok(());
}

pub fn create_list(query: String) -> Result<String> {
    let cf_config = config::load_config()?;
    let forwarding_email = cf_config.cloudflare_forward_email;

    let email = utils::get_email(query)?;
    let items = Items {
        items: vec![Item {
            title: email.to_owned(),
            arg: f!("alfred create -e {email}"),
            subtitle: f!("Create {email} forwarding to {forwarding_email}"),
        }],
    };

    let json_str = serde_json::to_string(&items)?;
    return Ok(json_str);
}

pub fn copy_to_clopboard(email: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(email).unwrap();
}

pub fn open_manage() -> Result<()> {
    let cf_config = config::load_config()?;
    let account_id = cf_config.cloudflare_account_id;
    let root_domain = cf_config.cloudflare_root_domain;

    open::that(f!(
        "https://dash.cloudflare.com/{account_id}/{root_domain}/email/routes"
    ))?;

    return Ok(());
}

pub async fn list_routes() -> Result<String> {
    let mut routes = cloudflare::list_routes()
        .await?
        .iter()
        .map(|e| {
            let mut emoji = "✅";
            let mut subtitle = f!("Forwarding to {e.forwarding_email}");
            if !e.enabled {
                emoji = "🟥";
                subtitle = f!("Disabled: {subtitle}");
            }

            return Item {
                title: f!("{emoji} {e.email}"),
                arg: f!("alfred clipboard -e {e.email}"),
                subtitle,
            };
        })
        .collect::<Vec<Item>>();

    let mut items_arr: Vec<Item> = vec![Item {
        title: "Manage...".to_string(),
        arg: "alfred manage".to_string(),
        subtitle: "Open the Cloudflare Email Routes UI".to_string(),
    }];

    items_arr.append(&mut routes);

    let items = Items { items: items_arr };

    let json_str = serde_json::to_string(&items)?;
    return Ok(json_str);
}
