use std::collections::HashMap;

pub mod intern;
mod parse;
pub use parse::node;

pub type NodeLabel = intern::InternedKey;

#[derive(Debug, Clone)]
pub struct Node {
    pub header: NodeHeader,
    pub lines: Vec<NodeLine>,
}

impl Node {
    pub fn title(&self) -> NodeLabel {
        self.header.title
    }
}

#[derive(Debug, Clone)]
pub struct NodeHeader {
    pub title: NodeLabel,
    pub tags: Vec<String>,
    pub custom: HashMap<NodeLabel, String>,
}

#[derive(Debug, Clone)]
pub struct NodeLine {
    pub label: Option<NodeLabel>,
    pub line: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(f32),
    String(NodeLabel),
}





