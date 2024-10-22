use crate::parser::ProgParser;
use anyhow::Result;
use lalrpop_util::lalrpop_mod;
use func_lang::eval_program;

lalrpop_mod!(pub parser);

fn main() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { print("Hello, world!"); }"#)?;
    //println!("{ast:#?}");
    eval_program(ast);
    Ok(())
}
