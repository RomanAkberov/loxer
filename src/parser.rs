use std::iter::Peekable;

use crate::{
    ast::{BinaryOperator, Expression, UnaryOperator},
    scanner::{Token, TokenType},
    value::Value,
};

#[derive(Debug)]
pub enum ParseError {
    ExpectedPrimary(Option<Token>),
    Expected(TokenType, Option<Token>),
    MalformedNumber(Token),
    MalformedString(Token),
}

pub struct Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    input: &'a str,
    tokens: Peekable<I>,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(input: &'a str, tokens: I) -> Self {
        Self {
            input,
            tokens: tokens.peekable(),
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        self.binary(
            Self::comparison,
            &[
                (TokenType::BangEqual, BinaryOperator::NotEqual),
                (TokenType::EqualEqual, BinaryOperator::Equal),
            ],
        )
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        self.binary(
            Self::term,
            &[
                (TokenType::Greater, BinaryOperator::Greater),
                (TokenType::GreaterEqual, BinaryOperator::GreaterEqual),
                (TokenType::Less, BinaryOperator::Less),
                (TokenType::LessEqual, BinaryOperator::LessEqual),
            ],
        )
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        self.binary(
            Self::factor,
            &[
                (TokenType::Minus, BinaryOperator::Sub),
                (TokenType::Plus, BinaryOperator::Add),
            ],
        )
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        self.binary(
            Self::unary,
            &[
                (TokenType::Slash, BinaryOperator::Div),
                (TokenType::Star, BinaryOperator::Mul),
            ],
        )
    }

    fn binary<O>(
        &mut self,
        mut operand: O,
        operators: &[(TokenType, BinaryOperator)],
    ) -> Result<Expression, ParseError>
    where
        O: FnMut(&mut Self) -> Result<Expression, ParseError>,
    {
        let mut left = operand(self)?;
        while let Some(operator) = self.match_one_of(operators) {
            let right = operand(self)?;
            left = Expression::Binary(operator, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if let Some(operator) = self.match_one_of(&[
            (TokenType::Minus, UnaryOperator::Neg),
            (TokenType::Bang, UnaryOperator::Not),
        ]) {
            let expr = self.unary()?;
            return Ok(Expression::Unary(operator, Box::new(expr)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        match self.tokens.next() {
            Some(token) => match token.tt {
                TokenType::Nil => Ok(Expression::Literal(Value::Nil)),
                TokenType::True => Ok(Expression::Literal(Value::Boolean(true))),
                TokenType::False => Ok(Expression::Literal(Value::Boolean(false))),
                TokenType::Number => Ok(Expression::Literal(Value::Number(
                    self.parse_number(token)?,
                ))),
                TokenType::String => Ok(Expression::Literal(Value::String(
                    self.parse_string(token)?,
                ))),
                TokenType::LeftParen => {
                    let expression = self.expression()?;
                    self.expect(TokenType::RightParen)?;
                    Ok(Expression::Grouping(Box::new(expression)))
                }
                _ => Err(ParseError::ExpectedPrimary(Some(token))),
            },
            None => Err(ParseError::ExpectedPrimary(None)),
        }
    }

    fn match_one_of<V>(&mut self, values: &[(TokenType, V)]) -> Option<V>
    where
        V: Copy,
    {
        self.tokens.peek().copied().and_then(|token| {
            for (tt, value) in values.iter() {
                if *tt == token.tt {
                    self.tokens.next();
                    return Some(*value);
                }
            }
            None
        })
    }

    fn expect(&mut self, expected: TokenType) -> Result<(), ParseError> {
        match self.tokens.peek().copied() {
            Some(token) if token.tt == expected => {
                self.tokens.next();
                Ok(())
            }
            token => Err(ParseError::Expected(expected, token)),
        }
    }

    fn parse_number(&self, token: Token) -> Result<f64, ParseError> {
        self.input[token.span.start as usize..token.span.end as usize]
            .parse()
            .map_err(|_| ParseError::MalformedNumber(token))
    }

    fn parse_string(&self, token: Token) -> Result<String, ParseError> {
        let str = &self.input[token.span.start as usize..token.span.end as usize];
        if !str.ends_with('"') {
            return Err(ParseError::MalformedString(token));
        }
        Ok(String::from(
            &self.input[token.span.start as usize + 1..token.span.end as usize - 1],
        ))
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.tokens.next() {
            if let TokenType::Semicolon = token.tt {
                return;
            }
            match self.tokens.peek() {
                Some(token) if starts_statement(token.tt) => {
                    return;
                }
                _ => {}
            }
        }
    }
}

impl<'a, I> Iterator for Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<Expression, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.peek().is_some() {
            Some(self.expression().map_err(|error| {
                self.synchronize();
                error
            }))
        } else {
            None
        }
    }
}

fn starts_statement(tt: TokenType) -> bool {
    matches!(
        tt,
        TokenType::Class
            | TokenType::If
            | TokenType::Var
            | TokenType::For
            | TokenType::Fun
            | TokenType::While
            | TokenType::Print
            | TokenType::Return
    )
}
