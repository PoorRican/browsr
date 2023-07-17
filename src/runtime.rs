use crate::ui::TerminalAlias;

use crate::
    mode::{Mode, ModeReturns, InputMode, DetailMode};

use std::io;

pub struct Runtime<'a> {
    terminal: &'a mut TerminalAlias,
    mode: Box<dyn Mode>
}

impl<'a> Runtime<'a> {
    pub fn new(terminal: &'a mut TerminalAlias, mode: Box<dyn Mode>) -> Self {
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
                    ModeReturns::GoToInput(next) => {
                        self.mode = Box::new(InputMode::new(next));
                    }
                    ModeReturns::GoToDetails(filename) => {
                        self.mode = Box::new(DetailMode::new(&filename.unwrap()));
                    }
                    _ => todo!()
                }
            }
        }
    }
}
