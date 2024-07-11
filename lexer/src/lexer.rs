use core::{Token,TokenType,NULL_CHAR,KEYWORD};

#[derive(Debug, Clone)]
pub struct Lexer {
    input_chars: Vec<char>,
    current_char: char,
    current_index: usize,
    next_index: usize,
    eof: bool,
}
impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            eof: false,
            input_chars: code.clone().chars().collect(),
            current_char: code.clone().chars().nth(0).unwrap_or_default(),
            current_index: 0,
            next_index: 1,
        }
    }
    fn read_char(&mut self) {
        if self.next_index >= self.input_chars.len() {
            self.current_char = NULL_CHAR;
            self.eof = true;
        } else {
            self.current_char = self.input_chars[self.next_index];
        }
        self.current_index = self.next_index;
        self.next_index += 1;
    }
    fn peek(&self) -> char {
        if self.next_index >= self.input_chars.len() {
            NULL_CHAR
        } else {
            self.input_chars[self.next_index]
        }
    }
    pub fn read_token(&mut self) -> Token {
        self.eat_whitespace();
        let token: Token = match self.current_char {
            '+' => Token::new(TokenType::ADD, String::from("+")),
            '-' => Token::new(TokenType::MINUS, String::from("-")),
            '*' => Token::new(TokenType::ASTERISK, String::from("*")),
            '/' => Token::new(TokenType::SLASH, String::from("/")),
            '<' => Token::new(TokenType::LT, String::from("<")),
            '>' => Token::new(TokenType::GT, String::from(">")),
            ';' => Token::new(TokenType::SEMICOLON, String::from(";")),
            ',' => Token::new(TokenType::COMMA, String::from(",")),
            '(' => Token::new(TokenType::LEFT_PARANTHESIS, String::from("(")),
            ')' => Token::new(TokenType::RIGHT_PARANTHESIS, String::from(")")),
            '{' => Token::new(TokenType::LEFT_CURL, String::from("{")),
            '}' => Token::new(TokenType::RIGHT_CURL, String::from("}")),
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::EQUAL, String::from("=="))
                } else {
                    Token::new(TokenType::ASSIGN, String::from("="))
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::NOTEQUAL, String::from("!="))
                } else {
                    Token::new(TokenType::EXCLAMATION, String::from("!"))
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let literal = self.read_long(TokenType::STRING);
                if KEYWORD.contains_key(&literal) {
                    return Token::new(KEYWORD[&literal], literal);
                } else {
                    return Token::new(TokenType::IDENTIFIER, literal);
                }
            }
            '0'..='9' => return Token::new(TokenType::INT, self.read_long(TokenType::INT)),
            _ => {
                if !self.eof {
                    let mut tmp = [0; 4];
                    Token::new(
                        TokenType::ILLEGAL,
                        self.current_char.encode_utf8(&mut tmp).to_string(),
                    )
                } else {
                    Token::new(TokenType::EOF, String::from(""))
                }
            }
        };
        self.read_char();
        token
    }
    fn read_long(&mut self, t: TokenType) -> String {
        let pos: usize = self.current_index;
        while (self.current_char.is_alphabetic() && t == TokenType::STRING)
            || (self.current_char.is_numeric() && t == TokenType::INT)
        {
            self.read_char();
        }
        self.input_chars[pos..self.current_index]
            .into_iter()
            .collect::<String>()
    }
    fn eat_whitespace(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            self.read_char()
        }
    }
}
impl<'a> IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = vec![];
        let mut new = self.clone();
        let mut t = new.read_token();
        while t.token_type != TokenType::EOF {
            v.push(t);
            t = new.read_token();
        }
        v.into_iter()
    }
}
