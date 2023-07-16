mod tree;
mod cli;
mod parsing;
mod runtime;
mod ui;

use crate::{
    cli::parse_args,
    parsing::*,
    ui::bootstrap_terminal
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, env, process::exit};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_tree_widget::Tree;

use ncbi::eutils::{
    parse_xml, get_local_xml, DataType
};

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

