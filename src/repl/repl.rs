use crate::{Lexer, TokenType};
use colored::Colorize;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() {
    loop {
        print!("{} ", PROMPT.italic().green());
        io::stdout().flush().unwrap();
        let mut codes = String::new();
        io::stdin()
            .read_line(&mut codes)
            .expect("invalid code line");
        let mut lexer = Lexer::new(codes);
        loop {
            let tok = lexer.next_token();
            if tok.r#type == TokenType::EOF {
                break;
            }
            println!("{:?}", tok);
        }
    }
}
