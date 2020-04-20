use crate::lexer::Tokens;
use crate::types::Literal;
use std::collections::VecDeque;

pub(crate) enum Node {
    Lit(Literal),
    Word(String),
}

pub(crate) struct Parser {
    tokens: VecDeque<Tokens>,
    ast: Vec<Node>,
}

impl Parser {
    pub fn new<T>(tokens: T) -> Self
    where
        T: Into<VecDeque<Tokens>>,
    {
        Parser {
            tokens: tokens.into(),
            ast: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Node> {
        while !self.tokens.is_empty() {
            match self.tokens.pop_front().unwrap() {
                Tokens::Lit(l) => self.ast.push(Node::Lit(l)),
                Tokens::Op(w) => self.ast.push(Node::Word(w)),
            }
        }
        self.ast
    }
}
