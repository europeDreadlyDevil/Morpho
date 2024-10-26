use std::collections::HashMap;
use crate::ast::{Expr, Stmt, VarIdent};
use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    function_fields: HashMap<String, Value>,
    environment: LocalEnvironment,
    ident: String,
    args: Vec<(String, String)>,
    pub rty: String,
    body: Vec<Stmt>,
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
    pub(crate) fn run(&mut self) {
        for stmt in self.body.clone() {
            match stmt {
                Stmt::Expr(expr) => match *expr {
                    Expr::Call(call_expr) => {
                        call_func(call_expr, &mut self.environment)
                    }
                    _ => panic!("Unhandled expression"),
                },
                Stmt::VarIdent(VarIdent { ident, expr }) => {
                    let value = eval_expr(expr, &mut self.environment);
                    self.environment.variables.insert(ident, value);
                }
                _ => panic!("Unhandled statement"),
            }
        }
    }
    
    pub(crate) fn get_ident(&self) -> &str {
        &self.ident
    }

    pub(crate) fn get_args(&self) -> &Vec<(String,String)> {
        &self.args
    }

    pub(crate) fn set_env(&mut self, env: LocalEnvironment) {
        self.environment = env
    }
}