#[cfg(test)]
mod tests {
    use monkey::{Identifier, Token, TokenType};

    #[test]
    fn test_macros() {
        let i = &Identifier {
            token: Token::new(TokenType::LET, "let".to_string()),
            value: "test_val".to_string(),
        };
        i.expression_node();
        assert_eq!("let".to_string(), i.token_literal());
    }
}