mod tree;
mod cli;

use crate::{
    cli::parse,
    tree::StatefulTree,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, env, process::exit, slice::Iter};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_tree_widget::{Tree, TreeItem};

use ncbi::eutils::{
    parse_xml, get_local_xml, DataType
};

struct App<'a> {
    filename: String,
    tree: StatefulTree<'a>,
}

fn group_lines<'a>(start: Option<&'a str>, lines: &mut Iter<&'a str>) -> TreeItem<'a> {
    let mut item = Vec::new();
    loop {
        let next = lines.next();
        if let Some(line) = next {
            if line.contains("{") || line.contains("[") {
                item.push(group_lines(Some(line), lines));
            } else if line.contains("}") || line.contains("]") {
                break;
            } else {
                item.push(TreeItem::new_leaf(*line));
            }
        } else {
            break;
        }
    }
    TreeItem::new(start.unwrap(), item)
}

impl<'a> App<'a> {
    fn new(lines: Vec<&'a str>, filename: String) -> Self {
        let tree = group_lines("bioseq-set".into(), &mut lines.iter());

        Self {
            filename,
            tree: StatefulTree::<'a>::with_items(vec![tree])
        }
    }
}

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

    // App
    let xml = get_local_xml(file.as_str());
    let parsed = parse_xml(xml.as_str()).unwrap();
    let pretty_string = match parsed {
        DataType::BioSeqSet(seq) => format!("{:?}", seq),
        _ => unimplemented!()
    };
    
    let formatted = format_strings(pretty_string);
    let lines = split_strings(&formatted);
    let app = App::new(lines, file);
    let res = run_app(&mut terminal, app);

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

fn format_strings(string: String) -> String {
    string.replace(',', "\n")
          .replace('{', "{\n").replace('}', "\n}")
          .replace('[', "[\n").replace(']', "\n]")
          .replace('"', " ")
          .replace('\'', " ")
          .replace("Str(", " ")
}
fn split_strings<'a>(formatted: &'a String) -> Vec<&'a str> {
    formatted.split("\n").map(|s| s.trim()).collect()
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.size();

            let items = Tree::new(app.tree.items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("browsr - {}", app.filename)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, area, &mut app.tree.state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('\n' | ' ') => app.tree.toggle(),
                KeyCode::Left => app.tree.left(),
                KeyCode::Right => app.tree.right(),
                KeyCode::Down => app.tree.down(),
                KeyCode::Up => app.tree.up(),
                KeyCode::Home => app.tree.first(),
                KeyCode::End => app.tree.last(),
                _ => {}
            }
        }
    }
}
