use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;
use crate::GLOBAL_ENV;
use std::sync::{Arc, RwLock};

pub fn print_func(args: Vec<Value>, _env: Arc<RwLock<LocalEnvironment>>) -> Value {
    //println!("ARGS: {:?}", args);
    for i in 0..args.len() - 1 {
        print!("{} ", args[i]);
    }
    println!("{}", args[args.len() - 1]);
    Value::None
}

pub fn if_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    if let Value::Eq(a, b) = args[0].clone() {
        if eval_expr(*a, env.clone()) == eval_expr(*b, env.clone()) {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env.clone()))
                }
                if let Some(func) = GLOBAL_ENV
                    .try_read()
                    .unwrap()
                    .global_stmts
                    .get(&call_expr.get_name())
                {
                    if let Value::FuncPtr(func) = func.try_read().unwrap().clone() {
                        return func(parsed_args, env.clone());
                    }
                }
                call_func(call_expr, env.clone());
            }
        } else {
            if let Value::CallFunc(call_expr) = args[2].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env.clone()))
                }
                if let Some(func) = GLOBAL_ENV
                    .try_read()
                    .unwrap()
                    .global_stmts
                    .get(&call_expr.get_name())
                {
                    if let Value::FuncPtr(func) = func.try_read().unwrap().clone() {
                        return func(parsed_args, env.clone());
                    }
                }
                call_func(call_expr, env.clone());
            }
        }
    }
    Value::None
}

pub fn for_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    match args[0].clone() {
        Value::Range(start, end) => {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env.clone()))
                }

                if let Some(func) = GLOBAL_ENV
                    .try_read()
                    .unwrap()
                    .global_stmts
                    .get(&call_expr.get_name())
                {
                    if let Value::FuncPtr(func) = func.try_read().unwrap().clone() {
                        for _ in start..end {
                            func(parsed_args.clone(), env.clone());
                        }
                        return Value::None;
                    }
                }
                for _ in start..end {
                    call_func(call_expr.clone(), env.clone());
                }
            }
        }
        Value::Counter(ident, start, end) => {
            if let Value::CallFunc(call_expr) = args[1].clone() {
                let mut parsed_args = vec![];

                for arg in call_expr.get_args() {
                    parsed_args.push(eval_expr(arg, env.clone()))
                }

                if let Some(func) = GLOBAL_ENV
                    .try_read()
                    .unwrap()
                    .global_stmts
                    .get(&call_expr.get_name())
                {
                    if let Value::FuncPtr(func) = func.try_read().unwrap().clone() {
                        for i in start..end {
                            env.try_write()
                                .unwrap()
                                .variables
                                .insert(ident.clone(), Arc::new(RwLock::new(Value::Int(i))));
                            func(parsed_args.clone(), env.clone());
                        }
                        return Value::None;
                    }
                }
                for i in start..end {
                    //println!("{env:?}");
                    *env.try_read()
                        .unwrap()
                        .variables
                        .get(&ident)
                        .unwrap()
                        .try_write()
                        .unwrap() = Value::Int(i);
                    call_func(call_expr.clone(), env.clone());
                }
            }
        }
        _ => {}
    }

    Value::None
}
