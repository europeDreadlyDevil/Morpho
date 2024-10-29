use anyhow::Result;
use clap::Parser;
use func_lang::parser::ProgParser;
use func_lang::program::evaluating_functions::eval_program;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command()]
struct CLI {
    path_to_file: PathBuf,
}

fn main() -> Result<()> {
    let path = CLI::parse().path_to_file;
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    let ast = ProgParser::new().parse(buf.leak())?;
    eval_program(ast)
}
