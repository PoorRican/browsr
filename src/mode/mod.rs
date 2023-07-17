//! Handle UI modes

mod detail;
mod menu;

pub use menu::MenuMode;
pub use detail::DetailMode;

use std::io;

use crate::ui::TerminalAlias;

pub enum ModeReturns {
    Quit,
    GoToDetails
}

pub trait Mode {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>>;
}
