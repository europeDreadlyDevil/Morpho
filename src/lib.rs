use lalrpop_util::lalrpop_mod;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use crate::program::environment::Environment;

lalrpop_mod!(pub parser);

lazy_static! {
    pub static ref GLOBAL_ENV: Arc<RwLock<Environment>> = Arc::new(RwLock::new(Environment::new()));
}

pub mod ast;
pub mod program;

