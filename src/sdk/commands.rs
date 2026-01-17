use std::path::Path;

use clap::ArgMatches;

use crate::{
    CommandHandler,
    sdk::download::{fetch_latest_version, fetch_version},
};

pub struct SDKVersion {}

impl CommandHandler for SDKVersion {
    async fn execute(
        &self,
        _: &Path,
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
}

pub struct SDKInstaller {}

impl CommandHandler for SDKInstaller {
    async fn execute(
        &self,
        root: &Path,
        latest_version_matches: &ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let branch = latest_version_matches
            .get_one::<String>("branch")
            .expect("Invalid branch")
            .clone();
        let result = fetch_version(root, branch).await;
        match result {
            Ok(version) => {
                println!("Latest version: {:?}", version);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
