use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

pub(crate) enum Token {
    String(String)
}

pub(crate) type TokenMatcher = Box<dyn Fn(String) -> bool>;
pub(crate) type Tokenizer = Box<dyn Fn(String) -> Token>;
pub(crate) type TokenizerAttempt = Box<dyn Fn(String) -> Option<Token>>;

pub(crate) struct MatchingTokenizer {
    name: String,
    token_matcher: TokenMatcher,
    tokenizer: Tokenizer,
}

impl MatchingTokenizer {}

impl PartialEq for MatchingTokenizer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for MatchingTokenizer {}
impl Hash for MatchingTokenizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub(crate) struct TryingTokenizer {
    name: String,
    tokenizer: TokenizerAttempt,
}

impl PartialEq for TryingTokenizer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for TryingTokenizer {}
impl Hash for TryingTokenizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
