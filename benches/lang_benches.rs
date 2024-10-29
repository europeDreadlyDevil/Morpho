use criterion::{criterion_group, criterion_main, Criterion};
use func_lang::parser::ProgParser;
use func_lang::program::evaluating_functions::eval_program;
use tracing_log::log::{log, Level};

fn bench_for_block_with_anon_func(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { for(i in 0..10, $|i: i| { print(i); } ); }"#)
        .unwrap();
    c.bench_function("bench_for_block_with_anon_func", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_print_func(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { print("Hello"); }"#)
        .unwrap();
    c.bench_function("bench_print_func", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_condition_block_recursion(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { foo(10); } func foo = (a: int) { if(a == 0, $print|"end"|, $foo|a - 1|); } func foo1 = () {}"#).unwrap();;
    c.bench_function("bench_condition_block_recursion",|b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_for_block_with_anon_func_and_ref(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 10; for(i in 0..10, $|a: &a, i: i| {a = a + a; print(i+1, ":", a);});}"#).unwrap();
    c.bench_function("for_block_with_anon_func_and_ref",|b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_for_block_with_anon_func,
    bench_print_func,
    bench_condition_block_recursion,
    bench_for_block_with_anon_func_and_ref
);

criterion_main!(benches);
