#![allow(unused)]

use std::iter::Peekable;
use logos::{Logos, Span};

use super::Node;
use super::NodeHeader;

use super::intern::Interner;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
enum NodeToken {
    #[token("---")]
    HeaderEnd,
    #[token("===")]
    NodeEnd,

    #[regex(r"\r?\n")]
    LineEnd,

    #[regex(r"(    )|[\t]", priority = 2)]
    Indent,

    #[regex(r"\w+:")]
    Label,

    #[token("<<")]
    CmdStart,

    #[token(">>")]
    CmdEnd,

    #[token("{")]
    ExprStart,

    #[token("}")]
    ExprEnd,

    #[token("[")]
    OptionStart,

    #[token("]")]
    OptionEnd,

    #[token("->")]
    ShortcutOption,

    #[token("#")]
    Tag,


    #[error]
    #[regex("[ ]+", logos::skip)]
    Unknown,

    Eof,
}

impl Default for NodeToken {
    fn default() -> Self {
        NodeToken::Eof
    }
}

#[derive(Debug)]
pub struct ParseError(pub u32, pub &'static str);

struct Lex<'src, I> 
    where I: Iterator<Item=(NodeToken, Span)>
{
    iter: Peekable<I>,
    line: u32,
    col: u32,
    text: &'src str,

    intern: &'src mut Interner,
}


pub fn node(input: &str, intern: &mut Interner) -> Result<(Node, usize), ParseError> {
    let iter = NodeToken::lexer(input)
        .spanned()
        //.filter(|(tok, _)| { *tok != NodeToken::Unknown})
        .peekable();

    let mut lex = Lex {
        iter,
        line: 0, col: 0,
        text: input,
        intern,
    };
 
    let (header, s) = header(&mut lex)?;


    Ok((Node {
        header,
        lines: vec![],
    }, s.end))
}

macro_rules! pfn {
    ($name:ident($lex:ident $intern:ident) -> $ty:ty {$($t:tt)*}) => {
        fn $name(
            $lex: &mut impl Iterator<Item = (NodeToken, Span)>, 
            $intern: &mut Interner
        ) -> Result<($ty, Span), ParseError> {
            $($t)*
        }
    };
}

fn header<I: Iterator<Item = (NodeToken, Span)>>(
    lex: &mut Lex<I>,
) -> Result<(NodeHeader, Span), ParseError> {

    loop {
        let (tok, x) = next_vis(&mut lex.iter, &mut lex.intern)?;
        lex.col = x.start as u32;

        let header_item = match tok {
            NodeToken::Label => {
                let (_, end_pos) = next_of_ty(&mut lex.iter, NodeToken::LineEnd)?;


            }
            _ => {
                return Err(ParseError(x.end as u32, "all header items have to be in key:value form"));
            }
        };
    }
}

pfn!(next_vis(lex intern) -> NodeToken {
    loop {
        if let Some((tok, s)) = lex.next() {
            match tok {
                NodeToken::LineEnd | NodeToken::Indent => {},
                tok => return Ok((tok,s))
            }
        } else {
            return Err(ParseError(0, "unexpected eof"));
        }
    }
});

fn next_of_ty(
    lex: &mut impl Iterator<Item = (NodeToken, Span)>,
    tok: NodeToken,
) -> Result<(NodeToken, Span), ParseError> {
    lex.skip_while(|(t, _)| *t != tok).next().ok_or_else(||
        ParseError(0, "expected token not found")
    )
}