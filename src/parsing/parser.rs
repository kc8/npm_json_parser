use std::fmt;

use crate::parsing::tokens::*;

impl<'parser> fmt::Display for Token<'parser> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type: {} | Literal: {} ",
            self.token_type, self.token_literal
        )
    }
}

pub struct Parser<'parser> {
    tokens: Vec<Token<'parser>>,
    current_index: usize,
}

impl<'parser> fmt::Display for Parser<'parser> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tokens.iter().fold(Ok(()), |result, token| {
            result.and_then(|_| writeln!(f, "{}", token))
        })
    }
}

impl<'parser> Parser<'parser> {
    pub fn new(input: &'parser str) -> Parser<'parser> {
        let tokens: Vec<Token> = input
            .split_whitespace()
            .map(|s| match s {
                "npm" => new_token(NPM, s),
                "run" => new_token(RUN, s),
                "&&" => new_token(AND, s),
                _ => new_token(OP, s),
            })
            .collect::<Vec<_>>();
        // TODO change how we init the current_token and peek_token
        Parser {
            tokens,
            current_index: 0,
        }
    }

    /// Peek at the next token, but do not advance the ptr
    pub fn peek_next(&self) -> &Token {
        match self.tokens.get(self.current_index + 1) {
            None => &Token {
                token_type: END,
                token_literal: "END",
            },
            Some(t) => t,
        }
    }

    /// Get current token AND then advance the token ptr
    pub fn next_token(&self) -> &Token {
        let result = match self.tokens.get(self.current_index) {
            None => &Token {
                token_type: END,
                token_literal: "END",
            },
            Some(t) => t,
        };
        result
    }

    /// Get current token. Do not advance the token ptr
    pub fn current(&self) -> &Token {
        match self.tokens.get(self.current_index) {
            None => &Token {
                token_type: END,
                token_literal: "END",
            },
            Some(t) => t,
        }
    }

    /// Get the script opt. Do not advance the token
    /// Used when current = NPM and next = peek. This should be an op
    pub fn get_script_op(&self) -> &Token {
        match self.tokens.get(self.current_index + 2) {
            None => &Token {
                token_type: END,
                token_literal: "END",
            },
            Some(t) => t,
        }
    }

    /// Set the incremented ahead
    pub fn next(&mut self) {
        self.current_index += 1;
    }
}
