use std::str::Chars;

use crate::span::{Span, Spanned};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Comments.
    Comment,

    // Unknown.
    Unknown,
}

pub struct Scanner<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
        }
    }

    fn consume_while<P>(&mut self, mut predicate: P) -> Option<char>
    where
        P: FnMut(char) -> bool,
    {
        while !self.is_empty() {
            let peeked = self.peek();
            if !predicate(peeked) {
                return Some(peeked);
            }
            self.next_char();
        }
        None
    }

    fn if_peek(&mut self, ch: char, matched: Token, unmatched: Token) -> Token {
        if self.peek() == ch {
            self.next_char();
            matched
        } else {
            unmatched
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn is_empty(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn current_index(&self) -> usize {
        self.input.len() - self.chars.as_str().len()
    }

    fn number(&mut self) -> Token {
        if let Some('.') = self.consume_while(|ch| ch.is_ascii_digit() || ch == '.') {
            self.consume_while(|ch| ch.is_ascii_digit());
        }
        Token::Number
    }

    fn comment(&mut self) -> Token {
        self.consume_while(|ch| ch != '\n');
        Token::Comment
    }

    fn string(&mut self) -> Token {
        if self.consume_while(|ch| ch != '"').is_some() {
            self.next_char();
        }
        Token::String
    }

    fn identifier_or_keyword(&mut self, start: usize) -> Token {
        self.consume_while(is_alphanumeric);
        match &self.input[start..self.current_index()] {
            "and" => Token::And,
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "for" => Token::For,
            "fun" => Token::Fun,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "var" => Token::Var,
            "while" => Token::While,
            _ => Token::Identifier,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_while(|ch| ch.is_ascii_whitespace());
        let start = self.current_index();
        self.next_char().map(|ch| {
            let token = match ch {
                '/' => match self.peek() {
                    '/' => self.comment(),
                    _ => Token::Slash,
                },
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '-' => Token::Minus,
                '+' => Token::Plus,
                ';' => Token::Semicolon,
                '*' => Token::Star,
                '!' => self.if_peek('=', Token::BangEqual, Token::Bang),
                '=' => self.if_peek('=', Token::EqualEqual, Token::Equal),
                '<' => self.if_peek('=', Token::LessEqual, Token::Less),
                '>' => self.if_peek('=', Token::GreaterEqual, Token::Greater),
                '"' => self.string(),
                ch if ch.is_ascii_digit() => self.number(),
                ch if is_alphabetic(ch) => self.identifier_or_keyword(start),
                _ => Token::Unknown,
            };
            let span = Span {
                start: start as i32,
                end: self.current_index() as i32,
            };
            Spanned { value: token, span }
        })
    }
}

fn is_alphabetic(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_alphanumeric(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
