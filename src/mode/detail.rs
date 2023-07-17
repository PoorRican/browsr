use crossterm::event::{self, Event, KeyCode};
use std::{io, sync::Arc};
use tui_tree_widget::Tree;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders}
};
use ncbi::{parse_xml, get_local_xml, DataType};

use super::{Mode, ModeReturns};
use crate::{tree::StatefulTree, parsing::{group_lines, split_strings, format_strings}, ui::TerminalAlias};

pub struct DetailMode<'a> {
    pub filename: String,
    pub tree: StatefulTree<'a>,
}

impl<'a> DetailMode<'a> {
    pub fn new(filename: &String) -> Self {
        Self {
            tree: Self::build_tree(filename.as_str()),
            filename: filename.to_owned(),
        }
    }

    fn build_tree(filename: &str) -> StatefulTree<'a> {
        let xml = Arc::new(get_local_xml(filename));
        let parsed = parse_xml(xml.as_str()).unwrap();
        let pretty_string = match parsed {
            DataType::BioSeqSet(seq) => format!("{:?}", seq),
            _ => unimplemented!()
        };

        let formatted = format_strings(pretty_string);
        let lines = split_strings(formatted);
        let tree = group_lines(None, &mut lines.iter());
        StatefulTree::with_items(vec![tree])
    }
}

impl Mode for DetailMode<'_> {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()> {
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
