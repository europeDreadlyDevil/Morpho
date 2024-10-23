use func_lang::eval_program;
use func_lang::parser::ProgParser;
use anyhow::Result;
use anyhow::Error;
use tracing_log::log::{log, Level};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    print_hello_world()?;
    print_hello_world_with_var()?;
    print_int_with_var()?;
    print_int_with_two_var()?;
    create_new_func()?;
    call_func_with_args()?;
    Ok(())
}

fn print_hello_world() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { print("Hello, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast)?;
    Ok(())
}

fn print_hello_world_with_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = "Hello, world!"; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast)?;
    Ok(())
}

fn print_int_with_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; print(x); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast)?;
    Ok(())
}

fn print_int_with_two_var() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { let x = 10; let y = x; print(y); }"#)?;
    log!(Level::Info, "Starting...");
    eval_program(ast)?;
    Ok(())
}

fn create_new_func() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { hello_world(); bye_world(); } func hello_world = () { print("Hello, world!"); } func bye_world = () { print("Bye, world!"); }"#)?;
    log!(Level::Info, "Starting...");
    println!("{ast:?}");
    eval_program(ast)?;
    Ok(())
}

fn call_func_with_args() -> Result<()> {
    let ast = ProgParser::new().parse(r#"func main = () { say("I'm John"); } func say = (x: string) { print(x); }"#)?;
    log!(Level::Info, "Starting...");
    println!("{ast:?}");
    eval_program(ast)?;
    Ok(())
}