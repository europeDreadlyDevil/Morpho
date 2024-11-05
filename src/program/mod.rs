pub mod environment;
pub mod evaluating_functions;
pub mod function;
pub mod primitive_functions;
pub mod value;

use crate::ast::Prog;
use crate::program::evaluating_functions::extract_func;
use crate::program::function::Function;
use crate::program::primitive_functions::{for_func, if_func, input_func, print_func, while_func};
use crate::program::value::Value;
use crate::GLOBAL_ENV;
use anyhow::{Error, Result};
use std::collections::HashMap;

struct Program {
    main_function: Function,
}

impl Program {
    pub fn new(prog: Prog) -> Result<Self> {
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("print", Value::FuncPtr(print_func));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("if".into(), Value::FuncPtr(if_func));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("for".into(), Value::FuncPtr(for_func));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("while".into(), Value::FuncPtr(while_func));
        GLOBAL_ENV
            .try_write()
            .unwrap()
            .insert_stmt("input".into(), Value::FuncPtr(input_func));


        let mut extracted_functions: HashMap<String, Function> = HashMap::new();

        for stmt in prog.0 {
            if let Some((ident, func)) = extract_func(stmt) {
                extracted_functions.insert(ident, func);
            }
        }

        for (ident, func) in extracted_functions {
            GLOBAL_ENV
                .try_write()
                .unwrap()
                .insert_stmt(&ident, Value::Func(func));
        }

        if let Some(main_func) = GLOBAL_ENV.try_read().unwrap().global_stmts.get("main") {
            if let Value::Func(main_func) = main_func.clone().try_read().unwrap().clone() {
                return Ok(Self {
                    main_function: main_func.clone(),
                });
            }
        }
        Err(Error::msg("Main function not found"))
    }
    pub fn run(mut self) -> Result<()> {
        self.main_function.run();
        Ok(())
    }
}
