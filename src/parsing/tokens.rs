// TODO ENUMS?
pub const NPM: TokenType = "npm";
pub const RUN: TokenType = "run";
// An op is an operation of some sort
pub const OP: TokenType = "OP";
pub const AND: TokenType = "&&";
pub const END: TokenType = "END";
pub type TokenType<'token> = &'token str;

pub struct Token<'token> {
    pub token_type: TokenType<'token>,
    pub token_literal: &'token str,
}

pub fn new_token<'token>(token_type: TokenType<'token>, literal: &'token str) -> Token<'token> {
    Token {
        token_type,
        token_literal: literal,
    }
}

