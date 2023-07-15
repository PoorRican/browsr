mod tree;
mod cli;
mod parsing;
mod runtime;

use crate::{
    cli::parse,
    parsing::*,
    runtime::Runtime
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, env, process::exit};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_tree_widget::Tree;

use ncbi::eutils::{
    parse_xml, get_local_xml, DataType
};

fn main() {
    let passed: Vec<String> = env::args().collect();
    match parse(&passed) {
        Ok(file) => {
            run(file).unwrap()
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn run(file: String) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Runtime
    let xml = get_local_xml(file.as_str());
    let parsed = parse_xml(xml.as_str()).unwrap();
    let pretty_string = match parsed {
        DataType::BioSeqSet(seq) => format!("{:?}", seq),
        _ => unimplemented!()
    };
    
    let formatted = format_strings(pretty_string);
    let lines = split_strings(&formatted);
    let runtime = Runtime::new(lines, file);
    let res = render(&mut terminal, runtime);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn render<B: Backend>(terminal: &mut Terminal<B>, mut runtime: Runtime) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.size();

            let items = Tree::new(runtime.tree.items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("browsr - {}", runtime.filename)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, area, &mut runtime.tree.state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('\n' | ' ') => runtime.tree.toggle(),
                KeyCode::Left => runtime.tree.left(),
                KeyCode::Right => runtime.tree.right(),
                KeyCode::Down => runtime.tree.down(),
                KeyCode::Up => runtime.tree.up(),
                KeyCode::Home => runtime.tree.first(),
                KeyCode::End => runtime.tree.last(),
                _ => {}
            }
        }
    }
}
