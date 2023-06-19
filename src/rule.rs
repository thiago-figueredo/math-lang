use crate::expr::{Bindings, Expr};
use std::fmt;

#[derive(Debug)]
pub struct Rule {
    head: Expr,
    body: Expr,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.head, self.body)
    }
}

impl Rule {
    #[inline]
    pub fn make(head: Expr, body: Expr) -> Self {
        Self { head, body }
    }

    pub fn apply_all(&self, expr: Expr) -> Expr {
        if let Some(bindings) = self.head.pattern_match(&expr) {
            return self.substitute_bindings(&bindings, &self.body);
        }

        match expr {
            Expr::Symbol(_) => expr.clone(),
            Expr::Fun(name, args) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(args.len());

                for arg in args {
                    new_args.push(self.apply_all(arg));
                }

                Expr::Fun(name, new_args)
            }
            Expr::Sentence(expressions) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(expressions.len());

                for expression in expressions {
                    new_args.push(self.apply_all(expression));
                }

                Expr::sentence(new_args)
            }
        }
    }

    pub fn substitute_bindings(&self, bindings: &Bindings, expr: &Expr) -> Expr {
        match expr {
            Expr::Symbol(name) => {
                if let Some(binding) = bindings.get(name) {
                    return binding.clone();
                }

                expr.clone()
            }

            Expr::Fun(name, args) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(args.len());

                for arg in args {
                    new_args.push(self.substitute_bindings(bindings, arg));
                }

                Expr::Fun(name.clone(), new_args)
            }

            Expr::Sentence(expressions) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(expressions.len());

                for expression in expressions {
                    new_args.push(self.substitute_bindings(bindings, expression));
                }

                Expr::sentence(new_args)
            }
        }
    }
}
