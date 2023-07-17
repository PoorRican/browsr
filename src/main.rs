mod tree;
mod cli;
mod parsing;
mod runtime;
mod ui;
mod mode;

use crate::{cli::parse_args, ui::bootstrap_terminal};

use std::process::exit;

fn main() {
    match parse_args() {
        Ok(mode) => {
            bootstrap_terminal(mode).unwrap()
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}

