mod expr;
mod lexer;
mod rule;

use expr::{Bindings, Expr};
use lexer::Lexer;
use rule::Rule;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut cmd = String::from("");

    loop {
        cmd.clear();
        print!("> ");
        io::stdout().flush().expect("Error while flushing stdout");

        match io::stdin().read_line(&mut cmd) {
            Ok(_) => {
                println!("{:?}", Expr::parse(Lexer::from_iter(cmd.trim().chars())));
            }
            Err(err) => eprintln!("Error reading input: `{}`", err.to_string()),
        }
    }
}
