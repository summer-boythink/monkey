use monkey_macros::DefaultExpressionNode;

use crate::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(DefaultExpressionNode)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn express_node(&self) {}

    pub fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}
