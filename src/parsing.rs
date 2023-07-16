//! Utilities for parsing and formatting output
//!
//! This module allows for the use of the Rust debug string to display
//! complex objects as a string. This method allows any object that
//! implements the `Debug` trait to be displayed in `browsr`.
//!
//! [`format_strings()`] takes the original debug string, removes
//! unnecessary characters, and adds additional newline characters
//! before and after every comma, bracket or curly brace.
//!
//! [`split_strings()`] then separates this string with additional newlines
//! into a vector of strings. This vector of strings results in strings
//! which are very human readable and fan out vertically. [`group_lines()`]
//! consumes these lines to create a deep map of [`TreeItem`] elements to
//! result in the collapsible UI list.

use tui_tree_widget::TreeItem;
use std::slice::Iter;

/// Format Rust debug string for use in collapsible list
///
/// Newlines are added at every bracket and curly brace open/close, and at every
/// comma. This is to give a vertical appearance of the object. Then unwanted
/// characters are removed (ie: quotation marks, phrases like "Some(") by
/// replacing with a space.
pub fn format_strings(string: String) -> String {
    // add newlines
    string.replace(',', "\n")
          .replace('{', "{\n").replace('}', "\n}")
          .replace('[', "[\n").replace(']', "\n]")
          // remove undesirable characters or phrases
          .replace('"', " ")
          .replace('\'', " ")
          .replace("Str(", " ")
          .replace("Some(", " ")
}

/// Split strings based on newline
///
/// Since Rust debug string seems to use a few newlines,
/// [`format_strings()`] adds newlines before and after every bracket.
pub fn split_strings(formatted: String) -> Vec<String> {
    formatted.split("\n").map(|s| s.trim().to_string()).collect()
}

/// Serves the main purpose of creating a collapsible list from a vector of newlines.
///
/// Each string is accumulated as new [`TreeItem`] elements. Accumulated strings are returned
/// with the occurrence of each closing character (eg: `[`, `}`). Depth within the [`TreeItem`]
/// is created by recursing at every opening character (ie: '{', '[') and feeding the
/// line containing the opening character as the `start` start parameter of the subsequent
/// function call.
///
/// The first call to [`group_lines()`] should pass `None` as the first line encountered is used
/// as opening leaf text in the collapsible list.
///
/// # Parameters
///
/// - `start`: Text to use for the opening leaf. Any implementation should only pass
///            `None` as the opening text is implied from the first line encountered.
/// - `lines`: The accumulation of strings to build into a collapsible UI list element.
///
/// # Returns
///
/// A well formatted [`TreeItem`] to be used in a [`crate::tree::StatefulTree`]
pub fn group_lines<'a>( start: Option<&String>,
                        lines: &mut Iter<String> ) -> TreeItem<'a> {
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
                item.push(TreeItem::new_leaf(line.to_owned()));
            }
        } else {
            break;
        }
    }
    TreeItem::new(start.unwrap().clone(), item)
}

