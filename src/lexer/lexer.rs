use crate::{Token, TokenType};

// #[derive(Copy, Clone)]
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: Option<char>, //current char
}

// impl Clone for Lexer {
//
//     fn clone(&self) -> Self {
//         let s = *self;
//         Lexer {
//             input: s.input,
//             position:s.position,
//             read_position:s.read_position,
//             ch:s.ch
//         }
//     }
// }

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
                }
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
        self
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        match self.ch {
            Some('=') => tok = Token::new(TokenType::ASSIGN, self.ch.unwrap().to_string()),
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
                todo!()
            }
        }
        self.read_char();
        tok
    }
}
