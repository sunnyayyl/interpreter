use lazy_static::lazy_static;
use std::collections::HashMap;
pub const NULL_CHAR: char = '\0';

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    KEYWORD,
    IDENTIFIER,
    ADD,
    MINUS,
    EOF,
    STRING,
    INT,
    ASSIGN,
    EQUAL,
    NOTEQUAL,
    EXCLAMATION,
    FUNCTION,
    LET,
    IF,
    ELSE,
    LT,
    GT,
    SEMICOLON,
    COMMA,
    ASTERISK,
    SLASH,
    LEFT_PARANTHESIS,
    RIGHT_PARANTHESIS,
    LEFT_CURL,
    RIGHT_CURL,
    TRUE,
    FALSE,
    RETURN
}
lazy_static! {
    pub static ref KEYWORD: HashMap<String, TokenType> = {
        let keyword: HashMap<String, TokenType> = HashMap::from([
            (String::from("fn"), TokenType::FUNCTION),
            (String::from("let"), TokenType::LET),
            (String::from("else"), TokenType::ELSE),
            (String::from("true"), TokenType::TRUE),
            (String::from("false"), TokenType::FALSE),
            (String::from("if"), TokenType::IF),
            (String::from("else"), TokenType::ELSE),
            (String::from("return"), TokenType::RETURN)
        ]);
        keyword
    };
  #[derive(Debug)]
    pub static ref NULL_STR: &'static str = {
        let null_char = std::str::from_utf8(&[NULL_CHAR as u8]).unwrap();
        null_char
    };
}

#[derive(Debug, PartialEq,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
    pub fn default() -> Self {
        Self {
            token_type: TokenType::ILLEGAL,
            literal: NULL_STR.to_string(),
        }
    }
    pub fn token_literal(&self)->String{
        self.literal.clone()
    }
}