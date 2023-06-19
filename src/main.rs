use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Expr {
    Symbol(String),
    Fun(String, Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
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

type Bindings = HashMap<String, Expr>;

impl Expr {
    #[inline]
    fn symbol(name: &'static str) -> Self {
        return Self::Symbol(name.to_string());
    }

    #[inline]
    fn function(name: &'static str, args: Vec<Self>) -> Self {
        return Self::Fun(name.to_string(), args);
    }

    fn pattern_matches(pattern: &Expr, Expr: &Expr, bindings: &mut Bindings) -> bool {
        match (pattern, Expr) {
            (Expr::Symbol(name), _) => {
                if let Some(binding) = bindings.get(name) {
                    return binding == Expr;
                }

                bindings.insert(name.clone(), Expr.clone());
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
            _ => false,
        }
    }

    fn pattern_match(&self, Expr: &Expr) -> Option<Bindings> {
        let mut bindings: Bindings = HashMap::new();

        if !Self::pattern_matches(self, Expr, &mut bindings) {
            return None;
        }

        Some(bindings)
    }
}

#[derive(Debug)]
struct Rule {
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
    fn make(head: Expr, body: Expr) -> Self {
        Self { head, body }
    }

    fn substitute_bindings(&self, bindings: &Bindings, Expr: &Expr) -> Expr {
        match Expr {
            Expr::Symbol(name) => {
                if let Some(binding) = bindings.get(name) {
                    return binding.clone();
                }

                Expr.clone()
            }

            Expr::Fun(name, args) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(args.len());

                for arg in args {
                    new_args.push(self.substitute_bindings(bindings, arg));
                }

                Expr::Fun(name.clone(), new_args)
            }
        }
    }

    fn apply_all(&self, Expr: Expr) -> Expr {
        if let Some(bindings) = self.head.pattern_match(&Expr) {
            return self.substitute_bindings(&bindings, &self.body);
        }

        match Expr {
            Expr::Symbol(_) => Expr.clone(),
            Expr::Fun(name, args) => {
                let mut new_args: Vec<Expr> = Vec::with_capacity(args.len());

                for arg in args {
                    new_args.push(self.apply_all(arg));
                }

                Expr::Fun(name, new_args)
            }
        }
    }
}

fn main() {}
