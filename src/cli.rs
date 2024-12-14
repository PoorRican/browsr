use std::error::Error;
use clap::{arg, Command};

use crate::mode::{Mode, DetailMode, MenuMode};

fn cli() -> Command {
    Command::new("browsr")
        .about("Browse biological annotation data")
        .subcommand(
            Command::new("view")
                .about("Directly view annotation data")
                .arg(arg!(<FILENAME> "the file to parse and view"))
        )
}


pub fn parse_args() -> Result<Box<dyn Mode>, Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("view", sub_matches)) => {
            let filename = sub_matches.get_one::<String>("FILENAME").expect("required");
            Ok(Box::new(DetailMode::new(filename)))
        }
        _ => Ok(Box::new(MenuMode::new()))
    }
}
