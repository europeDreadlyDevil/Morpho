use crate::program::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Environment {
    pub(crate) global_stmts: HashMap<String, Arc<RwLock<Value>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            global_stmts: HashMap::new(),
        }
    }
    pub fn insert_stmt(&mut self, ident: &str, stmt: Value) {
        self.global_stmts
            .insert(ident.into(), Arc::new(RwLock::new(stmt)));
    }
}

#[derive(Clone, Debug)]
pub struct LocalEnvironment {
    pub(crate) variables: HashMap<String, Arc<RwLock<Value>>>,
}

impl LocalEnvironment {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}
