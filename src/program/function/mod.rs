use crate::ast::{Expr, InlineAccess, PrivacyType, Stmt, VarAssign, VarIdent};
use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::{call_func, eval_expr};
use crate::program::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::GLOBAL_ENV;

#[derive(Clone, Debug)]
pub struct Function {
    privacy: PrivacyType,
    function_fields: HashMap<String, Value>,
    environment: Arc<RwLock<LocalEnvironment>>,
    ident: String,
    args: Vec<(String, String)>,
    pub rty: String,
    body: Vec<Stmt>,
}
impl Function {
    pub fn new(
        privacy: PrivacyType,
        function_fields: HashMap<String, Value>,
        environment: Arc<RwLock<LocalEnvironment>>,
        ident: String,
        args: Vec<(String, String)>,
        rty: String,
        body: Vec<Stmt>,
    ) -> Self {
        Self {
            privacy,
            function_fields,
            environment,
            ident,
            args,
            rty,
            body,
        }
    }
    pub(crate) fn run(mut self) -> Value {
        for stmt in self.body {
            match stmt {
                Stmt::Expr(expr) => match *expr {
                    Expr::Call(call_expr) => {
                        call_func(call_expr, self.environment.clone());
                    }
                    Expr::InlineAccess(InlineAccess{ ident, next }) => {
                        let mut idents = vec![ident];
                        let mut curr_expr = next;
                        while let Some(expr) = curr_expr.clone() {
                            match *expr {
                                Expr::InlineAccess(InlineAccess{ident, next}) => {
                                    idents.push(ident);
                                    curr_expr = next;
                                }
                                Expr::Call(call_expr) => {
                                    let mut module_value = GLOBAL_ENV.read().unwrap().global_stmts.get(&idents[0]).unwrap().clone();
                                    let mut deleted_func = None;
                                    let mut inserted_ident = call_expr.get_name();
                                    //println!("MODULE:{module_value:?}");
                                    for i in 1..idents.len() {
                                        let module = if let Value::Module(val) = module_value.read().unwrap().clone() {
                                            val
                                        } else { panic!("Module not found") };
                                        module_value = module.get(&idents[i]).unwrap().clone();
                                    }
                                    if let Value::Module(module) = module_value.read().unwrap().clone() {
                                        if let Some(func_value) = module.get(&inserted_ident) {
                                            if let Value::Func(func) = func_value.clone().read().unwrap().clone() {
                                                deleted_func = GLOBAL_ENV.write().unwrap().global_stmts.insert(inserted_ident.clone(), Arc::new(RwLock::new(Value::Func(func))));
                                            }
                                        }
                                    }

                                    //println!("SELF_ENV: {:?}", self.environment);
                                    call_func(call_expr, self.environment.clone());
                                    if let Some(value) = deleted_func {
                                        GLOBAL_ENV.write().unwrap().global_stmts.insert(inserted_ident, value);
                                    } else {
                                        GLOBAL_ENV.write().unwrap().global_stmts.remove(&inserted_ident);
                                    }
                                    curr_expr = None;
                                }
                                _ => panic!("Unhandled expression"),
                            }
                        }
                    }
                    _ => panic!("Unhandled expression"),
                },
                Stmt::VarIdent(VarIdent { ident, expr }) => {
                    let value = eval_expr(expr.clone(), self.environment.clone());
                    let value = if let Value::Cond(ty, l, r) = value {
                        Value::Bool(ty.eval_cond(l, r, self.environment.clone()))
                    } else {
                        value
                    };
                    (*self.environment.try_write().unwrap())
                        .variables
                        .insert(ident, Arc::new(RwLock::new(value)));
                }
                Stmt::VarAssign(VarAssign { ident, expr }) => {
                    let value = eval_expr(expr, self.environment.clone());
                    let value = if let Value::Cond(ty, l, r) = value {
                        Value::Bool(ty.eval_cond(l, r, self.environment.clone()))
                    } else {
                        value
                    };
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
                        continue;
                    }
                    *self
                        .environment
                        .try_read()
                        .unwrap()
                        .variables
                        .get(&ident)
                        .unwrap()
                        .try_write()
                        .unwrap() = value;
                }
                Stmt::ReturnValue(expr) => {
                    let value = eval_expr(*expr, self.environment.clone());
                    if value.clone().into_type() == Value::Type(self.rty.clone()) {
                        return value;
                    }
                    panic!("Excepted other returning type");
                }
                _ => panic!("Unhandled statement"),
            };
        }
        Value::Void
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
