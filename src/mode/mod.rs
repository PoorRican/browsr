//! Handle UI modes

mod detail;

pub use detail::DetailMode;

use tui::{Terminal, backend::Backend};
use std::io;

pub enum ModeReturns {
    Quit,
}

pub trait Mode<B: Backend> {
    fn render(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>>;
}
