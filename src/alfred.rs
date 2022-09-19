use crate::{cloudflare, config};
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

pub async fn create(email_prefix: String) -> Result<()> {
    let cf_config = config::load_config()?;
    let root_domain = cf_config.cloudflare_root_domain;
    let email = f!("{email_prefix}@{root_domain}");

    copy_to_clopboard(email.to_owned());
    cloudflare::create_route(email.to_owned()).await?;

    Notification::new()
        .summary("Cloudflare Emails")
        .body(&f!("{email} has been successfully created"))
        .icon("firefox")
        .show()?;

    thread::sleep(time::Duration::from_secs(5));

    return Ok(());
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
            let mut emoji = "🟥";
            if e.enabled {
                emoji = "✅";
            }

            return Item {
                title: f!("{emoji} {e.email}"),
                arg: f!("alfred clipboard -e {e.email}"),
                subtitle: "".to_string(),
            };
        })
        .collect::<Vec<Item>>();

    let mut items_arr: Vec<Item> = vec![
        Item {
            title: "Manage...".to_string(),
            arg: "alfred manage".to_string(),
            subtitle: "Open the Cloudflare Email Routes UI".to_string(),
        },
        Item {
            title: "Create".to_string(),
            arg: "alfred create -e".to_string(),
            subtitle: "Create a new forwarding email".to_string(),
        },
    ];

    items_arr.append(&mut routes);

    let items = Items { items: items_arr };

    let json_str = serde_json::to_string(&items)?;
    return Ok(json_str);
}
