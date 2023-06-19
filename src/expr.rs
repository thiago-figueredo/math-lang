use crate::lexer::{Token, TokenKind};
use std::iter::{Iterator, Peekable};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Symbol(String),
    Sentence(Vec<Expr>),
    Fun(String, Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Sentence(expressions) => {
                for expr in expressions {
                    write!(f, "{}", expr);
                }

                write!(f, "")
            }
            Expr::Fun(name, args) => {
                write!(f, "{}(", name)?;

                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", arg)?;
                }

                write!(f, ")")
            }
        }
    }
}

pub type Bindings = HashMap<String, Expr>;

impl Expr {
    #[inline]
    pub fn symbol(name: String) -> Self {
        Self::Symbol(name)
    }

    #[inline]
    pub fn function(name: String, args: Vec<Self>) -> Self {
        Self::Fun(name, args)
    }

    #[inline]
    pub fn sentence(args: Vec<Self>) -> Self {
        Self::Sentence(args)
    }

    pub fn parse_function_args(
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        name: String,
    ) -> Self {
        let mut args: Vec<Expr> = Vec::new();

        match lexer.next() {
            Some(token) => {
                args.push(Self::parse_token(lexer, token));

                if let Some(_) = lexer.next_if(|t| t.kind == TokenKind::Comma) {
                    while let Some(token) = lexer.next_if(|t| t.kind != TokenKind::CloseParen) {
                        args.push(Self::parse_token(lexer, token))
                    }
                }
            }
            None => {}
        }

        Expr::function(name, args)
    }

    pub fn parse_peekable(lexer: &mut Peekable<impl Iterator<Item = Token>>) -> Self {
        if let Some(token) = lexer.skip_while(|t| t.text.trim().is_empty()).next() {
            return Self::parse_token(lexer, token);
        }

        todo!();
    }

    pub fn parse(lexer: impl Iterator<Item = Token>) -> Self {
        return Self::parse_peekable(&mut lexer.peekable());
    }

    pub fn pattern_match(&self, expr: &Expr) -> Option<Bindings> {
        let mut bindings: Bindings = HashMap::new();

        if !Self::pattern_matches(self, expr, &mut bindings) {
            return None;
        }

        Some(bindings)
    }

    fn parse_token(lexer: &mut Peekable<impl Iterator<Item = Token>>, token: Token) -> Self {
        match token.kind {
            TokenKind::Symbol => {
                if let Some(_) = lexer.next_if(|t| t.kind == TokenKind::OpenParen) {
                    if let Some(_) = lexer.next_if(|t| t.kind == TokenKind::CloseParen) {
                        return Expr::function(token.text, vec![]);
                    }

                    return Self::parse_function_args(lexer, token.text);
                }

                return Expr::symbol(token.text);
            }
            _ => todo!(),
        }
    }

    fn pattern_matches(pattern: &Expr, expr: &Expr, bindings: &mut Bindings) -> bool {
        match (pattern, expr) {
            (Expr::Symbol(name), _) => {
                if let Some(binding) = bindings.get(name) {
                    return binding == expr;
                }

                bindings.insert(name.clone(), expr.clone());
                true
            }
            (Expr::Fun(name1, args1), Expr::Fun(name2, args2)) => {
                if name1 != name2 || args1.len() != args2.len() {
                    return false;
                }

                for i in 0..args1.len() {
                    if !Self::pattern_matches(&args1[i], &args2[i], bindings) {
                        return false;
                    }
                }

                true
            }
            (Expr::Sentence(expressions1), Expr::Sentence(expressions2)) => {
                if expressions1.len() != expressions2.len() {
                    return false;
                }

                for expr in expressions1 {
                    if !Self::pattern_matches(pattern, expr, bindings) {
                        return false;
                    }
                }

                true
            }
            _ => false,
        }
    }
}
