//! Handle UI modes

mod detail;

pub use detail::DetailMode;

use std::io;

use crate::ui::TerminalAlias;

pub enum ModeReturns {
    Quit,
}

pub trait Mode {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>>;
}
