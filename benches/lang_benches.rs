use criterion::{criterion_group, criterion_main, Criterion};
use func_lang::parser::ProgParser;
use func_lang::program::evaluating_functions::eval_program;

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
        .parse(r#"func main = () { foo(10); } func foo = (a: int) { if(a == 0, $print|"end"|, $foo|a - 1|); } func foo1 = () {}"#).unwrap();
    c.bench_function("bench_condition_block_recursion", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_for_block_with_anon_func_and_ref_10(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 10; for(i in 0..10, $|a: &a, i: i| {a = a + a; print(i+1, ":", a);});}"#).unwrap();
    c.bench_function("bench_for_block_with_anon_func_and_ref_10", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_for_block_with_func_and_ref_10(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 10; for(i in 0..10, $f|&a, i|); } func f = (a: int, i: int) { a = a + a; print(i+1, ":", a); }"#).unwrap();
    c.bench_function("bench_for_block_with_func_and_ref_10", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_for_block_with_anon_func_and_ref_20(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 10; for(i in 0..20, $|a: &a, i: i| {a = a + a; print(i+1, ":", a);});}"#).unwrap();
    c.bench_function("bench_for_block_with_anon_func_and_ref_20", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_for_block_with_func_and_ref_20(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 10; for(i in 0..20, $f|&a, i|); } func f = (a: int, i: int) { a = a + a; print(i+1, ":", a); }"#).unwrap();
    c.bench_function("bench_for_block_with_func_and_ref_20", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_evaluating_fibonacci_5(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { fibonacci(5); } func fibonacci = (n: int) -> int {return if(n <= 1, $|n: n| -> int { return n; }, $|n: n| -> int { return fibonacci(n-1) + fibonacci(n-2); });}"#).unwrap();
    c.bench_function("bench_evaluating_fibonacci_5", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_evaluating_fibonacci_10(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { fibonacci(10); } func fibonacci = (n: int) -> int {return if(n <= 1, $|n: n| -> int { return n; }, $|n: n| -> int { return fibonacci(n-1) + fibonacci(n-2); });}"#).unwrap();
    c.bench_function("bench_evaluating_fibonacci_10", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_evaluating_fibonacci_20(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { fibonacci(20);} func fibonacci = (n: int) -> int {return if(n <= 1, $|n: n| -> int { return n; }, $|n: n| -> int { return fibonacci(n-1) + fibonacci(n-2); });}"#).unwrap();
    c.bench_function("bench_evaluating_fibonacci_20", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}

fn bench_evaluating_fibonacci_iter_20(c: &mut Criterion) {
    let ast = ProgParser::new()
        .parse(r#"func main = () { let a = 0; let b = 1; for(0..20, $|a: &a, b: &b | { let temp = b; b = a + b; a = temp; }); }"#).unwrap();
    c.bench_function("bench_evaluating_fibonacci_iter_20", |b| {
        b.iter(|| {
            eval_program(ast.clone()).unwrap();
        });
    });
}
criterion_group!(
    benches,
    // bench_for_block_with_anon_func,
    // bench_print_func,
    // bench_condition_block_recursion,
    bench_for_block_with_anon_func_and_ref_10,
    bench_for_block_with_func_and_ref_10,
    bench_for_block_with_anon_func_and_ref_20,
    bench_for_block_with_func_and_ref_20
    // bench_evaluating_fibonacci_5,
    // bench_evaluating_fibonacci_10,
    // bench_evaluating_fibonacci_20,
    // bench_evaluating_fibonacci_iter_20
);

criterion_main!(benches);
