use crate::ast;
use core::{Token, TokenType};
use lexer::Lexer;
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?}",
            t, self.peek_token.token_type
        ));
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.read_token();
    }
    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::new();

        while !self.current_token_is(TokenType::EOF) {
            let statement = self.parse_statement();
            if statement.is_some() {
                program.statements.push(statement.unwrap());
            }
            self.next_token();
        }
        if program.statements.len() == 0 {
            None
        } else {
            Some(program)
        }
    }
    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.token_type {
            TokenType::LET => ast::Statement::from_some_let_statement(self.parse_let_statement()),
            TokenType::RETURN => {
                ast::Statement::from_some_return_statement(self.parse_return_statement())
            }
            _ => None,
        }
    }
    fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement> {
        let return_token = self.current_token.clone();
        self.next_token();
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(ast::ReturnStatement {
            token: return_token,
            return_value: None,
        })
    }
    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let let_token = self.current_token.clone();
        if !self.expect_peek(TokenType::IDENTIFIER) {
            return None;
        }
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.token_literal().clone(),
        };
        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(ast::LetStatement {
            token: let_token,
            name: name.clone(),
            value: None,
        })
    }
    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
    pub fn check_parser_errors(&self) {
        if self.errors.len() == 0 {
            return;
        }
        for error in self.errors.iter() {
            println!("{}", error);
        }
        panic!("Parser has {} errors", self.errors.len())
    }
}
