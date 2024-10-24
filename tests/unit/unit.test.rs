#[cfg(test)]
mod tests {
    use func_lang::ast::{CallExpr, Expr, FuncBody, FuncIdent, Stmt, VarIdent};
    use func_lang::*;
    #[test]
    fn expr_parsing_test() {
        assert_eq!(
            parser::ExprParser::new().parse("12").unwrap(),
            Expr::Integer(12)
        );
        assert_eq!(
            parser::ExprParser::new().parse("-3.14").unwrap(),
            Expr::Float(-3.14)
        );
        assert_eq!(
            parser::ExprParser::new()
                .parse(r#""Hello, world!""#)
                .unwrap(),
            Expr::StringLit("Hello, world!".into())
        );
        assert_eq!(
            parser::ExprParser::new().parse("main").unwrap(),
            Expr::Ident("main".into())
        );
        assert_eq!(
            parser::ExprParser::new().parse("true").unwrap(),
            Expr::Bool(true)
        );
        assert_eq!(
            parser::ExprParser::new().parse("[12, 543, 3213]").unwrap(),
            Expr::Array(vec![
                Expr::Integer(12),
                Expr::Integer(543),
                Expr::Integer(3213)
            ])
        );
        assert_eq!(
            parser::ExprParser::new()
                .parse(r#"{"key": value, ident: "Bye, world!"}"#)
                .unwrap(),
            Expr::Dictionary(vec![
                (Expr::StringLit("key".into()), Expr::Ident("value".into())),
                (
                    Expr::Ident("ident".into()),
                    Expr::StringLit("Bye, world!".into())
                )
            ])
        );
        assert_eq!(
            parser::ExprParser::new().parse("2+2*9==21*231").unwrap(),
            Expr::Eq(
                Box::new(Expr::Add(
                    Box::new(Expr::Integer(2)),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Integer(2)),
                        Box::new(Expr::Integer(9))
                    ))
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::Integer(21)),
                    Box::new(Expr::Integer(231))
                ))
            )
        )
    }

    #[test]
    fn stmt_parsing_test() {
        assert_eq!(
            parser::StmtParser::new()
                .parse(r#"func main = () {}"#)
                .unwrap(),
            Stmt::FuncIdent(FuncIdent::new_wo_rty(
                "main",
                vec![],
                Some(FuncBody::new(vec![]))
            ))
        );

        assert_eq!(
            parser::StmtParser::new()
                .parse(r#"func main = () -> void {}"#)
                .unwrap(),
            Stmt::FuncIdent(FuncIdent::new_w_rty(
                "main",
                vec![],
                "void".into(),
                Some(FuncBody::new(vec![]))
            ))
        );

        assert_eq!(
            parser::StmtParser::new()
                .parse(r#"func main = () { let var = 10; }"#)
                .unwrap(),
            Stmt::FuncIdent(FuncIdent::new_wo_rty(
                "main",
                vec![],
                Some(FuncBody::new(vec![Stmt::VarIdent(VarIdent::new(
                    "var",
                    Expr::Integer(10)
                ))]))
            ))
        );
        assert_eq!(
            parser::StmtParser::new()
                .parse(r#"func main = () { let var = 10; let str = "Hello, world!"; }"#)
                .unwrap(),
            Stmt::FuncIdent(FuncIdent::new_wo_rty(
                "main",
                vec![],
                Some(FuncBody::new(vec![
                    Stmt::VarIdent(VarIdent::new("var", Expr::Integer(10))),
                    Stmt::VarIdent(VarIdent::new(
                        "str",
                        Expr::StringLit("Hello, world!".into())
                    ))
                ]))
            ))
        );
        assert_eq!(
            parser::StmtParser::new()
                .parse(r#"func main = () { print("Hello, world!"); }"#)
                .unwrap(),
            Stmt::FuncIdent(FuncIdent::new_wo_rty(
                "main",
                vec![],
                Some(FuncBody::new(vec![Stmt::Expr(Box::new(Expr::Call(
                    CallExpr::new(
                        "print".into(),
                        vec![Expr::StringLit("Hello, world!".into())]
                    )
                )))]))
            ))
        );
    }
}
