#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod alfred;
mod cloudflare;
mod config;

use anyhow::Result;
use std::io;

async fn list_routes() -> Result<String> {
    let routes = cloudflare::list_routes().await?;
    let mut emails = routes
        .iter()
        .map(|e| return e.email.to_owned())
        .collect::<Vec<String>>();
    emails.sort();

    return Ok(emails.join("\n"));
}

fn build_cli() -> clap::Command<'static> {
    return clap::Command::new("cf-alias")
        .about("CLI interface for Cloudflare Email Routing")
        // .version(env!("VERGEN_GIT_SEMVER"))
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            clap::Command::new("alfred")
                .about("Commands for the Alfred extension")
                .subcommand(
                    clap::Command::new("clipboard")
                        .about("Copys email to clipboard.")
                        .arg(
                            clap::Arg::new("email")
                                .short('e')
                                .long("email")
                                .help("Email to copy")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(false),
                        ),
                )
                .subcommand(
                    clap::Command::new("create")
                        .about("Creates a new forwarding email")
                        .arg(
                            clap::Arg::new("email-prefix")
                                .short('e')
                                .long("email-prefix")
                                .help("Forwarding email prefix to create")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(false),
                        ),
                )
                .subcommand(
                    clap::Command::new("create-list")
                        .about("Autocomplete command used for creating new emails")
                        .arg(
                            clap::Arg::new("query")
                                .short('q')
                                .long("query")
                                .help("Command query or email prefix to be used when creating a new email")
                                .required(false)
                                .takes_value(true)
                                .multiple_values(false),
                        ),
                )
                .subcommand(
                    clap::Command::new("manage").about("Opens the Cloudflare Email management UI."),
                )
                .subcommand(clap::Command::new("list").about("List existing email routes.")),
        )
        .subcommand(
            clap::Command::new("completion")
                .about("Generates shell completions")
                .arg(
                    clap::Arg::new("shell")
                        .short('s')
                        .long("shell")
                        .help("Which shell to generate completions for.")
                        .possible_values(clap_complete::Shell::possible_values())
                        .required(true),
                ),
        )
        .subcommand(clap::Command::new("list").about("List existing email routes."));
}

fn print_completions<G: clap_complete::Generator>(gen: G, app: &mut clap::Command) {
    clap_complete::generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}

async fn parse_cli() -> Result<()> {
    let matches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("alfred", args)) => match args.subcommand() {
            Some(("create", run_matches)) => {
                let email_prefix = run_matches
                    .get_one::<String>("email-prefix")
                    .unwrap()
                    .to_string();
                alfred::create(email_prefix).await?;
            }
            Some(("create-list", run_matches)) => {
                let default_res = "".to_string();
                let query = run_matches
                    .get_one::<String>("query")
                    .unwrap_or_else(|| return &default_res)
                    .to_string();
                let res = alfred::create_list(query)?;
                println!("{}", res);
            }
            Some(("clipboard", run_matches)) => {
                let email = run_matches.get_one::<String>("email").unwrap().to_string();
                alfred::copy_to_clopboard(email);
            }
            Some(("manage", _)) => {
                alfred::open_manage()?;
            }
            Some(("list", _)) => {
                let emails = alfred::list_routes().await?;
                println!("{}", emails);
            }
            _ => unreachable!(),
        },
        Some(("completion", run_matches)) => {
            if let Ok(generator) = run_matches.value_of_t::<clap_complete::Shell>("shell") {
                eprintln!("Generating completion file for {}...", generator);
                let mut app = build_cli();
                print_completions(generator, &mut app);
            }
        }
        Some(("list", _)) => {
            let emails = list_routes().await?;
            println!("{}", emails);
        }
        _ => unreachable!(),
    }

    return Ok(());
}

#[tokio::main]
async fn main() {
    parse_cli().await.unwrap();
}
