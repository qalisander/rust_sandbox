use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use criterion_macro::criterion;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
