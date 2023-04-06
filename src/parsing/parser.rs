use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::fmt;

use crate::parsing::tokens::*;

/* impl<'parser> fmt::Display for Token<'parser> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type: {} | Literal: {} ",
            self.token_type, self.token_literal
        )
    }
}*/

/* TODO and NOTE we are trying to switch over to a hashmap, but we will need a method to go thorugh
 * each script */
/*pub struct Parser<'parser> {
    old_tokens: Vec<Token<'parser>>,
    curent_script: &'parser String,
    current_index: usize,
    tokens: HashMap<&'parser String, Vec<Token<'parser>>>,
}*/

/*
impl<'parser> fmt::Display for Parser<'parser> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tokens.iter().fold(Ok(()), |result, token| {
            result.and_then(|_| writeln!(f, "{}", token))
        })
    }
}*/

/*impl<'parser> Parser<'parser> {
pub fn new(input: &'parser str) -> Result<Parser<'parser>, &'parser str> {
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
    Ok(Parser {
        tokens,
        current_index: 0,
        script_tokens: HashMap::new(),
    })
} */
/*

    pub fn new(&mut self, scripts: IterMut<'parser, String, String>) -> Result<Parser<'parser>, &'parser str> {
        let mut map = HashMap::new();
        scripts.for_each(|v| {
        let tokens: Vec<Token> = v.1
            .split_whitespace()
            .map(|s| match s {
                "npm" => new_token(NPM, s),
                "run" => new_token(RUN, s),
                "&&" => new_token(AND, s),
                _ => new_token(OP, s),
            })
            .collect::<Vec<Token>>();
            map.insert(v.0, tokens);
        });
        Ok(Self {
            tokens: Vec::new(),
            current_index: 0,
            script_tokens: map,
        })
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
    */

pub enum Grammar {
    And(String),
    Npm(String),
    Run(String),
    Op(String),
}

fn lex(input: &str) -> Result<Vec<Grammar>, String> {
    let tokens = input
        .split_whitespace()
        .map(|s| match s {
            "npm" => Grammar::Npm(s.to_string()),
            "run" => Grammar::Run(s.to_string()),
            "&&" => Grammar::And(s.to_string()),
            _ => Grammar::Op(s.to_string()),
        })
        .collect::<Vec<_>>();
    Ok(tokens)
}

pub struct ParserNode {
    pub tokens: Vec<Grammar>,
    pub location: String,
    current_index: usize,
}

pub struct Parser {
    current_index: usize,
    nodes: Vec<ParserNode>,
}

pub fn parse(input: Iter<String, String>) -> Result<Parser, String> {
    let mut parser_nodes: Vec<ParserNode> = Vec::new();
    for t in input {
        let tokens = lex(t.1)?;
        parser_nodes.push(ParserNode::new(tokens, t.0.to_string()));
    }
    Ok(Parser {
        current_index: 0,
        nodes: parser_nodes,
    })
}

impl ParserNode {
    pub fn new(tokens: Vec<Grammar>, location: String) -> ParserNode {
        Self {
            current_index: 0,
            tokens,
            location,
        }
    }

    fn get_current_index(&mut self) -> usize {
        self.current_index
    }

    fn increment(&mut self) {
        self.current_index += 1
    }

    /// check to see if the next index is valid for the node
    fn is_peek_valid(&self) -> bool {
        if self.current_index + 1 >= self.tokens.len() {
            return true
        }
        return false
    }
}

impl Parser {
    // TODO we need to advance the current_index based on what is null
    /// Peek at the next token, but do not advance the ptr
    pub fn peek_next(&self) -> Token {
        match self.nodes.get(self.current_index) {
            Some(n) => {
                match n.tokens.get(n.get_current_index() + 1) {
                    Some(t) => translate_grammer_to_token(t),
                    None => Token {
                        token_type: END,
                        token_literal: "END",
                    },
                }
            },
            None => Token {
                token_type: END,
                token_literal: "END",
            },
        }
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

fn translate_grammer_to_token(grammar: &Grammar) -> Token {
        match grammar {
            Grammar::And(s) => new_token(AND, s),
            Grammar::Npm(s) => new_token(AND, s),
            Grammar::Run(s) => new_token(AND, s),
            Grammar::Op(s) => new_token(OP, s),
        }
}
