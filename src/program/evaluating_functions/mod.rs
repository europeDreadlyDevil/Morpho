use crate::ast::{CallExpr, Expr, FuncBody, Prog, Stmt};
use crate::program::environment::LocalEnvironment;
use crate::program::function::Function;
use crate::program::value::{CondType, Value};
use crate::program::Program;
use crate::GLOBAL_ENV;
use std::collections::HashMap;
use std::ops::{Neg, Not};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub fn eval_program(prog: Prog) -> anyhow::Result<()> {
    let prog = Program::new(prog)?;
    prog.run()?;
    Ok(())
}
pub fn extract_func(func_stmt: Stmt) -> Option<(String, Function)> {
    match func_stmt {
        Stmt::FuncIdent(f_ident) => {
            if let Some(FuncBody { stmt }) = f_ident.stmt {
                return Some((
                    f_ident.ident.clone(),
                    Function::new(
                        HashMap::new(),
                        Arc::new(RwLock::new(LocalEnvironment::new())),
                        f_ident.ident,
                        f_ident.args,
                        f_ident.rty,
                        stmt,
                    ),
                ));
            }
        }
        _ => {}
    }
    None
}

pub fn eval_expr(expr: Expr, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    //println!("EXPR: {expr:?}");
    match expr {
        Expr::Ident(ident) => env
            .try_read()
            .unwrap()
            .variables
            .get(&ident)
            .unwrap()
            .clone()
            .try_read()
            .unwrap()
            .clone(),
        Expr::Call(call_expr) => {
            call_func(call_expr, env)
        }
        Expr::Func(f_ptr) => {
            Value::CallFunc(CallExpr::new(f_ptr.ident.clone(), f_ptr.args.unwrap()))
        }
        Expr::AnonFunc(a_func) => {
            let ident = Uuid::new_v4().to_string();
            let mut args = vec![];
            let mut call_args = vec![];
            for (ident, expr) in a_func.args.clone() {
                let value = eval_expr(expr.clone(), env.clone());
                args.push((ident, value.into_type().to_string()));
                call_args.push(expr)
            }
            let func = Function::new(
                HashMap::new(),
                env.clone(),
                ident.clone(),
                args,
                a_func.rty,
                a_func.stmt.unwrap().stmt,
            );
            env.try_write()
                .unwrap()
                .variables
                .insert(ident.clone(), Arc::new(RwLock::new(Value::Func(func))));
            Value::CallFunc(CallExpr::new(ident, call_args))
        }
        Expr::Eq(l, r) => Value::Cond(CondType::Eq, l, r),
        Expr::NotEq(l, r) => Value::Cond(CondType::Ne, l, r),
        Expr::Gt(l, r) => Value::Cond(CondType::Gt, l, r),
        Expr::Lt(l, r) => Value::Cond(CondType::Lt, l, r),
        Expr::Ge(l, r) => Value::Cond(CondType::Ge, l, r),
        Expr::Le(l, r) => Value::Cond(CondType::Le, l, r),
        Expr::Add(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l + r
        }
        Expr::Sub(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l - r
        }
        Expr::Mul(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l * r
        }
        Expr::Div(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l / r
        }
        Expr::Range((start, end)) => Value::Range(start, end),
        Expr::Counter((ident, (start, end))) => {
            env.try_write()
                .unwrap()
                .variables
                .insert(ident.clone(), Arc::new(RwLock::new(Value::Int(start))));
            Value::Counter(ident, start, end)
        }
        Expr::Ref(expr) => match *expr {
            Expr::Ident(ident) => Value::RefValue(
                env.try_read()
                    .unwrap()
                    .variables
                    .get(&ident)
                    .unwrap()
                    .clone(),
            ),
            _ => Value::RefValue(Arc::new(RwLock::new(eval_expr(*expr, env.clone())))),
        },
        Expr::Not(expr) => {
            let rhs = eval_expr(*expr, env.clone());
            rhs.not()
        }
        Expr::Neg(expr) => {
            let rhs = eval_expr(*expr, env.clone());
            rhs.neg()
        }
        Expr::Or(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l.logical_or(r, env.clone())
        }
        Expr::And(l, r) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l.logical_and(r)
        }
        Expr::Mod(l, r ) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l%r
        }
        Expr::Xor(l, r ) => {
            let l = eval_expr(*l, env.clone());
            let r = eval_expr(*r, env.clone());
            l^r
        }
        _ => eval_primitive_expr(expr, env.clone()),
    }
}

pub fn eval_primitive_expr(expr: Expr, _env: Arc<RwLock<LocalEnvironment>>) -> Value {
    match expr {
        Expr::Integer(i) => Value::Int(i),
        Expr::StringLit(s) => Value::String(s),
        Expr::Bool(b) => Value::Bool(b),
        _ => Value::None,
    }
}

macro_rules! macro_extract_func {
    ($expr: expr, $call_expr: expr, $env: expr) => {{
        let call_expr: CallExpr = $call_expr;
        let env: Arc<RwLock<LocalEnvironment>> = $env;
        let args = call_expr.get_args();
        let mut parsed_args: Vec<Value> = vec![];
        for arg in args {
            parsed_args.push(eval_expr(arg, env.clone()));
        }

        return match $expr.try_read().unwrap().clone() {
            Value::FuncPtr(func) => {
                func(parsed_args, env.clone())
            }
            Value::Func(mut func) => {
                let mut l_env = Arc::new(RwLock::new(LocalEnvironment::new()));
                for i in 0..func.get_args().len() {
                    let (ident, ty) = func.get_args()[i].clone();
                    if Value::Type(ty) == parsed_args[i].clone().into_type() {
                        if let Value::RefValue(r) = parsed_args[i].clone() {
                            l_env.try_write().unwrap().variables.insert(ident, r);
                        } else {
                            l_env
                                .try_write()
                                .unwrap()
                                .variables
                                .insert(ident, Arc::new(RwLock::new(parsed_args[i].clone())));
                        }
                    } else {
                        panic!("Expected other type")
                    }
                }
                func.set_env(l_env);
                func.run()

            }
            _ => panic!("Expected func")
        };
    }}
}

pub fn call_func(call_expr: CallExpr, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    let ident = call_expr.get_name();
    match GLOBAL_ENV
        .try_read()
        .unwrap()
        .global_stmts
        .clone()
        .get(&ident)
    {
        None => {
            let func = env
                .try_read()
                .unwrap()
                .variables
                .get(&ident)
                .unwrap()
                .clone();
            macro_extract_func!(func, call_expr, env)
        }
        Some(func) => macro_extract_func!(func.clone(), call_expr, env),
    }
}
