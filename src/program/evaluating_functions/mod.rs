use crate::ast::{CallExpr, Expr, FuncBody, Prog, Stmt};
use crate::program::environment::LocalEnvironment;
use crate::program::function::Function;
use crate::program::value::{CondType, Value};
use crate::program::Program;
use crate::GLOBAL_ENV;
use std::collections::HashMap;
use std::ops::{Neg, Not};
use std::sync::{Arc, RwLock};
use trees::{Forest, Tree};
use uuid::Uuid;
#[inline]
pub fn eval_program(prog: Prog) -> anyhow::Result<()> {
    let prog = Program::new(prog)?;
    prog.run()?;
    Ok(())
}

#[inline]
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

#[inline]
pub fn eval_expr(expr: Expr, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    println!("EXPR: {expr:?}");

    // let mut expr_tree = trees::Tree::new(expr.clone());
    // expr_tree.push_back(Tree::new(Expr::Integer(10)));
    // expr_tree.push_back(Tree::new(Expr::Mul(Box::new(Expr::Integer(10)), Box::new(Expr::Integer(35)))));
    // println!("{}", expr_tree.to_string());

    let mut expr_stack = vec![expr.clone()];
    let mut curr_exprs = vec![expr];
    while let Some(expr) = curr_exprs.pop() {
        println!("CURR_EXPR: {:?}", curr_exprs);
        match expr {
            Expr::Add(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Sub(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Mul(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Div(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Xor(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Mod(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            _ => {}
        }
    }

    println!("EXPR_STACK: {expr_stack:#?}");

    let mut value = Value::None;
    let mut rhs = Value::None;
    let mut lhs = Value::None;
    while let Some(expr) = expr_stack.pop() {
        println!("LHS: {lhs} RHS: {rhs}");
        match expr {
            Expr::Integer(v) => if let Value::None = rhs {
                rhs = Value::Int(v);
            } else {
                lhs = Value::Int(v);
            }
            Expr::Float(v) => if let Value::None = rhs {
                rhs = Value::Float(v);
            } else {
                lhs = Value::Float(v);
            }
            Expr::Bool(v) => if let Value::None = rhs {
                rhs = Value::Bool(v);
            } else {
                lhs = Value::Bool(v);
            }
            Expr::StringLit(v) => if let Value::None = rhs {
                rhs = Value::String(v);
            } else {
                lhs = Value::String(v);
            }
            Expr::Add(_, _) => {
                rhs = lhs + rhs;
                lhs = Value::None
            }
            Expr::Sub(_, _) => {
                rhs = lhs - rhs;
                lhs = Value::None
            }
            Expr::Mul(_, _) => {
                rhs = lhs * rhs;
                lhs = Value::None
            }
            Expr::Div(_, _) => {
                rhs = lhs / rhs;
                lhs = Value::None
            }
            Expr::Eq(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Eq, l, r);
            } else {
                lhs = Value::Cond(CondType::Eq, l, r);
            }
            Expr::NotEq(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Ne, l, r);
            } else {
                lhs = Value::Cond(CondType::Ne, l, r);
            }
            Expr::Gt(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Gt, l, r);
            } else {
                lhs = Value::Cond(CondType::Gt, l, r);
            }
            Expr::Lt(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Lt, l, r);
            } else {
                lhs = Value::Cond(CondType::Lt, l, r);
            }
            Expr::Ge(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Ge, l, r);
            } else {
                lhs = Value::Cond(CondType::Ge, l, r);
            }
            Expr::Le(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Le, l, r);
            } else {
                lhs = Value::Cond(CondType::Le, l, r);
            }
            Expr::Or(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::Or, l, r);
            } else {
                lhs = Value::Cond(CondType::Or, l, r);
            }
            Expr::And(l, r) => if let Value::None = rhs {
                rhs = Value::Cond(CondType::And, l, r);
            } else {
                lhs = Value::Cond(CondType::And, l, r);
            }
            Expr::Not(_) => if let Value::None = rhs {
                lhs = lhs.not()
            } else {
                rhs = rhs.not()
            }
            Expr::Neg(_) => if let Value::None = rhs {
                lhs = lhs.neg()
            } else {
                rhs = rhs.neg()
            }
            Expr::Xor(_, _) => {
                rhs = lhs ^ rhs;
                lhs = Value::None
            }
            Expr::Mod(_, _) => {
                rhs = lhs % rhs;
                lhs = Value::None
            }
            Expr::Call(call_expr) => if let Value::None = rhs {
                rhs = call_func(call_expr, env.clone())
            } else {
                lhs = call_func(call_expr, env.clone())
            }
            Expr::Ident(ident) => if let Value::None = rhs {
                rhs = env
                    .try_read()
                    .unwrap()
                    .variables
                    .get(&ident)
                    .unwrap()
                    .clone()
                    .try_read()
                    .unwrap()
                    .clone();
            } else {
                lhs = env
                    .try_read()
                    .unwrap()
                    .variables
                    .get(&ident)
                    .unwrap()
                    .clone()
                    .try_read()
                    .unwrap()
                    .clone();
            }
            _ => {}
        }
    }

    value = rhs;
    println!("FINAL: {value:?}");
    value
    // match expr {
    //     Expr::Ident(ident) => env
    //         .try_read()
    //         .unwrap()
    //         .variables
    //         .get(&ident)
    //         .unwrap()
    //         .clone()
    //         .try_read()
    //         .unwrap()
    //         .clone(),
    //     Expr::Call(call_expr) => {
    //         call_func(call_expr, env)
    //     }
    //     Expr::Func(f_ptr) => {
    //         Value::CallFunc(CallExpr::new(f_ptr.ident.clone(), f_ptr.args.unwrap()))
    //     }
    //     Expr::AnonFunc(a_func) => {
    //         let ident = Uuid::new_v4().to_string();
    //         let mut args = vec![];
    //         let mut call_args = vec![];
    //         for (ident, expr) in a_func.args.clone() {
    //             let value = eval_expr(expr.clone(), env.clone());
    //             args.push((ident, value.into_type().to_string()));
    //             call_args.push(expr)
    //         }
    //         let func = Function::new(
    //             HashMap::new(),
    //             env.clone(),
    //             ident.clone(),
    //             args,
    //             a_func.rty,
    //             a_func.stmt.unwrap().stmt,
    //         );
    //         env.try_write()
    //             .unwrap()
    //             .variables
    //             .insert(ident.clone(), Arc::new(RwLock::new(Value::Func(func))));
    //         Value::CallFunc(CallExpr::new(ident, call_args))
    //     }
    //     Expr::Eq(l, r) => Value::Cond(CondType::Eq, l, r),
    //     Expr::NotEq(l, r) => Value::Cond(CondType::Ne, l, r),
    //     Expr::Gt(l, r) => Value::Cond(CondType::Gt, l, r),
    //     Expr::Lt(l, r) => Value::Cond(CondType::Lt, l, r),
    //     Expr::Ge(l, r) => Value::Cond(CondType::Ge, l, r),
    //     Expr::Le(l, r) => Value::Cond(CondType::Le, l, r),
    //     Expr::Add(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l + r
    //     }
    //     Expr::Sub(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l - r
    //     }
    //     Expr::Mul(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l * r
    //     }
    //     Expr::Div(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l / r
    //     }
    //     Expr::Range((start, end)) => Value::Range(start, end),
    //     Expr::Counter((ident, (start, end))) => {
    //         env.try_write()
    //             .unwrap()
    //             .variables
    //             .insert(ident.clone(), Arc::new(RwLock::new(Value::Int(start))));
    //         Value::Counter(ident, start, end)
    //     }
    //     Expr::Ref(expr) => match *expr {
    //         Expr::Ident(ident) => Value::RefValue(
    //             env.try_read()
    //                 .unwrap()
    //                 .variables
    //                 .get(&ident)
    //                 .unwrap()
    //                 .clone(),
    //         ),
    //         _ => Value::RefValue(Arc::new(RwLock::new(eval_expr(*expr, env.clone())))),
    //     },
    //     Expr::Not(expr) => {
    //         let rhs = eval_expr(*expr, env.clone());
    //         rhs.not()
    //     }
    //     Expr::Neg(expr) => {
    //         let rhs = eval_expr(*expr, env.clone());
    //         rhs.neg()
    //     }
    //     Expr::Or(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l.logical_or(r, env.clone())
    //     }
    //     Expr::And(l, r) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l.logical_and(r)
    //     }
    //     Expr::Mod(l, r ) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l%r
    //     }
    //     Expr::Xor(l, r ) => {
    //         let l = eval_expr(*l, env.clone());
    //         let r = eval_expr(*r, env.clone());
    //         l^r
    //     }
    //     _ => eval_primitive_expr(expr, env.clone()),
    // }
}

#[inline]
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
            Value::FuncPtr(func) => func(parsed_args, env.clone()),
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
            _ => panic!("Expected func"),
        };
    }};
}

#[inline]
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
