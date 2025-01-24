use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;
use crate::GLOBAL_ENV;
use std::sync::{Arc, RwLock};

#[inline]
pub fn print_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    for i in 0..args.len() - 1 {
        args[i].print(env.clone())
    }
    args[args.len() - 1].println(env);
    Value::Void
}

#[inline]
fn extract_value(value: Value, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    match value {
        Value::CallFunc(call_expr) => {
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
            return call_func(call_expr, env.clone());
        }
        _ => {}
    }
    Value::Void
}

#[inline]
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
        _ => Value::Void,
    }
}

#[inline(always)]
pub fn for_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    let (start, end, ident) = match args[0].clone() {
        Value::Range(start, end) => (start, end, None),
        Value::Counter(ident, start, end) => (start, end, Some(ident)),
        _ => return Value::Void,
    };

    let call_expr = match args[1].clone() {
        Value::CallFunc(call_expr) => call_expr,
        _ => return Value::Void,
    };

    let parsed_args: Vec<Value> = call_expr.get_args()
        .iter()
        .map(|arg| eval_expr(arg.clone(), env.clone()))
        .collect();

    let func_option = GLOBAL_ENV
        .try_read()
        .unwrap()
        .global_stmts
        .get(&call_expr.get_name())
        .and_then(|func| {
            if let Value::FuncPtr(func_ptr) = func.try_read().unwrap().clone() {
                Some(func_ptr)
            } else {
                None
            }
        });

    for i in start..end {
        if let Some(ref ident) = ident {
            env.try_write().unwrap().variables.insert(ident.clone(), Arc::new(RwLock::new(Value::Int(i))));
        }
        if let Some(func) = func_option {
            func(parsed_args.clone(), env.clone());
        } else {
            call_func(call_expr.clone(), env.clone());
        }
    }

    Value::Void
}

pub fn while_func(args: Vec<Value>, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    let (ty, lhs, rhs) = match args[0].clone() {
        Value::Cond(ty, lhs, rhs) => (ty, lhs, rhs),
        _ => return Value::Void,
    };

    let call_expr = match args[1].clone() {
        Value::CallFunc(call_expr) => call_expr,
        _ => return Value::Void,
    };

    let parsed_args: Vec<Value> = call_expr.get_args()
        .iter()
        .map(|arg| eval_expr(arg.clone(), env.clone()))
        .collect();

    let func_option = GLOBAL_ENV
        .try_read()
        .unwrap()
        .global_stmts
        .get(&call_expr.get_name())
        .and_then(|func| {
            if let Value::FuncPtr(func_ptr) = func.try_read().unwrap().clone() {
                Some(func_ptr)
            } else {
                None
            }
        });

    while ty.eval_cond(lhs.clone(), rhs.clone(), env.clone()) {
        if let Some(func) = func_option {
            func(parsed_args.clone(), env.clone());
        } else {
            call_func(call_expr.clone(), env.clone());
        }
    }
    Value::Void
}

pub fn input_func(_args: Vec<Value>, _env: Arc<RwLock<LocalEnvironment>>) -> Value {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        panic!("{}",e);
    }
    Value::String(input.trim().to_string())
}