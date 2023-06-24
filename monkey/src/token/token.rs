use crate::token::keywords::KeyWord;
use crate::TokenType::IDENT;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NotEq,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            r#type: token_type,
            literal,
        }
    }

    pub fn look_up_ident(ident: String) -> TokenType {
        for v in KeyWord::new().key_word {
            if v.0 == ident {
                return v.1;
            }
        }
        IDENT
    }
}
