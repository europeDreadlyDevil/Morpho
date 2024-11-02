use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;
use crate::GLOBAL_ENV;
use std::sync::{Arc, RwLock};

pub fn print_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    for i in 0..args.len() - 1 {
        args[i].print(env.clone())
    }
    args[args.len() - 1].println(env);
    Value::None
}

fn extract_value(value: Value, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    match value {
        Value::CallFunc( call_expr) => {
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
                    return func(parsed_args, env.clone())
                }
            }
            return call_func(call_expr, env.clone())
        }
        _ => {}
    }
    Value::None
}

pub fn if_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    match args[0].clone() {
        Value::Cond(ty, a, b) => {
            if ty.eval_cond(a, b, env.clone()) {
                extract_value(args[1].clone(), env.clone())
            } else {
                extract_value(args[2].clone(), env.clone())
            }
        }
        Value::Bool(b) => {
            if b {
                extract_value(args[1].clone(), env.clone())
            } else {
                extract_value(args[2].clone(), env.clone())
            }
        }
        _ => Value::None
    }
}

pub fn for_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    //println!("ARGS: {args:?}");
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
