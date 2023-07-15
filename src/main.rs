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
use std::{error::Error, io, env, process::exit};
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
use serde::Serialize;
use serde_json::value::{
    to_value,
    Value
};

struct App<'a> {
    tree: StatefulTree<'a>,
    filename: String,
}

fn val_to_tree_item<'a>(val: Value) -> Option<TreeItem<'a>> {
    match val {
        Value::Null => None,
        Value::Bool(val) => {
            Some(
                TreeItem::new_leaf(
                    match val {
                        true => "true",
                        false => "false"
                    }
                )
            )
        }
        Value::Number(num) => Some(TreeItem::new_leaf(format!("{}", num))),
        Value::String(val) => Some(TreeItem::new_leaf(val.to_string())),
        Value::Array(val) => {
            let items: Vec<TreeItem> = 
                val.into_iter()
                   .map(|i| val_to_tree_item(i))
                   .filter(|i| i.is_some())
                   .map(|i| i.unwrap())
                   .collect();
            Some(TreeItem::new("array", items))
        }
        Value::Object(obj) => {
            let mut values = Vec::new();
            for (key, value) in obj.into_iter() {
                let item = val_to_tree_item(value);
                if let Some(item) = item {
                    values.push(TreeItem::new(key, vec![item]));
                }
            }
            Some(TreeItem::new("object", values))
        }
    }
}

fn parse_to_tree<'a, T: Serialize>(object: &T) -> Option<TreeItem<'a>> {
    if let Ok(json) = to_value(object) {
        return val_to_tree_item(json)
    }
    None
}

impl<'a> App<'a> {
    fn new(filename: String) -> Self {
        let file = get_local_xml(filename.as_str());
        let data = parse_xml(file.as_str());
        if let DataType::BioSeqSet(bioseq) = data.unwrap() {

            Self {
                filename,
                tree: StatefulTree::with_items(vec![parse_to_tree(&bioseq).unwrap()])
            }
        } else {
            unimplemented!()
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
    let app = App::new(file);
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
