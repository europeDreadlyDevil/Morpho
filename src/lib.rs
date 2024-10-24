#![feature(tuple_trait)]

use crate::ast::{CallExpr, Expr, FuncBody, Prog, Stmt, VarIdent};
use anyhow::__private::kind::TraitKind;
use anyhow::{Error, Result};
use lalrpop_util::lalrpop_mod;
use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;
use std::env::Args;
use std::fmt::{Display, Formatter};
use std::marker::Tuple;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::{Arc, Mutex, RwLock, TryLockResult};

lalrpop_mod!(pub parser);

lazy_static! {
    static ref GLOBAL_ENV: Arc<RwLock<Environment>> = Arc::new(RwLock::new(Environment::new()));
}

pub mod ast;

pub fn eval_program(prog: Prog) -> Result<()> {
    let prog = Program::new(prog)?;
    prog.run()?;
    Ok(())
}

struct Program {
    main_function: Function,
}

impl Program {
    pub(crate) fn run(mut self) -> Result<()> {
        self.main_function.run();
        Ok(())
    }
}

impl Program {
    pub fn new(prog: Prog) -> Result<Self> {
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("print", Value::FuncPtr(print));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("if".into(), Value::FuncPtr(if_func));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("for".into(), Value::FuncPtr(for_func));
        let mut extracted_functions: HashMap<String, Function> = HashMap::new();

        for stmt in prog.0 {
            if let Some((ident, func)) = extract_func(stmt) {
                extracted_functions.insert(ident, func);
            }
        }

        for (ident, func) in extracted_functions {
            GLOBAL_ENV
                .try_write()
                .unwrap()
                .insert_stmt(&ident, Value::Func(func));
        }

        if let Some(main_func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get("main") {
            if let Value::Func(main_func) = main_func {
                return Ok(Self {
                    main_function: main_func.clone(),
                });
            }
        }
        Err(Error::msg("Main function not found"))
    }
}

fn extract_func(func_stmt: Stmt) -> Option<(String, Function)> {
    match func_stmt {
        Stmt::FuncIdent(f_ident) => {
            if let Some(FuncBody { stmt }) = f_ident.stmt {
                return Some((
                    f_ident.ident.clone(),
                    Function::new(
                        HashMap::new(),
                        LocalEnvironment::new(),
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

#[derive(Debug)]
struct Environment {
    global_stmts: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            global_stmts: HashMap::new(),
        }
    }
    pub fn insert_stmt(&mut self, ident: &str, stmt: Value) {
        self.global_stmts.insert(ident.into(), stmt);
    }
}

#[derive(Clone, PartialEq, Debug)]
struct LocalEnvironment {
    variables: HashMap<String, Value>,
}

impl LocalEnvironment {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    String(String),
    Int(i64),
    Bool(bool),
    FuncPtr(fn(Vec<Value>, &LocalEnvironment) -> Value),
    Func(Function),
    CallFunc(CallExpr),
    Type(String),
    Range(i64, i64),
    None,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => Value::None
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            _ => Value::None
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => Value::None
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => Value::None
        }
    }
}

impl Value {
    pub(crate) fn into_type(self) -> Value {
        match self {
            Value::String(_) => Value::Type("string".into()),
            Value::Int(_) => Value::Type("int".into()),
            Value::FuncPtr(func) => Value::Type(format!("{:?}", func)),
            Value::Func(Function { rty, .. }) => Value::Type(rty),
            Value::Type(ty) => Value::Type(ty),
            Value::None => Value::Type("none".into()),
            Value::Bool(_) => Value::Type("bool".into()),
            Value::CallFunc { .. } => Value::Type("func".into()),
            Value::Range(s, e) => Value::Type(format!("range<{}, {}>", s, e))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Func(func) => write!(f, "{}", func.ident),
            Value::None => write!(f, "None"),
            Value::FuncPtr(func_ptr) => write!(f, "{:?}", func_ptr),
            Value::Type(ty) => write!(f, "{}", ty),
            Value::Bool(b) => write!(f, "{}", b),
            Value::CallFunc (call) => write!(f, "{}", call.get_name()),
            Value::Range(s, e) => write!(f, "range<{}, {}>", s, e)
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Function {
    function_fields: HashMap<String, Value>,
    environment: LocalEnvironment,
    ident: String,
    args: Vec<(String, String)>,
    rty: String,
    body: Vec<Stmt>,
}

impl Function {
    pub(crate) fn run(&mut self) {
        for stmt in self.body.clone() {
            match stmt {
                Stmt::Expr(expr) => match *expr {
                    Expr::Call(call_expr) => {
                        call_func(call_expr, &self.environment)
                    }
                    _ => panic!("Unhandled expression"),
                },
                Stmt::VarIdent(VarIdent { ident, expr }) => {
                    let value = eval_expr(expr, &self.environment);
                    self.environment.variables.insert(ident, value);
                }
                _ => panic!("Unhandled statement"),
            }
        }
    }
}

fn eval_expr(expr: Expr, env: &LocalEnvironment) -> Value {
    match expr {
        Expr::Integer(i) => Value::Int(i),
        Expr::StringLit(s) => Value::String(s),
        Expr::Ident(ident) => env.variables.get(&ident).unwrap().clone(),
        Expr::Bool(b) => Value::Bool(b),
        Expr::Call(call_expr) => { call_func(call_expr, env); Value::None },
        Expr::Func(f_ptr) => Value::CallFunc (CallExpr::new(f_ptr.ident.clone(), f_ptr.args.unwrap())),
        Expr::Eq(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            Value::Bool(l == r)
        },
        Expr::NotEq(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            Value::Bool(l != r)
        },
        Expr::Add(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            l + r
        },
        Expr::Sub(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            l - r
        },
        Expr::Mul(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            l * r
        },
        Expr::Div(l, r) => {
            let l = eval_expr(*l, env);
            let r = eval_expr(*r, env);
            l / r
        },
        Expr::Range((start, end)) => Value::Range(start, end),
        _ => Value::None,
    }
}

fn call_func(call_expr: CallExpr, env: &LocalEnvironment) {
    let ident = call_expr.get_name();
    let args = call_expr.get_args();
    let mut parsed_args: Vec<Value> = vec![];
    for arg in args {
        parsed_args.push(eval_expr(arg, env));
    }
    match GLOBAL_ENV
        .try_read()
        .unwrap()
        .global_stmts
        .clone()
        .get(&ident)
        .unwrap()
        .clone()
    {
        Value::FuncPtr(func) => {
            func(parsed_args, env);
        }
        Value::Func(mut func) => {
            let mut l_env = LocalEnvironment::new();
            for i in 0..func.args.len() {
                let (ident, ty) = func.args[i].clone();
                if Value::Type(ty) == parsed_args[i].clone().into_type() {
                    l_env.variables.insert(ident, parsed_args[i].clone());
                } else {
                    panic!("Expected other type")
                }
            }
            func.environment = l_env;
            func.run()
        }
        _ => {}
    }
}

impl Function {
    pub fn new(
        function_fields: HashMap<String, Value>,
        environment: LocalEnvironment,
        ident: String,
        args: Vec<(String, String)>,
        rty: String,
        body: Vec<Stmt>,
    ) -> Self {
        Self {
            function_fields,
            environment,
            ident,
            args,
            rty,
            body,
        }
    }
}

fn print(args: Vec<Value>, env: &LocalEnvironment) -> Value {
    for i in 0..args.len() - 1 {
        print!("{} ", args[i]);
    }
    println!("{}", args[args.len() - 1]);
    Value::None
}

fn if_func(args: Vec<Value>, env: &LocalEnvironment) -> Value {
    //println!("If block: {:?}", args);
    if let Value::Bool(true) = &args[0] {
        if let Value::CallFunc(call_expr) = args[1].clone() {
            if let Some(func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get(&call_expr.get_name()) {
                if let Value::FuncPtr(func) = func {
                    let mut parsed_args = vec![];
                    
                    for arg in call_expr.get_args() {
                        parsed_args.push(eval_expr(arg, &env))
                    }
                    return func(parsed_args, env)
                }
            }
        }
    } else if let Value::Bool(false) = &args[0] {
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
            call_func(call_expr, &env);
        }
    }
    Value::None
}

fn for_func(args: Vec<Value>, env: &LocalEnvironment) -> Value {
    if let Value::Range(start, end) = args[0] {
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
            }

            for _ in start..end {
                call_func(call_expr.clone(), &env);
            }
        }
    }
    Value::None
}
