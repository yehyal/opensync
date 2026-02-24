use clap::{Arg, ArgMatches, Command, value_parser};

use crate::settings::Settings;

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP Server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    println!("Start server on port {}", port);
    Ok(())
}
