use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    ///Start of a new Node
    NewNode(NodeInfo),
    ///A line of dialog, with optional user
    Line(Option<String>, String),
    ///A list of options to choose
    Options(Vec<String>),
    ///The DialogRunner is paused waiting for input
    WaitInput,
    ///The DialogRunner is paused for 
    Wait(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeInfo {
    pub title: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}
