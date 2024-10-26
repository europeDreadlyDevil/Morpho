use anyhow::Result;
use func_lang::parser::ProgParser;
use tracing_log::log::{log, Level};
use func_lang::program::evaluating_functions::eval_program;

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
    for_block_with_anon_func()?;
    Ok(())
}

fn print_hello_world() -> Result<()> {
    log!(Level::Info, "print_hello_world");
    let ast = ProgParser::new().parse(r#"func main = () { print("Hello, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_hello_world_with_var() -> Result<()> {
    log!(Level::Info, "print_hello_world_with_var");
    let ast =
        ProgParser::new().parse(r#"func main = () { let x = "Hello, world!"; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_int_with_var() -> Result<()> {
    log!(Level::Info, "print_int_with_var");
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn print_int_with_two_var() -> Result<()> {
    log!(Level::Info, "print_int_with_two_var");
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; let y = x; print(y); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn create_new_func() -> Result<()> {
    log!(Level::Info, "create_new_func");
    let ast = ProgParser::new().parse(r#"func main = () { hello_world(); bye_world(); } func hello_world = () { print("Hello, world!"); } func bye_world = () { print("Bye, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn call_func_with_args() -> Result<()> {
    log!(Level::Info, "call_func_with_args");
    let ast = ProgParser::new()
        .parse(r#"func main = () { say("I'm John"); } func say = (x: string) { print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn eval_expr_in_func() -> Result<()> {
    log!(Level::Info, "eval_expr_in_func");
    let ast = ProgParser::new()
        .parse(r#"func main = () { print(20 - 50); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn eval_expr_in_func_with_vars() -> Result<()> {
    log!(Level::Info, "eval_expr_in_func_with_vars");
    let ast = ProgParser::new()
        .parse(r#"func main = () { let x1 = 60; let x2 = 30; let y = x1 * x2; print(x1, x2, y, x1-x2); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn condition_block() -> Result<()> {
    log!(Level::Info, "condition_block");
    let ast = ProgParser::new()
        .parse(r#"func main = () { if(true, print("TRUE!")); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}
fn condition_block_recursion() -> Result<()> {
    log!(Level::Info, "condition_block_recursion");
    let ast = ProgParser::new()
        .parse(r#"func main = () { foo(10); } func foo = (a: int) { print(a); if(a == 0, $print|"end"|, $foo|a - 1|); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn for_block() -> Result<()> {
    log!(Level::Info, "for_block");
    let ast = ProgParser::new()
        .parse(r#"func main = () { for(i in 0..10, $say|i|); } func say = (num: int) { print(num); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}

fn for_block_with_anon_func() -> Result<()> {
    log!(Level::Info, "for_block_with_anon_func");
    let ast = ProgParser::new()
        .parse(r#"func main = () { for(i in 0..10, $|i: i| { print(i); } ); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast).unwrap();
    Ok(())
}