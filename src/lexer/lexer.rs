use std::collections::HashSet;
use crate::lexer::types::{MatchingTokenizer, Token, TryingTokenizer};
use linked_hash_set::LinkedHashSet;

struct Lexer {
    unordered: HashSet<MatchingTokenizer>,
    ordered: LinkedHashSet<TryingTokenizer>,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            unordered: HashSet::new(),
            ordered: LinkedHashSet::new(),
        }
    }

    fn add_matching_tokenizer(&mut self, tokenizer: MatchingTokenizer) -> &mut Self {
        self.unordered.insert(tokenizer);
        self
    }

    fn add_trying_tokenizer(&mut self, tokenizer: TryingTokenizer)  -> &mut Self {
        self.ordered.insert(tokenizer);
        self
    }
    
    fn lex(code: String) -> Vec<Token> {
        Vec::new()
    }
}