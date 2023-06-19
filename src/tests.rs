#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_display() {
        let expr = Expr::function("foo", vec![Expr::symbol("x"), Expr::symbol("y")]);
        assert_eq!(format!("{}", expr), "foo(x, y)");
    }

    #[test]
    fn test_expr_pattern_match() {
        let pattern = Expr::function("foo", vec![Expr::symbol("x"), Expr::symbol("y")]);
        let expr = Expr::function("foo", vec![Expr::symbol("a"), Expr::symbol("b")]);

        let bindings = pattern.pattern_match(&expr);
        assert_eq!(bindings.unwrap().len(), 2);
        assert_eq!(bindings.unwrap()["x"], Expr::symbol("a"));
        assert_eq!(bindings.unwrap()["y"], Expr::symbol("b"));
    }

    #[test]
    fn test_rule_apply_all() {
        let rule = Rule::make(
            Expr::function("foo", vec![Expr::symbol("x"), Expr::symbol("y")]),
            Expr::function("bar", vec![Expr::symbol("y"), Expr::symbol("x")]),
        );

        let expr = Expr::function("foo", vec![Expr::symbol("a"), Expr::symbol("b")]);
        let result = rule.apply_all(expr);
        assert_eq!(
            result,
            Expr::function("bar", vec![Expr::symbol("b"), Expr::symbol("a")])
        );
    }
}
