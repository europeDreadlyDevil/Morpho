use std::collections::HashMap;
use crate::program::value::Value;

#[derive(Debug)]
pub struct Environment {
    pub(crate) global_stmts: HashMap<String, Value>,
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
pub struct LocalEnvironment {
    pub(crate) variables: HashMap<String, Value>,
}

impl LocalEnvironment {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}