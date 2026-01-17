use std::path::Path;

use clap::ArgMatches;

mod plugin;
pub mod sdk;

pub trait CommandHandler {
    fn execute(
        &self,
        root: &Path,
        args: &ArgMatches,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
