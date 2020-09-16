use crate::oldlexer::Token;
use crate::types::Literal;
use std::collections::VecDeque;

// TODO: add a definition node
// TODO: after definition works, add a quotation node

pub(crate) enum Node {
    Lit(Literal),
    Word(String),
}

pub(crate) struct Parser {
    tokens: VecDeque<Token>,
    ast: Vec<Node>,
}

impl Parser {
    pub fn new<T>(tokens: T) -> Self
    where
        T: Into<VecDeque<Token>>,
    {
        Parser {
            tokens: tokens.into(),
            ast: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Node> {
        while !self.tokens.is_empty() {
            match self.tokens.pop_front().unwrap() {
                Token::Lit(l) => self.ast.push(Node::Lit(l)),
                Token::Op(w) => self.ast.push(Node::Word(w)),
            }
        }
        self.ast
    }
}
