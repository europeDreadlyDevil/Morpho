use crate::ast::{Expr, Stmt, VarAssign, VarIdent};
use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Function {
    function_fields: HashMap<String, Value>,
    environment: Arc<RwLock<LocalEnvironment>>,
    ident: String,
    args: Vec<(String, String)>,
    pub rty: String,
    body: Vec<Stmt>,
}
impl Function {
    pub fn new(
        function_fields: HashMap<String, Value>,
        environment: Arc<RwLock<LocalEnvironment>>,
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
    pub(crate) fn run(&mut self) {
        for stmt in self.body.clone() {
            //println!("RUN_ENV: {:?}", self.environment);
            match stmt {
                Stmt::Expr(expr) => match *expr {
                    Expr::Call(call_expr) => call_func(call_expr, self.environment.clone()),
                    _ => panic!("Unhandled expression"),
                },
                Stmt::VarIdent(VarIdent { ident, expr }) => {
                    let value = eval_expr(expr, self.environment.clone());
                    (*self.environment.try_write().unwrap())
                        .variables
                        .insert(ident, Arc::new(RwLock::new(value)));
                }
                Stmt::VarAssign(VarAssign { ident, expr }) => {
                    let value = eval_expr(expr, self.environment.clone());
                    if let Value::RefValue(r) = self
                        .environment
                        .try_read()
                        .unwrap()
                        .variables
                        .get(&ident)
                        .unwrap()
                        .try_read()
                        .unwrap()
                        .clone()
                    {
                        *r.try_write().unwrap() = value;
                        return;
                    }
                    *self.environment.try_read().unwrap().variables.get(&ident).unwrap().try_write().unwrap() = value;
                }
                _ => panic!("Unhandled statement"),
            }
        }
    }

    pub(crate) fn get_ident(&self) -> &str {
        &self.ident
    }

    pub(crate) fn get_args(&self) -> &Vec<(String, String)> {
        &self.args
    }

    pub(crate) fn set_env(&mut self, env: Arc<RwLock<LocalEnvironment>>) {
        self.environment = env
    }
}
