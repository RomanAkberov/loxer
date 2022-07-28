use std::iter::Peekable;

use crate::{
    ast::{BinaryOperator, Expression, UnaryOperator},
    scanner::Token,
    span::{Span, Spanned},
    value::Value,
};

#[derive(Debug)]
pub enum Error {
    ExpectedPrimary,
    Expected(Token),
    MalformedNumber,
    MalformedString,
}

pub struct Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    input: &'a str,
    tokens: Peekable<I>,
    end: i32,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    pub fn new(input: &'a str, tokens: I) -> Self {
        Self {
            input,
            tokens: tokens.peekable(),
            end: 0,
        }
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        self.binary(
            Self::comparison,
            &[
                (Token::BangEqual, BinaryOperator::NotEqual),
                (Token::EqualEqual, BinaryOperator::Equal),
            ],
        )
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        self.binary(
            Self::term,
            &[
                (Token::Greater, BinaryOperator::Greater),
                (Token::GreaterEqual, BinaryOperator::GreaterEqual),
                (Token::Less, BinaryOperator::Less),
                (Token::LessEqual, BinaryOperator::LessEqual),
            ],
        )
    }

    fn term(&mut self) -> Result<Expression, Error> {
        self.binary(
            Self::factor,
            &[
                (Token::Minus, BinaryOperator::Sub),
                (Token::Plus, BinaryOperator::Add),
            ],
        )
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        self.binary(
            Self::unary,
            &[
                (Token::Slash, BinaryOperator::Div),
                (Token::Star, BinaryOperator::Mul),
            ],
        )
    }

    fn binary<O>(
        &mut self,
        mut operand: O,
        operators: &[(Token, BinaryOperator)],
    ) -> Result<Expression, Error>
    where
        O: FnMut(&mut Self) -> Result<Expression, Error>,
    {
        let mut left = operand(self)?;
        while let Some(operator) = self.match_one_of(operators) {
            let right = operand(self)?;
            left = Expression::Binary(operator, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if let Some(operator) = self.match_one_of(&[
            (Token::Minus, UnaryOperator::Neg),
            (Token::Bang, UnaryOperator::Not),
        ]) {
            let expr = self.unary()?;
            return Ok(Expression::Unary(operator, Box::new(expr)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        match self.next_token() {
            Some(token) => match token.value {
                Token::Nil => Ok(Expression::Literal(Value::Nil)),
                Token::True => Ok(Expression::Literal(Value::Boolean(true))),
                Token::False => Ok(Expression::Literal(Value::Boolean(false))),
                Token::Number => Ok(Expression::Literal(Value::Number(
                    self.parse_number(token.span)?,
                ))),
                Token::String => Ok(Expression::Literal(Value::String(
                    self.parse_string(token.span)?,
                ))),
                Token::LeftParen => {
                    let expression = self.expression()?;
                    self.expect(Token::RightParen)?;
                    Ok(Expression::Grouping(Box::new(expression)))
                }
                _ => Err(Error::ExpectedPrimary),
            },
            None => Err(Error::ExpectedPrimary),
        }
    }

    fn match_one_of<V>(&mut self, values: &[(Token, V)]) -> Option<V>
    where
        V: Copy,
    {
        self.tokens.peek().copied().and_then(|next| {
            for (tt, value) in values.iter() {
                if *tt == next.value {
                    self.next_token();
                    return Some(*value);
                }
            }
            None
        })
    }

    fn expect(&mut self, expected: Token) -> Result<(), Error> {
        match self.tokens.peek().copied() {
            Some(token) if token.value == expected => {
                self.next_token();
                Ok(())
            }
            _ => Err(Error::Expected(expected)),
        }
    }

    fn parse_number(&self, span: Span) -> Result<f64, Error> {
        self.input[span.start as usize..span.end as usize]
            .parse()
            .map_err(|_| Error::MalformedNumber)
    }

    fn parse_string(&self, span: Span) -> Result<String, Error> {
        let str = &self.input[span.start as usize..span.end as usize];
        if !str.ends_with('"') {
            return Err(Error::MalformedString);
        }
        Ok(String::from(
            &self.input[span.start as usize + 1..span.end as usize - 1],
        ))
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.next_token() {
            if let Token::Semicolon = token.value {
                return;
            }
            match self.tokens.peek() {
                Some(token) if starts_statement(token.value) => {
                    return;
                }
                _ => {}
            }
        }
    }

    fn next_token(&mut self) -> Option<Spanned<Token>> {
        self.tokens.next().map(|token| {
            self.end = token.span.end;
            token
        })
    }
}

impl<'a, I> Iterator for Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    type Item = Spanned<Result<Expression, Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.peek().copied().map(|token| {
            let start = token.span.start;
            let result = self.expression();
            if result.is_err() {
                self.synchronize();
            }
            Spanned {
                value: result,
                span: Span {
                    start,
                    end: self.end,
                },
            }
        })
    }
}

fn starts_statement(token: Token) -> bool {
    matches!(
        token,
        Token::Class
            | Token::If
            | Token::Var
            | Token::For
            | Token::Fun
            | Token::While
            | Token::Print
            | Token::Return
    )
}
