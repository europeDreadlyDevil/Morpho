use crate::GLOBAL_ENV;
use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;

pub fn print_func(args: Vec<Value>, _env: &mut LocalEnvironment) -> Value {
    for i in 0..args.len() - 1 {
        print!("{} ", args[i]);
    }
    println!("{}", args[args.len() - 1]);
    Value::None
}

pub fn if_func(args: Vec<Value>, env: &mut LocalEnvironment) -> Value {
    if let Value::Eq(a, b) = args[0].clone() {
        if eval_expr(*a, env) == eval_expr(*b, env) {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env))
                }
                if let Some(func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get(&call_expr.get_name()) {
                    if let Value::FuncPtr(func) = func {

                        return func(parsed_args,env)
                    }
                }
                call_func(call_expr, env);
            }
        } else {
            if let Value::CallFunc(call_expr) = args[2].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env))
                }
                if let Some(func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get(&call_expr.get_name()) {
                    if let Value::FuncPtr(func) = func {

                        return func(parsed_args,env)
                    }
                }
                call_func(call_expr, env);
            }
        }

    }
    Value::None
}

pub fn for_func(args: Vec<Value>, env: &mut LocalEnvironment) -> Value {
    match args[0].clone() {
        Value::Range(start, end) => {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env))
                }

                if let Some(func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get(&call_expr.get_name()) {
                    if let Value::FuncPtr(func) = func {
                        for _ in start..end {
                            func(parsed_args.clone(), env);
                        }
                    }
                    for _ in start..end {
                        call_func(call_expr.clone(), env);
                    }
                }
            }
        }
        Value::Counter(ident, start, end) => {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env))
                }
                
                if let Some(func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get(&call_expr.get_name()) {
                    if let Value::FuncPtr(func) = func {
                        for i in start..end {
                            env.variables.insert(ident.clone(), Value::Int(i));
                            func(parsed_args.clone(), env);
                            return Value::None
                        }
                    }
                }
                for i in start..end {
                    env.variables.insert(ident.clone(), Value::Int(i));
                    call_func(call_expr.clone(), env);
                }

            }
        }
        _ => {}
    }
    
    Value::None
}