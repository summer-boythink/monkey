#[cfg(test)]
mod tests {
    use monkey::{Lexer, Token, TokenType};

    #[test]
    fn test_next_token() {
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

        let lexer = Lexer::new(input);

    }
}
