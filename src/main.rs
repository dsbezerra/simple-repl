pub mod lexer;

use std::io::{self, Write};

use crate::lexer::{Lexer, TokenKind};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(&line.as_str());

        loop {
            let token = lexer.next_tok();
            match token.kind {
                TokenKind::EndOfFile | TokenKind::BadToken => {
                    println!("Invalid token: {:?}", token.kind);
                    break;
                }
                _ => println!("Got token kind {:?}", token.kind),
            }
        }
    }
}
