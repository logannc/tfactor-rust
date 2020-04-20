use crate::types::Literal;

pub(crate) enum Tokens {
    Lit(Literal),
    Op(String),
}

impl From<String> for Tokens {
    fn from(s: String) -> Self {
        if let Ok(num) = str::parse::<i64>(&s) {
            Self::Lit(Literal::Int(num))
        } else {
            Self::Op(s)
        }
    }
}
pub(crate) struct Lexer {}

impl Lexer {
    pub fn lex(code: String) -> Vec<Tokens> {
        code.split_whitespace().map(|s| s.to_owned().into()).collect()
    }
}