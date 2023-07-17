use crate::{
    runtime::Runtime,
    mode::Mode,
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

pub type TerminalAlias = Terminal<CrosstermBackend<io::Stdout>>;

fn init_terminal() -> io::Result<TerminalAlias> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
}

fn restore_terminal(mut terminal: TerminalAlias) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}


pub fn bootstrap_terminal(mode: Box<dyn Mode>) -> Result<(), Box<dyn Error>> {
    let mut terminal = init_terminal()?;

    let mut runtime = Runtime::new(&mut terminal, mode);

    runtime.run_ui()?;

    restore_terminal(terminal)
}

