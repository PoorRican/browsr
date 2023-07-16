use tui::{backend::Backend, Terminal, };

use crate::
    mode::{Mode, ModeReturns};

use std::io;

pub struct Runtime<'a, B: Backend> {
    terminal: &'a mut Terminal<B>,
    mode: Box<dyn Mode<B>>
}

impl<'a, B: Backend> Runtime<'a, B> {
    pub fn new(terminal: &'a mut Terminal<B>, mode: Box<dyn Mode<B>>) -> Self {
        Self {
            mode,
            terminal
        }
    }

    pub fn run_ui(&mut self) -> io::Result<Option<ModeReturns>> {
        loop {
            self.mode.render(&mut self.terminal)?;
            let mode_return = self.mode.handle_input()?;
            if let Some(inner) = mode_return {
                // catch mode changes here
                match inner {
                    ModeReturns::Quit => return Ok(Some(inner)),
                }
            }
        }
    }

}
