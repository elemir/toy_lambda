#![feature(plugin)]
#![plugin(oak)]

extern crate liner;
extern crate oak_runtime;

#[allow(dead_code)]
mod parser;
mod reductor;

use liner::Context;

use parser::parse_lambda;
use reductor::call_by_name;
use reductor::call_by_value;
use reductor::Reductor::*;

fn main() {
    let mut con = Context::new();
    let mut reductor = CallByName;

    loop {
        let res = match con.read_line("> ", &mut |_| {}) {
            Ok(a) => {
                if a == ":call-by-name" {
                    reductor = CallByName;
                    println!("call-by-name reduction is using now");
                    continue
                } else if a == ":call-by-value" {
                    reductor = CallByValue;
                    println!("call-by-value reduction is using now");
                    continue
                };
                a
            },
            _ => {
                break
            }
        };

        match (parse_lambda(res.as_str()), &reductor) {
            (Some(a), &CallByName) => println!("{:?}", call_by_name::run_lambda(a)),
            (Some(a), &CallByValue) => println!("{:?}", call_by_value::run_lambda(a)),
            (None, _) => {
                println!("Parse error");
            }
        }

        con.history.push(res.into()).unwrap();
    }
}
