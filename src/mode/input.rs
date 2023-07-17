use super::{Mode, ModeReturns};
use crossterm::
    event::{self, Event, KeyCode};
use crate::ui::TerminalAlias;
use std::{io, ops::Deref};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Spans, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

enum EditMode {
    Normal,
    Editing
}

pub struct InputMode {
    input: Input,
    edit_mode: EditMode,
    next: Box<ModeReturns>
}

impl InputMode {
    pub fn new(next: Box<ModeReturns>) -> Self {
        Self {
            input: Input::default(),
            edit_mode: EditMode::Normal,
            next
        }
    }
}

impl Mode for InputMode {
    fn render(&mut self, terminal: &mut TerminalAlias) -> io::Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
    
            let (msg, style) = match self.edit_mode {
                EditMode::Normal => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing."),
                    ],
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                EditMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to record the message"),
                    ],
                    Style::default(),
                ),
            };
            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);
    
            let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    
            let scroll = self.input.visual_scroll(width as usize);
            let input = Paragraph::new(self.input.value())
                .style(match self.edit_mode {
                    EditMode::Normal => Style::default(),
                    EditMode::Editing => Style::default().fg(Color::Yellow),
                })
                .scroll((0, scroll as u16))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);
            match self.edit_mode {
                EditMode::Normal =>
                    // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {}
    
                EditMode::Editing => {
                    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[1].x
                            + ((self.input.visual_cursor()).max(scroll) - scroll) as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        chunks[1].y + 1,
                    )
                }
            }
        })?;
    
        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<Option<ModeReturns>> {
        if let Event::Key(key) = event::read()? {
            match self.edit_mode {
                EditMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        self.edit_mode = EditMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(Some(ModeReturns::Quit));
                    }
                    _ => {}
                },
                EditMode::Editing => match key.code {
                    KeyCode::Enter => {
                        match self.next.deref() {
                            ModeReturns::GoToDetails(_) => return Ok(
                                ModeReturns::GoToDetails(
                                    self.input.value()
                                              .to_string()
                                              .into())
                                .into()),
                                _ => todo!()
                        }
                    }
                    KeyCode::Esc => {
                        self.edit_mode = EditMode::Normal;
                    }
                    _ => {
                        self.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
        Ok(None)
    }
}
