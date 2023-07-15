use crate::{
    parsing::group_lines,
    tree::StatefulTree
};

pub struct Runtime<'a> {
    pub filename: String,
    pub tree: StatefulTree<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(lines: Vec<&'a str>, filename: String) -> Self {
        let tree = group_lines(None, &mut lines.iter());

        Self {
            filename,
            tree: StatefulTree::<'a>::with_items(vec![tree])
        }
    }
}

