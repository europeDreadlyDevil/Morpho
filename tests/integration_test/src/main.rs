use anyhow::Result;
use func_lang::eval_program;
use func_lang::parser::ProgParser;
use tracing_log::log::{log, Level};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    print_hello_world()?;
    print_hello_world_with_var()?;
    print_int_with_var()?;
    print_int_with_two_var()?;
    create_new_func()?;
    call_func_with_args()?;
    eval_expr_in_func()?;
    eval_expr_in_func_with_vars()?;
    condition_block()?;
    condition_block_recursion()?;
    for_block()?;
    Ok(())
}

fn print_hello_world() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { print("Hello, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_hello_world_with_var() -> Result<()> {
    let ast =
        ProgParser::new().parse(r#"func main = () { let x = "Hello, world!"; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_int_with_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_int_with_two_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; let y = x; print(y); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn create_new_func() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { hello_world(); bye_world(); } func hello_world = () { print("Hello, world!"); } func bye_world = () { print("Bye, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn call_func_with_args() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { say("I'm John"); } func say = (x: string) { print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn eval_expr_in_func() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { print(20 - 50); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn eval_expr_in_func_with_vars() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let x1 = 60; let x2 = 30; let y = x1 * x2; print(x1, x2, y, x1-x2); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn condition_block() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { if(true, print("TRUE!")); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}
fn condition_block_recursion() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { foo(10); } func foo = (a: int) { print(a); if(a == 0, $print|"end"|, $foo|a - 1|); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn for_block() -> Result<()> {
    let ast = ProgParser::new()
        .parse(r#"func main = () { for(0..10000, $for|0..10000, $say|"I love rust"||); } func say = (str: string) { print(str); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}