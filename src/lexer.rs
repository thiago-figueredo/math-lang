use std::iter::{Iterator, Peekable};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Symbol,
    OpenParen,
    CloseParen,
    Comma,
    Equal,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    fn make(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

pub struct Lexer<Chars>
where
    Chars: Iterator<Item = char> + Clone,
{
    chars: Peekable<Chars>,
}

impl<Chars> Iterator for Lexer<Chars>
where
    Chars: Iterator<Item = char> + Clone,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.chars.next() {
                Some(')') => Some(Token::make(TokenKind::CloseParen, String::from(")"))),
                Some('(') => Some(Token::make(TokenKind::OpenParen, String::from("("))),
                Some(',') => Some(Token::make(TokenKind::Comma, String::from(","))),
                Some('=') => Some(Token::make(TokenKind::Equal, String::from("="))),
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    let mut text = String::from(c);

                    while let Some(c) = self.chars.next_if(|c| c.is_alphanumeric()) {
                        text.push(c);
                    }

                    Some(Token::make(TokenKind::Symbol, text))
                }
                None => None,
            };
        }
    }
}

impl<Chars> Lexer<Chars>
where
    Chars: Iterator<Item = char> + Clone,
{
    pub fn from_iter(source: Chars) -> Self {
        Self {
            chars: source.peekable(),
        }
    }
}
