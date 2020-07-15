use std::collections::HashMap;

pub mod intern;
mod parse;
pub use parse::node;

pub type NodeLabel = intern::InternedKey;

#[derive(Debug, Clone)]
pub struct Node {
    header: NodeHeader,
    lines: Vec<NodeLine>,
}

impl Node {
    pub fn title(&self) -> NodeLabel {
        self.header.title
    }
}

#[derive(Debug, Clone)]
pub struct NodeHeader {
    title: NodeLabel,
    tags: Vec<String>,
    custom: HashMap<NodeLabel, String>,
}

#[derive(Debug, Clone)]
pub struct NodeLine {
    label: Option<NodeLabel>,
    line: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(f32),
    String(NodeLabel),
}





