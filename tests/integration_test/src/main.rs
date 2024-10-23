use func_lang::eval_program;
use func_lang::parser::ProgParser;
use anyhow::Result;

fn main() -> Result<()> {
    print_hello_world()?;
    print_hello_world_with_var()?;
    print_int_with_var()?;
    print_int_with_two_var()?;
    
    Ok(())
}

fn print_hello_world() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { print("Hello, world!"); }"#)?;
    eval_program(ast);
    Ok(())
}

fn print_hello_world_with_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = "Hello, world!"; print(x); }"#)?;
    eval_program(ast);
    Ok(())
}

fn print_int_with_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; print(x); }"#)?;
    eval_program(ast);
    Ok(())
}

fn print_int_with_two_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; let y = x; print(y); }"#)?;
    eval_program(ast);
    Ok(())
}