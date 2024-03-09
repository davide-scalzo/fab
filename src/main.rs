use std::path::Path;

use clap::{command, Arg, Command};
mod commands;
mod state;

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("up")
                .about("Starts one or more instances of the target executable")
                .arg(
                    Arg::new("filename")
                        .required(true)
                        .help("Path for the executable"),
                ),
        )
        .subcommand(
            Command::new("down")
                .about("stops an executable instance that was started with fab")
                .arg(
                    Arg::new("filename")
                        .required(true)
                        .help("Path for the executable"),
                ),
        )
        .subcommand(Command::new("status").about("lists intances currently running"))
        .get_matches();

    let state = state::State::new();

    match matches.subcommand() {
        Some(("up", sub_matches)) => {
            let filename = sub_matches.get_one::<String>("filename").unwrap();
            commands::up(Path::new(filename), state);
        }
        Some(("down", sub_matches)) => {
            let filename = sub_matches.get_one::<String>("filename").unwrap();
            commands::down(filename, state);
        }
        Some(("status", _)) => {
            commands::status(state);
        }
        _ => unreachable!("Invalid fab command"),
    }
}
