use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_lab::nonogram_solvers::solve_nonogram;
use rust_lab::nonogram_solvers::test::CLUES_15;

//https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html

fn nonogram_solover_bench(c: &mut Criterion) {
    c.benchmark_group("nonogram_solver_bench")
        .sample_size(20)
        .bench_function("nonogram_solover_bench", |b| {
            b.iter(|| solve_nonogram(CLUES_15))
        });
}

criterion_group!(benches, nonogram_solover_bench);
criterion_main!(benches);
