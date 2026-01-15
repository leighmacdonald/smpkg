use clap::ArgMatches;
use smpkg::{
    commands::parser::commands,
    sourcemod::download::{fetch_latest_version, fetch_version},
};
use tokio;

async fn handle_sourcemod_latest(
    latest_version_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let branch = latest_version_matches
        .get_one::<String>("branch")
        .expect("Invalid branch");
    let result = fetch_latest_version(branch).await;
    match result {
        Ok(version) => {
            println!("Latest version: {version}");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

async fn handle_sourcemod_install(
    latest_version_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let branch = latest_version_matches
        .get_one::<String>("branch")
        .expect("Invalid branch")
        .clone();
    let result = fetch_version(branch).await;
    match result {
        Ok(version) => {
            println!("Latest version: {:?}", version);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = commands();

    match matches.subcommand() {
        Some(("sourcemod", sourcemod_matches)) => {
            let subcommand = sourcemod_matches
                .subcommand()
                .unwrap_or(("latest-version", sourcemod_matches));

            match subcommand {
                ("latest-version", sourcemod_matches) => {
                    handle_sourcemod_latest(sourcemod_matches).await?
                }
                ("install", sourcemod_matches) => {
                    handle_sourcemod_install(sourcemod_matches).await?
                }
                (_, _) => {
                    unreachable!()
                }
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
