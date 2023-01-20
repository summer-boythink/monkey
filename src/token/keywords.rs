use crate::TokenType;
use std::collections::HashMap;
use std::string::ToString;

// pub const KeyWord: HashMap<&str, TokenType> = HashMap::from([
//     ("fn", TokenType::FUNCTION),
//     ("let", TokenType::LET),
// ]);

//TODO: how to make a const Map?
pub struct KeyWord {
    pub key_word: HashMap<String, TokenType>,
}

impl KeyWord {
    pub fn new() -> KeyWord {
        KeyWord {
            key_word: HashMap::from([
                ("fn".to_string(), TokenType::FUNCTION),
                ("let".to_string(), TokenType::LET),
                ("true".to_string(), TokenType::TRUE),
                ("false".to_string(), TokenType::FALSE),
                ("if".to_string(), TokenType::IF),
                ("else".to_string(), TokenType::ELSE),
                ("return".to_string(), TokenType::RETURN),
            ]),
        }
    }
}
