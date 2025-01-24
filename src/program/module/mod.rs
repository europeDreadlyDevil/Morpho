use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::program::value::Value;

#[derive(Clone, Debug)]
pub struct Module {
    ident: String,
    stmts: HashMap<String,Arc<RwLock<Value>>>
}

impl Module {
    pub fn new(ident: &str) -> Self {
        Self { ident: ident.to_string(), stmts: Default::default() }
    }
    pub fn insert(&mut self, ident: &str, stmt: Value) {
        self.stmts.insert(ident.into(), Arc::new(RwLock::new(stmt)));
    }
    pub fn get(&self, ident: &str) -> Option<&Arc<RwLock<Value>>> {
        self.stmts.get(ident)
    }
}