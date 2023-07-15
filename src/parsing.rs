//! Utilities for parsing and formatting output

use tui_tree_widget::TreeItem;
use std::slice::Iter;

pub fn format_strings(string: String) -> String {
    string.replace(',', "\n")
          .replace('{', "{\n").replace('}', "\n}")
          .replace('[', "[\n").replace(']', "\n]")
          .replace('"', " ")
          .replace('\'', " ")
          .replace("Str(", " ")
}

pub fn split_strings<'a>(formatted: &'a String) -> Vec<&'a str> {
    formatted.split("\n").map(|s| s.trim()).collect()
}

pub fn group_lines<'a>(start: Option<&&'a str>, lines: &mut Iter<&'a str>) -> TreeItem<'a> {
    let mut item = Vec::new();
    let mut start = start;
    loop {
        if start.is_none() {
            start = lines
                .next()
        }
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
    TreeItem::new(*start.unwrap(), item)
}

