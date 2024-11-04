use std::collections::HashMap;
use crate::program::environment::Environment;
use lalrpop_util::lalrpop_mod;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use crate::ast::AnonymousFunc;
use crate::program::function::Function;

lalrpop_mod!(pub parser);

lazy_static! {
    pub static ref GLOBAL_ENV: Arc<RwLock<Environment>> = Arc::new(RwLock::new(Environment::new()));
    pub static ref ANON_FUNC_CACHE: Arc<RwLock<HashMap<AnonymousFunc, Function>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub mod ast;
pub mod program;
