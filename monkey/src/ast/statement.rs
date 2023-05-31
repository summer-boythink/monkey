use crate::{Expression, Identifier, Token};

pub struct LetStatement {
    token: Token,
    name: *mut Identifier,
    value: dyn Expression,
}
