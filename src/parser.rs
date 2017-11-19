use oak_runtime::IntoState;
use oak_runtime::ParseResult::Success;

pub use self::toy_lambda::LambdaExpr;

grammar! toy_lambda {
    spacing = [" \n\r\t"]* -> (^)

    lambda  = "\\" spacing
    lparen  = "("    spacing
    rparen  = ")"    spacing
    arrow   = "->" spacing

    ident   = ["a-zA-Z0-9_"]+ spacing > to_string

    expr
        = lambda ident+ arrow expr > abstr_expr
        / app_expr app_expr*       > app_expr

    app_expr
        = ident              > term_expr
        / lparen expr rparen

    program = spacing expr

    use self::LambdaExpr::*;

    pub type PExpr = Box<LambdaExpr>;

    #[derive(Debug, Clone)]
    pub enum LambdaExpr {
        Abstraction(String, PExpr),
        Application(PExpr, PExpr),
        Term(String)
    }

    fn to_string(raw_text: Vec<char>) -> String {
        raw_text.into_iter().collect()
    }

    fn abstr_expr(vars: Vec<String>, expr: PExpr) -> PExpr {
        vars.into_iter().rev().fold(expr,
            |accu, var| Box::new(Abstraction(var, accu)))
    }

    fn app_expr(func: PExpr, exprs: Vec<PExpr>) -> PExpr {
        exprs.into_iter().fold(func,
            |accu, expr| Box::new(Application(accu, expr)))
    }

    fn term_expr(ident: String) -> PExpr {
        Box::new(Term(ident))
    }
}

pub fn parse_lambda(input: &str) -> Option<LambdaExpr> {
    match toy_lambda::parse_program(input.into_state()).into_result() {
        Success(boxed) => Some(*boxed),
        _ => None
    }
}
