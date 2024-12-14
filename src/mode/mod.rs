//! Handle UI modes

mod detail;
mod menu;
mod input;

pub use input::InputMode;
pub use menu::MenuMode;
pub use detail::DetailMode;

use std::io;

use crate::ui::TerminalAlias;

pub enum ModeReturns {
    Quit,
    GoToDetails(Option<String>),
    GoToInput(Box<ModeReturns>),
}

pub trait Mode {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>>;
}
