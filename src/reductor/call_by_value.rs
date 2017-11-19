use std::collections::HashMap;
use std::clone::Clone;
use std::boxed::Box;

use parser::LambdaExpr;
use parser::LambdaExpr::*;


pub fn run_lambda(expr: LambdaExpr) -> LambdaExpr {
    eval_lambda(expr, &HashMap::new())
}

fn eval_lambda(expr: LambdaExpr, context: &HashMap<String, LambdaExpr>) 
    -> LambdaExpr
{
    match expr {
        Term(term) => 
            if let Some(nexpr) = context.get(&term) {
                nexpr.clone()
            } else {
                Term(term)
            },
        Application(func, arg) => {
            let fexpr = eval_lambda(*func, &context);
            let nexpr = eval_lambda(*arg, &context);
            match fexpr {
                Term(term) => Application(Box::new(Term(term)), Box::new(nexpr)),
                Abstraction(var, body) => {
                    let mut context = context.clone();
                    context.insert(var, nexpr);
                    eval_lambda(*body, &context)
                },
                _ => panic!("Application after full reduce!11")
            }
        },
        _ => expr
  }
}


