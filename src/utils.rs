use crate::config;
use anyhow::Result;
use fstrings::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn get_email(query: String) -> Result<String> {
    let cf_config = config::load_config()?;
    let root_domain = cf_config.cloudflare_root_domain;

    let mut email = f!("{query}@{root_domain}");
    if query == "random" {
        let random_word = memorable_wordlist::WORDS
            .choose(&mut rand::thread_rng())
            .unwrap();
        let num = rand::thread_rng().gen_range(1000..9999).to_string();
        email = f!("{random_word}-{num}@{root_domain}");
    }

    return Ok(email);
}
