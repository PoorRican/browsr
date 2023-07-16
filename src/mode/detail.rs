use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui_tree_widget::Tree;
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use super::{Mode, ModeReturns};
use crate::{tree::StatefulTree, parsing::group_lines};

pub struct DetailMode<'a> {
    pub filename: String,
    pub tree: StatefulTree<'a>,
}

impl<'a> DetailMode<'a> {
    pub fn new(lines: Vec<String>, filename: String) -> Self {
        let tree = group_lines(None, &mut lines.iter());

        Self {
            filename,
            tree: StatefulTree::<'a>::with_items(vec![tree])
        }
    }
}

impl<B: Backend> Mode<B> for DetailMode<'_> {
    fn render(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let area = f.size();

            let items = Tree::new(self.tree.items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("browsr - {}", self.filename)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, area, &mut self.tree.state);
        })?;

        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(ModeReturns::Quit)),
                KeyCode::Char('\n' | ' ') => self.tree.toggle(),
                KeyCode::Left => self.tree.left(),
                KeyCode::Right => self.tree.right(),
                KeyCode::Down => self.tree.down(),
                KeyCode::Up => self.tree.up(),
                KeyCode::Home => self.tree.first(),
                KeyCode::End => self.tree.last(),
                _ => {}
            }
        }
        Ok(None)
    }

}
