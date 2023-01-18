use std::ops::Index;

pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: Option<char>, //current char
}

impl Lexer {
    pub fn new(input: String) -> &Lexer {
        let mut lexer = &Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(mut self) {
        if self.read_position >= self.input.len() as i32 {
            self.ch = None
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
