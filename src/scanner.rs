use std::{ops::Range, str::Chars};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
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

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub span: Range<u32>,
}

pub struct Scanner<'a> {
    input: &'a str,
    chars: Chars<'a>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            line: 1,
        }
    }

    fn consume_while<P: FnMut(char) -> bool>(&mut self, mut predicate: P) -> Option<char> {
        while !self.is_empty() {
            let peeked = self.peek();
            if !predicate(peeked) {
                return Some(peeked);
            }
            self.next_char();
        }
        None
    }

    fn if_peek(&mut self, ch: char, matched: TokenType, unmatched: TokenType) -> TokenType {
        if self.peek() == ch {
            self.next_char();
            matched
        } else {
            unmatched
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch == Some('\n') {
            self.line += 1;
        }
        ch
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

    fn number(&mut self) -> TokenType {
        if let Some('.') = self.consume_while(|ch| ch.is_ascii_digit() || ch == '.') {
            self.consume_while(|ch| ch.is_ascii_digit());
        }
        TokenType::Number
    }

    fn comment(&mut self) -> TokenType {
        self.consume_while(|ch| ch != '\n');
        TokenType::Comment
    }

    fn string(&mut self) -> TokenType {
        if self.consume_while(|ch| ch != '"').is_some() {
            self.next_char();
        }
        TokenType::String
    }

    fn identifier_or_keyword(&mut self, start: usize) -> TokenType {
        self.consume_while(is_alphanumeric);
        match &self.input[start..self.current_index()] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_while(|ch| ch.is_ascii_whitespace());
        let start = self.current_index();
        let line = self.line;
        self.next_char().map(|ch| {
            let ty = match ch {
                '/' => match self.peek() {
                    '/' => self.comment(),
                    _ => TokenType::Slash,
                },
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::Semicolon,
                '*' => TokenType::Star,
                '!' => self.if_peek('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.if_peek('=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.if_peek('=', TokenType::LessEqual, TokenType::Less),
                '>' => self.if_peek('=', TokenType::GreaterEqual, TokenType::Greater),
                '"' => self.string(),
                ch if ch.is_ascii_digit() => self.number(),
                ch if is_alphabetic(ch) => self.identifier_or_keyword(start),
                _ => TokenType::Unknown,
            };
            let end = self.current_index();
            Token {
                ty,
                line,
                span: start as u32..end as u32,
            }
        })
    }
}

fn is_alphabetic(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_alphanumeric(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
