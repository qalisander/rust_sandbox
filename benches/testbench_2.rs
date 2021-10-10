use bit_set::BitSet;
use bit_vec::BitVec;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
//use criterion_macro::criterion;

const SIZES: [usize; 5] = [20, 30, 40, 60, 100];

fn bench_slice_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("Slice_join");

    for size in SIZES {
        let vec3 = (1usize..size).filter(|i| i % 3 == 0).into_iter().collect_vec();
        let vec5 = (1usize..size).filter(|i| i % 5 == 0).into_iter().collect_vec();

        group.bench_function(&*format!("size: {}", &size), |b| {
            b.iter(|| { vec3.clone().iter().merge(&vec5).collect_vec(); });
        });
    }
}

fn bench_slice_bitset(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitset join");

    for size in SIZES {
        let vec3 = (1usize..size).filter(|i| i % 3 == 0).into_iter().collect_vec();
        let vec5 = (1usize..size).filter(|i| i % 5 == 0).into_iter().collect_vec();

        let bit_set_1: BitSet<u32> = BitSet::from_iter(vec3);
        let bit_set_2: BitSet<u32> = BitSet::from_iter(vec5);

        group.bench_function(&*format!("size: {}", &size), |b| {
            b.iter(|| { bit_set_1.clone().intersect_with(&bit_set_2) });
        });
    }

    group.finish();
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)
}

criterion_group!(benches, bench_slice_join, bench_slice_bitset);
criterion_main!(benches);
