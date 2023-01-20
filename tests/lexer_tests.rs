#[cfg(test)]
mod tests {
    use monkey::{Lexer, Token, TokenType};

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;".to_string();
        let hope_test_val = vec![
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for item in hope_test_val {
            let tok = lexer.next_token();
            assert_eq!(tok.r#type, item.r#type);
            assert_eq!(tok.literal, item.literal);
        }
    }

    #[test]
    fn test_next_token() {
        let input = "let five = 5;\
        let ten = 10;\
        fn(x,y) {\
        x+y;\
        };"
        .to_string();
        let hope_test_val = vec![
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "five".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "x".to_string(),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "y".to_string(),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "x".to_string(),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "y".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for item in hope_test_val {
            let tok = lexer.next_token();
            assert_eq!(tok.r#type, item.r#type);
            assert_eq!(tok.literal, item.literal);
        }
    }
}
