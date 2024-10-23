use crate::ast::{Expr, FuncBody, FuncIdent, Prog, Stmt, VarIdent};
use std::collections::HashMap;
use std::fmt::{write, Display, Formatter};
use anyhow::{Error, Result};
use std::sync::{Arc, Mutex};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub mod ast;

pub fn eval_program(prog: Prog) -> Result<()> {
    let prog = Program::new(prog)?;
    prog.run();
    Ok(())
}

struct Program {
    env: Environment,
}

impl Program {
    pub(crate) fn run(mut self) {
        let mut main_func = self.env.main_function.clone();
        main_func.run(&mut self.env);
    }
}

impl Program {
    pub fn new(prog: Prog) -> Result<Self> {
        let mut global_stmts: HashMap<String, Value> = HashMap::new();
        
        global_stmts.insert("print".into(), Value::FuncPtr(print));
        
        let mut extracted_functions: HashMap<String, Function> = HashMap::new();

        for stmt in prog.0 {
            if let Some((ident, func)) = extract_func(stmt) {
                extracted_functions.insert(ident, func);
            }
        }

        for (ident, func) in extracted_functions {
            global_stmts.insert(ident, Value::Func(func));
        }
        
        
        if let Some(main_func) = if let Some(main_func) = global_stmts.get("main") {
            if let crate::Value::Func(main_func) = main_func {
                Some(main_func.clone())
            } else { None }
        } else { None } {
            return Ok(Self {
                env: Environment::new(
                    main_func,
                    global_stmts,
                ),
            })
        }
        
        Err(Error::msg("main func not found"))
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

struct Environment {
    global_stmts: HashMap<String, Value>,
    main_function: Function,
}

impl Environment {
    pub fn new(main_function: Function, global_stmts: HashMap<String, Value>) -> Self {
        Self {
            global_stmts,
            main_function,
        }
    }
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
enum Value {
    String(String),
    Int(i64),
    FuncPtr(fn(Vec<Value>)),
    Func(Function),
    Type(String),
    None,
}

impl Value {
    pub(crate) fn into_type(self) -> Value {
        match self {
            Value::String(_) => Value::Type("string".into()),
            Value::Int(_) => Value::Type("int".into()),
            Value::FuncPtr(func) => Value::Type(format!("{:?}", func)),
            Value::Func(Function{rty, ..}) => Value::Type(rty),
            Value::Type(ty) => Value::Type(ty),
            Value::None => Value::Type("none".into())
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
            Value::Type(ty) => write!(f, "{}", ty)
        }
    }
}

#[derive(Clone, PartialEq)]
struct Function {
    function_fields: HashMap<String, Value>,
    environment: LocalEnvironment,
    ident: String,
    args: Vec<(String, String)>,
    rty: String,
    body: Vec<Stmt>,
}

impl Function {
    pub(crate) fn run(&mut self, g_env: &mut Environment) {
        for stmt in self.body.clone() {
            match stmt {
                Stmt::Expr(expr) => match *expr { 
                    Expr::Call(call_expr) => {
                        let ident = call_expr.get_name();
                        let args = call_expr.get_args();
                        let mut parsed_args: Vec<Value> = vec![];
                        for arg in args {
                            parsed_args.push(eval_expr(arg, &self.environment));
                        }
                        match g_env.global_stmts.clone().get_mut(&ident).unwrap() {
                            Value::FuncPtr(func) => func(parsed_args),
                            Value::Func(ref mut func) => {
                                let mut l_env = LocalEnvironment::new();
                                for i in 0..func.args.len() {
                                    let (ident, ty) = func.args[i].clone();
                                    if Value::Type(ty) == parsed_args[i].clone().into_type() {
                                        l_env.variables.insert(ident, parsed_args[i].clone());
                                    }
                                    else { panic!("Expected other type") }
                                }
                                func.environment = l_env;
                                func.run(g_env)
                            }
                            _ => {}
                        }
                        
                    },
                    _ => panic!("Unhandled expression")
                },
                Stmt::VarIdent(VarIdent{ ident, expr }) => {
                    let value = eval_expr(expr, &self.environment);
                    self.environment.variables.insert(ident, value);
                }
                _ => panic!("Unhandled statement")
            }
        }
    }
}

fn eval_expr(expr: Expr, env: &LocalEnvironment) -> Value {
    match expr {
        Expr::Integer(i) => Value::Int(i),
        Expr::StringLit(s) => Value::String(s),
        Expr::Ident(ident) => env.variables.get(&ident).unwrap().clone(),
        _ => Value::None
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


fn print(items: Vec<Value>) {
    for i in 0..items.len()-1 {
        print!("{} ", items[i]);
    }
    println!("{}", items[items.len()-1])
}