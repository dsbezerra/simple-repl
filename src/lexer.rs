
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number,
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

    fn peek(&mut self, offset: usize) -> Option<char> {
        let index = self.position + offset;
        if index >= self.input.len() {
            return Some('\0');
        }

        self.input.chars().nth(index)
    }

    fn current(&mut self) -> char {
        self.peek(0).unwrap_or_default()
    }

    fn lex_number(&mut self) -> Token {        
        Token { kind: TokenKind::Number }
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
            '\0' => Token { kind: TokenKind::EndOfFile },
            '+' => Token { kind: TokenKind::Plus },
            '-' => Token { kind: TokenKind::Minus },
            '*' => Token { kind: TokenKind::Asterisk },
            '/' => Token { kind: TokenKind::Slash },
            char if char.is_digit(10) => self.lex_number(),
            _ => Token { kind: TokenKind::BadToken },
        };

        self.position += char.len_utf8();
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
            Token { kind: TokenKind::Plus },
            Token { kind: TokenKind::Slash },
        ];

        for token in expected {
            assert_eq!(lexer.next_tok(), token);    
        }
    }

    #[test]
    fn next_tok() {
        let input = "+-*/";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token { kind: TokenKind::Plus }, 
            Token { kind: TokenKind::Minus }, 
            Token { kind: TokenKind::Asterisk },
            Token { kind: TokenKind::Slash },
        ];

        for token in expected {
            assert_eq!(lexer.next_tok(), token);    
        }
    }
}