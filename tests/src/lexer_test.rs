use lexer::Lexer;
use core::{Token,TokenType};
#[macro_export]
macro_rules! assert_token {
    ($( { $token_type:path , $literal:tt } ),+) => {
      {
        $(
          let expected=Token::new($token_type, String::from($literal));
          let got=Lexer::new($literal.to_string()).read_token();
          assert_eq!(expected,got,"\nExpected: {:?}\nGot:{:?}\n",expected,got)
          )+
      }
    }
}
#[macro_export]
macro_rules! assert_token_sequence {

    ($code:literal,$( { $token_type:path , $literal:tt } ),+) => {
      {
        let mut lexer = Lexer::new($code.to_string());
        $(
          let expected=Token::new($token_type, String::from($literal));
          let got=lexer.read_token();
          assert_eq!(
            expected,got,"\nExpected: {:?}\nGot: {:?}\n",expected,got
          );
        )+
        let expected=Token::new(TokenType::EOF, String::from(""));
          let got=lexer.read_token();
        assert_eq!(
            expected,got,"\nExpected: {:?}\nGot: {:?}\n",expected,got
        );
      }
    }
}
#[test]
fn let_statement(){
    assert_token_sequence!(
        "let x=10;",
        {TokenType::LET,"let"},
        {TokenType::IDENTIFIER,"x"},
        {TokenType::ASSIGN,"="},
        {TokenType::INT,"10"},
        {TokenType::SEMICOLON,";"}
    )
}
#[test]
fn let_function(){
    assert_token_sequence!(
        "let function=fn(){
            let testing=10*3+5/4;
        };",
        {TokenType::LET,"let"},
        {TokenType::IDENTIFIER,"function"},
        {TokenType::ASSIGN,"="},
        {TokenType::FUNCTION,"fn"},
        {TokenType::LEFT_PARANTHESIS,"("},
        {TokenType::RIGHT_PARANTHESIS,")"},
        {TokenType::LEFT_CURL,"{"},
        {TokenType::LET,"let"},
        {TokenType::IDENTIFIER,"testing"},
        {TokenType::ASSIGN,"="},
        {TokenType::INT,"10"},
        {TokenType::ASTERISK,"*"},
        {TokenType::INT,"3"},
        {TokenType::ADD,"+"},
        {TokenType::INT,"5"},
        {TokenType::SLASH,"/"},
        {TokenType::INT,"4"},
        {TokenType::SEMICOLON,";"},
        {TokenType::RIGHT_CURL,"}"},
        {TokenType::SEMICOLON,";"}
    )
}