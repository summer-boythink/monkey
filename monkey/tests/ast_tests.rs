#[cfg(test)]
mod tests {
    use monkey::{Expression, Identifier, LetStatement, Node, Program, Token, TokenType};

    #[test]
    fn test_macros() {
        let i = &Identifier {
            token: Token::new(TokenType::LET, "let".to_string()),
            value: "test_val".to_string(),
        };
        i.expression_node();
        assert_eq!("let".to_string(), i.token_literal());
    }

    #[test]
    fn test_let_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    r#type: TokenType::LET,
                    literal: "let".to_string(),
                },
                name: Some(Identifier {
                    token: Token {
                        r#type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                }),
                value: Some(Box::new(Identifier {
                    token: Token {
                        r#type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };
        assert_eq!(
            format!("{}", program.to_string()),
            "let myVar = anotherVar;"
        )
    }
}
