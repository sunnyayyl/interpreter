use core::{Token, TokenType};
use parser::ast::{self, Expression, Identifier, LetStatement, Node, Statement};
#[test]
fn test_string() {
    let program = ast::Program {
        statements: vec![Statement::LetStatement(LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            name: Identifier {
                token: core::Token {
                    token_type: TokenType::IDENTIFIER,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Some(Expression::Identifier(Identifier {
                token: Token {
                    token_type: TokenType::IDENTIFIER,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            })),
        })],
    };
    assert_eq!(program.string(), "let myVar = anotherVar;")
}
