use crate::{Lexer, TokenType};
use colored::Colorize;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() {
    // This is primarily used for Windows 10 environments
    // which will not correctly colorize the outputs based on ANSI escape codes.
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();
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
