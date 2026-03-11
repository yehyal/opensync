pub mod sync;

// use clap::{ArgMatches, Command};

// pub fn configure(command: Command) -> Command {
//     command
//         .subcommand(sync::configure())
//         .arg_required_else_help(true)
// }

// pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
//     if let Some((cmd, matches)) = matches.subcommand() {
//         match cmd {
//             sync::COMMAND_NAME => sync::handle(matches)?,
//             &_ => {}
//         }
//     }

//     Ok(())
// }
