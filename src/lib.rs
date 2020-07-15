use std::collections::HashMap;
use thiserror::Error;

mod ast;
use ast::intern::Interner;
use ast::NodeLabel;

pub mod event;
use event::Event;

pub trait Context {
    fn get_var(&self, var: NodeLabel) -> Option<ast::Value>;
    fn set_var(&mut self, var: NodeLabel, val: ast::Value);
}

impl Context for HashMap<NodeLabel, ast::Value> {
    fn get_var(&self, var: NodeLabel) -> Option<ast::Value> { 
        self.get(&var).map(|v| v.clone())   
    }
    fn set_var(&mut self, var: NodeLabel, val: ast::Value) {
        self.insert(var, val);
    }
}

enum Wait {
    Input,
    Deadline( std::time::Instant),
    Nothing,
}

pub struct DialogRunner<C: Context = HashMap<NodeLabel, ast::Value>> {
    string_table: Interner,
    
    nodes: HashMap<NodeLabel, ast::Node>,
    functions: HashMap<NodeLabel, (usize, Box<dyn FnMut(&mut C, &[&str])>)>, 
    ctx: C,

    curr_node: Option<NodeLabel>,
    curr_line: usize,
    waiting: Wait,
}

impl DialogRunner {
    pub fn new() -> Self {
        Self::with_context(HashMap::new())
    }
}

impl<C: Context> DialogRunner<C> {
    pub fn with_context(ctx: C) -> Self {
        DialogRunner {
            string_table: Interner::with_capacity(128),
            nodes: HashMap::new(),
            functions: HashMap::new(),
            ctx,

            curr_node: None,
            curr_line: 0,
            waiting: Wait::Nothing,
        }
    }

    pub fn add_function<F>(&mut self, name: &str, args: usize, func: F) 
        where F: FnMut(&mut C, &[&str]) + 'static
    {
        let name = self.string_table.intern(name);
        self.functions.insert(name, (args, Box::new(func)));
    }

    pub fn load_yarn_file<'src>(&mut self, text: &'src str) -> Result<(), Error<'src>> {
        let mut start = 0;
        
        loop {
            let node = ast::node(&text[start..], &mut self.string_table);
            match node {
                Ok((node, pos)) => {
                    start += pos;
                    let title = node.title();
                    self.nodes.insert(title, node);
                    if self.curr_node.is_none() {
                        self.curr_node = Some(title);
                    }
                }
                Err(e) => {
                    if e.1 == "empty" {
                        break;
                    }

                    let line = text[start..].lines().nth(e.0 as usize);
                    return Err(Error::Syntax(e.0, line.unwrap(), e.1));
                }
            }

        }

        Ok(())
    }

    pub fn next_event(&mut self) -> Option<Event> {
        None
    }
}

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Syntax Error:\n\n{0} | {1}\n\t-- {2}")]
    Syntax(u32, &'a str, &'static str),
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
