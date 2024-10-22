use crate::ast::{Expr, FuncBody, Prog, Stmt};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub mod ast;

pub fn eval_program(prog: Prog) {
    let prog = Program::new(prog);
    prog.run()
}

struct Program {
    env: Environment,
}

impl Program {
    pub(crate) fn run(self) {
        self.env.main_function.run(&self.env);
    }
}

impl Program {
    pub fn new(prog: Prog) -> Self {
        let mut global_stmts: HashMap<String, Value> = HashMap::new();
        
        global_stmts.insert("print".into(), Value::Func(print));
        
        let mut extracted_functions: HashMap<String, Function> = HashMap::new();

        for stmt in prog.0 {
            if let Some((ident, func)) = extract_func(stmt) {
                extracted_functions.insert(ident, func);
            }
        }
        
        Self {
            env: Environment::new(
                extracted_functions.get("main").unwrap().clone(),
                global_stmts,
            ),
        }
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

#[derive(Clone)]
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

#[derive(Clone)]
enum Value {
    String(String),
    Int(i64),
    Func(fn(Vec<Value>)),
    None
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Func(func) => write!(f, "{:?}", func),
            Value::None => write!(f, "None")
        }
    }
}

#[derive(Clone)]
struct Function {
    function_fields: HashMap<String, Value>,
    environment: LocalEnvironment,
    ident: String,
    args: Vec<(String, String)>,
    rty: String,
    body: Vec<Stmt>,
}

impl Function {
    pub(crate) fn run(&self, g_env: &Environment) {
        for stmt in self.body.clone() {
            match stmt {
                Stmt::Expr(expr) => match *expr { 
                    Expr::Call(call_expr) => {
                        let ident = call_expr.get_name();
                        let args = call_expr.get_args();
                        let mut parsed_args: Vec<Value> = vec![];
                        for arg in args {
                            parsed_args.push(eval_expr(arg));
                        }
                        match g_env.global_stmts.get(&ident).unwrap() {
                            Value::String(_) => {}
                            Value::Int(_) => {}
                            Value::Func(func) => func(parsed_args),
                            Value::None => {}
                        }
                        
                    },
                    _ => panic!("Unhandled expression")
                },
                _ => panic!("Unhandled statement")
            }
        }
    }
}

fn eval_expr(expr: Expr) -> Value {
    match expr {
        Expr::Integer(i) => Value::Int(i),
        Expr::StringLit(s) => Value::String(s),
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
    print!("{}", items[items.len()-1])
}