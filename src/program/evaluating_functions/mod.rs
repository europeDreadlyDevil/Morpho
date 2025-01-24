use crate::ast::{CallExpr, Expr, Body, Prog, Stmt, PrivacyType, Import, InlineAccess};
use crate::program::environment::LocalEnvironment;
use crate::program::function::Function;
use crate::program::value::{CondType, Value};
use crate::program::Program;
use crate::{ANON_FUNC_CACHE, GLOBAL_ENV};
use std::collections::HashMap;
use std::ops::{Neg, Not};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::program::module::Module;

#[inline]
pub fn eval_program(prog: Prog) -> anyhow::Result<()> {
    let prog = Program::new(prog)?;
    prog.run()?;
    Ok(())
}

#[inline]
pub fn extract_func(func_stmt: &Stmt) -> Option<(String, Function)> {
    match func_stmt {
        Stmt::FuncIdent(f_ident) => {
            let f_ident = f_ident.clone();
            if let Some(Body { stmt }) = f_ident.stmt {
                return Some((
                    f_ident.ident.clone(),
                    Function::new(
                        f_ident.privacy_type,
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

pub fn extract_module(mod_stmt: &Stmt) -> Option<(String, Module)> {
    match mod_stmt {
        Stmt::Module(module_ident) => {
            let mut module = Module::new(&module_ident.ident);
            let mut extracted_functions = HashMap::new();
            let mut extracted_modules = HashMap::new();
            for stmt in &module_ident.body.stmt {
                if let Some((ident, module)) = extract_module(stmt) {
                    extracted_modules.insert(ident, module);
                }
                if let Some((ident, func)) = extract_func(stmt) {
                   extracted_functions.insert(ident, func);
                }
            }
            for (ident, m) in extracted_modules {
                module.insert(&ident, Value::Module(m));
            }
            for (ident, func) in extracted_functions {
                module.insert(&ident, Value::Func(func));
            }
            Some((module_ident.ident.clone(), module))
        }
        _ => None
    }
}

pub fn extract_import(import_stmt: &Stmt, ) -> Option<(String, Arc<RwLock<Value>>)> {
    match import_stmt {
        Stmt::Import(Import{ inline_access }) => {
            let mut idents = vec![];
            let mut curr_expr = Some(inline_access.clone());
            while let Some(expr) = curr_expr {
                match *expr {
                    Expr::InlineAccess(InlineAccess { ident, next }) => {
                        idents.push(ident);
                        curr_expr = next;
                    }
                    Expr::Ident(ident) => {
                        idents.push(ident);
                        curr_expr = None;
                    }
                    _ => {
                        curr_expr = None;
                    }
                }
            }
            let mut curr_stmt = GLOBAL_ENV.read().unwrap().global_stmts.get(&idents[1]).unwrap().clone();
            for i in 2..idents.len() {
                let module = if let Value::Module(module) = curr_stmt.read().unwrap().clone() {
                    module
                } else { panic!("Module not found!") };
                curr_stmt = module.get(&idents[i]).unwrap().clone();
            }
            return Some((idents[idents.len()-1].clone(), curr_stmt))
        }
        _ => {}
    }
    None
}

macro_rules! eval_primitive_expr {
        ($rhs: expr, $lhs: expr, $v: expr) => {
            if let Value::Void = $rhs {
                $rhs = $v;
            } else {
                $lhs = $v;
            }
        };
    }

macro_rules! eval_binary_expr {
        ($rhs: expr, $lhs: expr, $op: tt) => {
            {
                $rhs = $lhs $op $rhs;
                $lhs = Value::Void
            }
        };
    }

macro_rules! eval_cond_expr {
        ($rhs: expr, $lhs: expr, $l: expr, $r: expr, $cond_type: expr) => {
            if let Value::Void = $rhs {
                $rhs = Value::Cond($cond_type, $l, $r);
            } else {
                $lhs = Value::Cond($cond_type, $l, $r);
            }
        };
    }


#[inline]
pub fn eval_expr(expr: Expr, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    let mut expr_stack = vec![expr.clone()];
    let mut curr_exprs = vec![expr];
    while let Some(expr) = curr_exprs.pop() {
        //println!("CURR_EXPR: {:?}", curr_exprs);
        match expr {
            Expr::Add(l, r)
            | Expr::Sub(l, r)
            | Expr::Mul(l, r)
            | Expr::Div(l, r)
            | Expr::Xor(l, r)
            | Expr::Mod(l, r) => {
                expr_stack.push(*l.clone());
                expr_stack.push(*r.clone());
                curr_exprs.push(*l.clone());
                curr_exprs.push(*r.clone());
            }
            Expr::Neg(r) | Expr::Not(r) => {
                expr_stack.push(*r.clone());
                curr_exprs.push(*r.clone());
            }
            _ => {}
        }
    }

    //println!("EXPR_STACK: {expr_stack:#?}");


    let mut value = Value::Void;
    let mut rhs = Value::Void;
    let mut lhs = Value::Void;
    while let Some(expr) = expr_stack.pop() {
        //println!("LHS: {lhs} RHS: {rhs}");
        match expr {
            Expr::Integer(v) => eval_primitive_expr!(rhs, lhs, Value::Int(v)),
            Expr::Float(v) => eval_primitive_expr!(rhs, lhs, Value::Float(v)),
            Expr::Bool(v) => eval_primitive_expr!(rhs, lhs, Value::Bool(v)),
            Expr::StringLit(v) => eval_primitive_expr!(rhs, lhs, Value::String(v)),
            Expr::Add(_, _) => eval_binary_expr!(rhs, lhs, +),
            Expr::Sub(_, _) => eval_binary_expr!(rhs, lhs, -),
            Expr::Mul(_, _) => eval_binary_expr!(rhs, lhs, *),
            Expr::Div(_, _) => eval_binary_expr!(rhs, lhs, /),
            Expr::Eq(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Eq),
            Expr::NotEq(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Ne),
            Expr::Gt(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Gt),
            Expr::Lt(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Lt),
            Expr::Ge(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Ge),
            Expr::Le(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Le),
            Expr::Or(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::Or),
            Expr::And(l, r) => eval_cond_expr!(rhs, lhs, l, r, CondType::And),
            Expr::Not(_) => {
                if let Value::Void = rhs {
                    lhs = lhs.not()
                } else {
                    rhs = rhs.not()
                }
            }
            Expr::Neg(_) => {
                if let Value::Void = rhs {
                    lhs = lhs.neg()
                } else {
                    rhs = rhs.neg()
                }
            }
            Expr::Xor(_, _) => eval_binary_expr!(rhs, lhs, ^),
            Expr::Mod(_, _) => eval_binary_expr!(rhs, lhs, %),
            Expr::Call(call_expr) => eval_primitive_expr!(rhs, lhs, call_func(call_expr, env.clone())),
            Expr::Ident(ident) => {
                let var_value = {
                    env.try_read().unwrap().variables.get(&ident).unwrap().clone()
                };
                let resolved_value = var_value.try_read().unwrap().clone();

                if let Value::Void = rhs {
                    rhs = resolved_value;
                } else {
                    lhs = resolved_value;
                }
            }
            Expr::Func(f_ptr) => eval_primitive_expr!(rhs, lhs, Value::CallFunc(CallExpr::new(
                f_ptr.ident.clone(),
                f_ptr.args.unwrap()
            ))),
            Expr::AnonFunc(a_func) => {
                let mut cache = ANON_FUNC_CACHE.write().unwrap();
                let cached_func = match cache.get(&a_func) {
                    None => {
                        //println!("ANON: {a_func:?}");
                        let ident = Uuid::new_v4().to_string();
                        let a_func_clone = a_func.clone();

                        let args: Vec<_> = a_func.args.iter().map(|(arg_ident, expr)| {
                            let value = eval_expr(expr.clone(), Arc::clone(&env));
                            (arg_ident.clone(), value.into_type().to_string())
                        }).collect();


                        let call_args: Vec<_> = a_func.args.into_iter().map(|(_, expr)| expr).collect();


                        let func = Function::new(
                            PrivacyType::Private,
                            HashMap::new(),
                            Arc::clone(&env),
                            ident.clone(),
                            args,
                            a_func.rty,
                            a_func.stmt.unwrap().stmt,
                        );

                        env.write().unwrap().variables.insert(
                            ident.clone(),
                            Arc::new(RwLock::new(Value::Func(func.clone()))),
                        );

                        eval_primitive_expr!(rhs, lhs, Value::CallFunc(CallExpr::new(ident, call_args)));
                        Some((a_func_clone, func))
                    }
                    Some(func) => {
                        let call_args: Vec<_> = a_func.args.into_iter().map(|(_, expr)| expr).collect();

                        env.write().unwrap().variables.insert(
                            func.get_ident().to_string(),
                            Arc::new(RwLock::new(Value::Func(func.clone()))),
                        );

                        // Подготавливаем значение для вызова
                        eval_primitive_expr!(rhs, lhs, Value::CallFunc(CallExpr::new(func.get_ident().to_string(), call_args)));
                        None
                    }
                };
                if let Some((a_func, func)) = cached_func {
                    cache.insert(a_func, func);
                }
                drop(cache)

            }
            Expr::Range((start, end)) => eval_primitive_expr!(rhs, lhs, Value::Range(start, end)),
            Expr::Counter((ident, (start, end))) => {
                env.try_write()
                    .unwrap()
                    .variables
                    .insert(ident.clone(), Arc::new(RwLock::new(Value::Int(start))));
                eval_primitive_expr!(rhs, lhs, Value::Counter(ident, start, end))
            }
            Expr::Ref(expr) => match *expr {
                Expr::Ident(ident) => {
                    let var_value = env.try_read().unwrap().variables.get(&ident).unwrap().clone();
                    eval_primitive_expr!(rhs, lhs, Value::RefValue(var_value));
                }
                _ => {
                    let evaluated = eval_expr(*expr, env.clone());
                    eval_primitive_expr!(rhs, lhs, Value::RefValue(Arc::new(RwLock::new(evaluated))));
                }
            },


            _ => {}
        }
    }

    value = rhs;
    //println!("FINAL: {value:?}");
    value
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
                let l_env = Arc::new(RwLock::new(LocalEnvironment::new()));
                let args = func.get_args();
                let parsed_args_len = parsed_args.len();
                let l_env_clone = l_env.clone();
                let mut env_lock = l_env_clone.write().expect("Failed to acquire write lock");

                for (i, (ident, ty)) in args.into_iter().enumerate() {
                    // Early exit if index is out of bounds
                    if i >= parsed_args_len {
                        panic!("Argument index out of bounds");
                    }

                    let parsed_value = &parsed_args[i];

                    // Check if the type matches
                    if Value::Type(ty.to_string()) == parsed_value.clone().into_type() {
                        match parsed_value {
                            Value::RefValue(r) => {
                                env_lock.variables.insert(ident.clone(), r.clone());
                            },
                            _ => {
                                env_lock.variables.insert(ident.clone(), Arc::new(RwLock::new(parsed_value.clone())));
                            },
                        }
                    } else {
                        panic!("Expected other type");
                    }
                }
                drop(env_lock);
                func.set_env(l_env);
                func.run()
            }

            _ => panic!("Expected func"),
        };
    }};
}

#[inline]
pub fn call_func(call_expr: CallExpr, env: Arc<RwLock<LocalEnvironment>>) -> Value {
    //println!("{call_expr:?} {env:?}");
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
