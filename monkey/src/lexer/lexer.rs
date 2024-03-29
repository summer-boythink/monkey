use crate::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: Option<char>, //current char
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) -> &Lexer {
        if self.read_position >= self.input.len() as i32 {
            self.ch = None
        } else {
            //TODO:better index char
            let char_indices = self.input.char_indices();
            for c in char_indices {
                if c.0 == self.read_position as usize {
                    self.ch = Option::from(c.1);
                    break;
                }
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
        self
    }

    // peek forward char
    pub fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() as i32 {
            None
        } else {
            //TODO:better index char
            let char_indices = self.input.char_indices();
            let mut res = None;
            for c in char_indices {
                if c.0 == self.read_position as usize {
                    res = Some(c.1);
                }
            }
            res
        }
    }

    pub fn skip_white_space(&mut self) {
        if self.ch != None {
            let mut ch = self.ch.unwrap();
            while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
                if self.ch != None {
                    ch = self.ch.unwrap();
                } else {
                    break;
                }
            }
        }
    }

    pub fn is_letter(&self) -> bool {
        let ch = self.ch.unwrap();
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    pub fn is_digit(&self) -> bool {
        let ch = self.ch.unwrap();
        '0' <= ch && ch <= '9'
    }

    pub fn read_identifier(&mut self) -> String {
        let prev_position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        //TODO:better slice ?
        String::from(&self.input[prev_position as usize..(self.position) as usize])
    }

    pub fn read_number(&mut self) -> String {
        let prev_position = self.position;
        while self.is_digit() {
            self.read_char();
        }
        String::from(&self.input[prev_position as usize..(self.position) as usize])
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_white_space();
        match self.ch {
            Some('-') => tok = Token::new(TokenType::MINUS, self.ch.unwrap().to_string()),
            Some('<') => tok = Token::new(TokenType::LT, self.ch.unwrap().to_string()),
            Some('>') => tok = Token::new(TokenType::GT, self.ch.unwrap().to_string()),
            Some('*') => tok = Token::new(TokenType::ASTERISK, self.ch.unwrap().to_string()),
            Some('/') => tok = Token::new(TokenType::SLASH, self.ch.unwrap().to_string()),
            Some('!') => {
                if self.peek_char() == Some('=') {
                    let ch = self.ch.clone();
                    self.read_char();
                    let literal: String = format!("{}{}", ch.unwrap(), self.ch.unwrap());
                    tok = Token::new(TokenType::NotEq, literal);
                } else {
                    tok = Token::new(TokenType::BANG, self.ch.unwrap().to_string())
                }
            }
            Some('=') => {
                if self.peek_char() == Some('=') {
                    let ch = self.ch.clone();
                    self.read_char();
                    let literal: String = format!("{}{}", ch.unwrap(), self.ch.unwrap());
                    tok = Token::new(TokenType::EQ, literal);
                } else {
                    tok = Token::new(TokenType::ASSIGN, self.ch.unwrap().to_string())
                }
            }
            Some(';') => tok = Token::new(TokenType::SEMICOLON, self.ch.unwrap().to_string()),
            Some('(') => tok = Token::new(TokenType::LPAREN, self.ch.unwrap().to_string()),
            Some(')') => tok = Token::new(TokenType::RPAREN, self.ch.unwrap().to_string()),
            Some(',') => tok = Token::new(TokenType::COMMA, self.ch.unwrap().to_string()),
            Some('+') => tok = Token::new(TokenType::PLUS, self.ch.unwrap().to_string()),
            Some('{') => tok = Token::new(TokenType::LBRACE, self.ch.unwrap().to_string()),
            Some('}') => tok = Token::new(TokenType::RBRACE, self.ch.unwrap().to_string()),
            None => {
                tok = Token::new(TokenType::EOF, "".to_string());
            }
            _ => {
                if self.is_letter() {
                    let val = self.read_identifier();
                    tok = Token::new(Token::look_up_ident(val.clone()), val);
                    //TODO why i can't remove `return` ?
                    return tok;
                } else if self.is_digit() {
                    let val = self.read_number();
                    tok = Token::new(TokenType::INT, val);
                    return tok;
                } else {
                    tok = Token::new(TokenType::ILLEGAL, self.ch.unwrap().to_string());
                }
            }
        }
        self.read_char();
        tok
    }
}
