use crate::{
    runtime::Runtime,
    parsing::{format_strings, split_strings},
    mode::DetailMode
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, error::Error};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use ncbi::{parse_xml, get_local_xml, DataType};

pub fn bootstrap_terminal(file: String) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Runtime
    let xml = get_local_xml(file.as_str());
    let parsed = parse_xml(xml.as_str()).unwrap();
    let pretty_string = match parsed {
        DataType::BioSeqSet(seq) => format!("{:?}", seq),
        _ => unimplemented!()
    };

    let formatted = format_strings(pretty_string);
    let lines = split_strings(formatted);
    let mode = Box::new(DetailMode::new(lines, file));
    let mut runtime = Runtime::new(&mut terminal, mode);

    let res = runtime.run_ui();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

