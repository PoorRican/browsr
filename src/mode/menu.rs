use crossterm::event::{self, Event, KeyCode};
use tui_tree_widget::{Tree, TreeItem};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear},
};
use std::io;

use crate::{tree::StatefulTree, ui::TerminalAlias};

use super::{Mode, ModeReturns};

pub struct MenuMode<'a> {
    tree: StatefulTree<'a>
}

impl<'a> MenuMode<'a> {
    pub fn new() -> Self {
        let tree = StatefulTree::with_items(vec![
            TreeItem::new_leaf("Open File"),
            TreeItem::new_leaf("Exit")
        ]);
        Self { tree }
    }

    pub fn execute_option(&self) -> Option<ModeReturns> {
        let selected = self.tree.state.selected();
        if let Some(selected) = selected.first() {
            match selected {
                0usize => todo!(),
                1usize => Some(ModeReturns::Quit),
                _ => Some(ModeReturns::Quit),
            }
        } else {
            None
        }

    }
}



impl<'a> Mode for MenuMode<'a> {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();

            let items = Tree::new(self.tree.items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("browsr - Main menu"),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area); //this clears out the background
            f.render_stateful_widget(items, area, &mut self.tree.state);
        })?;
        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(ModeReturns::Quit)),
                KeyCode::Enter => return Ok(self.execute_option().into()),
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


/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
