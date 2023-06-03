use crate::{Expression, Identifier, Token};

pub struct LetStatement {
    pub token: Token,
    pub name: *mut Identifier,
    pub value: dyn Expression,
}
