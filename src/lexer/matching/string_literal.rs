use crate::lexer::types::{MatchingTokenizer, Token, Tokenizer, TokenMatcher};

static mut STRING_TOKENIZER: MatchingTokenizer = MatchingTokenizer {
    name: "String".into(),
    token_matcher: Box::new(|s: String| s.starts_with('"')),
    tokenizer: Box::new(|s| Token::String(s)),
};