pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Integer(isize),
    Decimal(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    BadToken,
    EndOfFile,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

fn is_whitespace(c: &char) -> bool {
    matches!(c, '\n' | '\t' | '\x0C' | '\r' | ' ')
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { 
            input,
            position: 0,
         }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let index = self.position + offset;
        if index >= self.input.len() {
            return Some('\0');
        }

        self.input.chars().nth(index)
    }

    fn current(&mut self) -> char {
        self.peek(0).unwrap_or_default()
    }

    fn lex_number(&self) -> Token {
        let start = self.position;

        let mut len = 0;
        let mut seen_dot = false;
        let mut seen_minus = false;
        while let Some(c) = self.peek(len) {
            let should_continue = if c.is_digit(10) {
                true
            } else if c == '-' {
                if !seen_minus {
                    seen_minus = true;
                    true
                } else {
                    false
                }
            } else if c == '.' {
                if !seen_dot {
                    seen_dot = true;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if !should_continue {
                break;
            }

            len += c.len_utf8();
        }

        let end = start + len;

        let text = self.input[start..end].to_string();
        let kind = if seen_dot {
            TokenKind::Decimal(text.parse::<f64>().unwrap())
        } else {
            TokenKind::Integer(text.parse::<isize>().unwrap())
        };
        Token { kind, text }
    }

    fn eat_whitespaces(&mut self) {
        loop {
            let c = self.current();
            if is_whitespace(&c) {
                self.position += c.len_utf8();
            } else {
                break;
            }
        }
    }

    pub fn next_tok(&mut self) -> Token {
        self.eat_whitespaces();

        let char = self.current();
        let token = match char {
            '\0' => Token { kind: TokenKind::EndOfFile, text: '\0'.to_string() },
            '+' => Token { kind: TokenKind::Plus, text: "+".to_string() },
            '-' => {
                let next = self.peek(1);
                if next.is_some() && next.unwrap().is_digit(10) {
                    self.lex_number()
                } else {
                    Token { kind: TokenKind::Minus, text: "-".to_string() }
                }
            },
            '*' => Token { kind: TokenKind::Asterisk, text: "*".to_string() },
            '/' => Token { kind: TokenKind::Slash, text: "/".to_string() },
            char if char.is_digit(10) => self.lex_number(),
            _ => Token { kind: TokenKind::BadToken, text: String::from(char) },
        };

        self.position += token.text.len();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_whitespaces() {
        let input = "        +        /";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token { kind: TokenKind::Plus, text: "+".to_string() },
            Token { kind: TokenKind::Slash, text: "/".to_string() },
        ];

        for token in expected {
            assert_eq!(lexer.next_tok(), token);    
        }
    }

    #[test]
    fn next_tok() {
        let input = "+-*/ 12356";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token { kind: TokenKind::Plus, text: "+".to_string() },
            Token { kind: TokenKind::Minus, text: "-".to_string() },
            Token { kind: TokenKind::Asterisk, text: "*".to_string() },
            Token { kind: TokenKind::Slash, text: "/".to_string() },
        ];

        for token in expected {
            assert_eq!(lexer.next_tok(), token);    
        }
    }
}