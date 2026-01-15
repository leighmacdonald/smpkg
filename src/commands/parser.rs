use clap::{ArgMatches, Command, arg};

pub fn commands_sourcemod_latest_version() -> Command {
    Command::new("latest-version")
        .short_flag('l')
        .about("Fetches the latest version of Sourcemod")
        .arg(arg!(<branch> "The remote branch"))
}

pub fn commands_sourcemod_install() -> Command {
    Command::new("install")
        .about("Download and install sourcemod")
        .arg(arg!(<branch> "The remote branch"))
}

pub fn commands_sourcemod() -> Command {
    Command::new("sourcemod")
        .about("Sourcemod distribution management commands")
        .alias("sm")
        .arg_required_else_help(true)
        .subcommand(commands_sourcemod_latest_version())
        .subcommand(commands_sourcemod_install())
}

pub fn commands() -> ArgMatches {
    return Command::new("smpkg")
        .about("A simple package manager")
        // .version(crate_version!)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands_sourcemod())
        .get_matches();
}
