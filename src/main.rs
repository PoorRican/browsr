mod tree;
mod cli;
mod parsing;
mod runtime;
mod ui;
mod mode;

use crate::{cli::parse_args, ui::bootstrap_terminal};

use std::{env, process::exit};

fn main() {
    let passed: Vec<String> = env::args().collect();
    match parse_args(&passed) {
        Ok(file) => {
            bootstrap_terminal(file).unwrap()
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

