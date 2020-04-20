use crate::types::Literal;
use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;
use std::fmt::Debug;

// TODO: add more literal types

#[derive(Debug)]
pub(crate) enum Token {
    Lit(Literal),
    Op(String),
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        if let Ok(num) = str::parse::<i64>(&s) {
            Self::Lit(Literal::Int(num))
        } else if let Ok(num) = str::parse::<f64>(&s){
            Self::Lit(Literal::Float(num))
        } else {
            Self::Op(s)
        }
    }
}

fn string_literal(start: String, chars: &mut Peekable<Chars>) -> Token {
    let mut was_escaped = false;
    let mut end = false;
    let rest: String = chars.peeking_take_while(|c|{
        if end {
            return false;
        }
        if !was_escaped && *c == '"' {
            end = true;
        }
        if *c == '\\' {
            was_escaped = true;
        } else {
            was_escaped = false;
        }
        true
    }).collect();
    Token::Lit(Literal::String(start + &rest))
}

pub(crate) struct Lexer {}

impl Lexer {
    pub fn lex(code: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = code.chars().into_iter().peekable();
        while chars.peek().is_some() {
            // skip front whitespace while preserving peekability
            let _ = chars.peeking_take_while(|c| c.is_whitespace()).collect::<String>();
            let token_string = chars.peeking_take_while(|c| !c.is_whitespace()).collect::<String>();
            if token_string.starts_with('"') {
                tokens.push(string_literal(token_string, &mut chars));
            } else {
                tokens.push(token_string.into());
            }
            // skip back whitespace while preserving peekability
            let _ = chars.peeking_take_while(|c| c.is_whitespace()).collect::<String>();
        }
        tokens
    }
}