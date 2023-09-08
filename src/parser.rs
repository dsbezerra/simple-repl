use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
    position: usize,
    pub diagnostics: Vec<String>,
}

#[derive(Debug)]
pub enum Number {
    Integer(isize),
    Decimal(f64),
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
    pub fn is_number(&self) -> bool {
        match self {
            Expression::Number(_) => true,
            _ => false
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Expression::Binary(_, _, _) => true,
            _ => false
        }
    }

    pub fn is_number_integer(&self) -> bool {
        match self {
            Expression::Number(Number::Integer(_)) => true,
            _ => false
        }
    }

    pub fn is_number_decimal(&self) -> bool {
        match self {
            Expression::Number(Number::Decimal(_)) => true,
            _ => false
        }
    }

    pub fn evaluate_decimal(&self) -> f64 {
        match self {
            Expression::Number(Number::Decimal(value)) => *value,
            _ => unreachable!()
        }
    }

    pub fn evaluate_int(&self) -> isize {
        match self {
            Expression::Number(Number::Integer(value)) => *value,
            _ => unreachable!()
        }
    }

    pub fn evaluate_binary_dec(&self) -> f64 {
        match self {
            Expression::Binary(left, operator, right) => {
                match operator {
                    Operator::Add => left.evaluate_decimal() + right.evaluate_decimal(),
                    Operator::Subtract => left.evaluate_decimal() - right.evaluate_decimal(),
                    _ => todo!()
                }
            }
            _ => unreachable!()
        }
    }

    pub fn evaluate_binary_int(&self) -> isize {
        match self {
            Expression::Binary(left, operator, right) => {
                match operator {
                    Operator::Add => left.evaluate_int() + right.evaluate_int(),
                    Operator::Subtract => left.evaluate_int() - right.evaluate_int(),
                    _ => todo!()
                }
            }
            _ => unreachable!()
        }
    }

    pub fn evaluate(&self) {
        if self.is_number() {
            if self.is_number_integer() {
                println!("{}", self.evaluate_int());
                return;
            }
            if self.is_number_decimal() {
                println!("{}", self.evaluate_decimal());
                return;
            }
        }

        match self {
            Expression::Binary(left, _, right) => {
                if left.is_number_integer() {
                    println!("{}", self.evaluate_binary_int());
                    return;
                }
                println!("{}", self.evaluate_binary_dec());
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl<'a> Parser {
    pub fn new(input: &'a str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_tok();
            match token.kind {
                TokenKind::EndOfFile => break,
                TokenKind::BadToken => continue,
                _ => tokens.push(token)
            }
        }

        Self {
            tokens,
            position: 0,
            diagnostics: vec![],
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        let index = self.position + offset;
        if index >= self.tokens.len() {
            return self.tokens.last();
        }

        return Some(&self.tokens[index]);
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn match_token(&self, kind: TokenKind) -> bool {
        let current = self.current().unwrap();
        if current.kind == kind {
            return true;
        }

        return false;
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        let token = self.current().unwrap();
        return match token.kind {
            TokenKind::Integer(value) => Some(Expression::Number(Number::Integer(value))),
            TokenKind::Decimal(value) => Some(Expression::Number(Number::Decimal(value))),
            _ => {
                self.diagnostics.push(
                    format!("Expected an integer or decimal, but got a {:?}", token.text)
                );
                None
            }
        };
    }

    fn advance(&mut self, offset: usize) {
        self.position += offset
    }

    pub fn parse(&mut self) -> Expression {
        let mut left = self.parse_expression();

        while left.is_some() {
            self.advance(1);

            let token = self.current().unwrap();
            if token.kind == TokenKind::Plus || token.kind == TokenKind::Minus {
                let operator = match token.kind {
                    TokenKind::Plus => Operator::Add,
                    TokenKind::Minus => Operator::Subtract,
                    _ => {
                        self.diagnostics.push(
                            format!("Expected a + or - but got a {:?}", token.kind)
                        );
                        break;
                    }
                };

                self.advance(1);

                let right = self.parse_expression();
                match right {
                    None => break,
                    Some(_) => {
                        left = Some(
                            Expression::Binary(Box::new(left.unwrap()), operator, Box::new(right.unwrap()))
                        );
                    }
                }
            } else {
                break;
            }
        }

        return left.unwrap();
    }
}