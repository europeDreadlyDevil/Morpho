use criterion::{criterion_group, criterion_main, Criterion};
use tracing_log::log::{log, Level};
use func_lang::parser::ProgParser;
use func_lang::program::evaluating_functions::eval_program;

fn bench_for_block_with_anon_func(c: &mut Criterion) {
    log!(Level::Info, "Benching for_block_with_anon_func...");
    c.bench_function("bench_for_block_with_anon_func",|b| {
        b.iter(|| {
            let ast = ProgParser::new()
                .parse(r#"func main = () { for(i in 0..10, $|i: i| { print(i); } ); }"#).unwrap();
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_print_func(c: &mut Criterion) {
    log!(Level::Info, "Benching bench_print_func...");
    c.bench_function("bench_print_func",|b| {
        b.iter(|| {
            let ast = ProgParser::new()
                .parse(r#"func main = () { print("Hello, world!"); }"#).unwrap();
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_condition_block_recursion(c: &mut Criterion)  {
    log!(Level::Info, "Benching condition_block_recursion...");
    c.bench_function("bench_condition_block_recursion",|b| {
        b.iter(|| {
            let ast = ProgParser::new()
                .parse(r#"func main = () { foo(10); } func foo = (a: int) { print(a); if(a == 0, $print|"end"|, $foo|a - 1|); }"#).unwrap();
            eval_program(ast).unwrap();
        });
    });
}


criterion_group!(
    benches,
    bench_for_block_with_anon_func,
    bench_print_func,
    bench_condition_block_recursion
);

criterion_main!(benches);