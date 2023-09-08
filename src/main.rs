pub mod lexer;
pub mod parser;

use std::io::{self, Write};

use crate::parser::Parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim() {
            line if line == ":q" || line == "quit" || line == "exit" => {
                break;
            },
            _ => {
                let mut parser = Parser::new(&line.as_str());
                let expr = parser.parse();

                if !parser.diagnostics.is_empty() {
                    for diagnostic in parser.diagnostics {
                        println!("{}", diagnostic);
                    }
                } else {
                    expr.evaluate()
                }
            },
        }
    }
}
