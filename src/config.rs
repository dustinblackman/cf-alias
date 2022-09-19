use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    pub cloudflare_account_id: String,
    pub cloudflare_forward_email: String,
    pub cloudflare_root_domain: String,
    pub cloudflare_token: String,
    pub cloudflare_zone: String,
}

pub fn load_config() -> Result<Config> {
    let file_path = dirs::home_dir().unwrap().join(".cf-alias");
    if !file_path.exists() {
        return Err(anyhow!(
            "$HOME/.cf-alias does not exist. Refer to the documentation to get started."
        ));
    }

    let config_str = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&config_str)?;

    return Ok(config);
}
