use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ex2::{naive, regexp, run, run_2};

fn criterion_benchmark(c: &mut Criterion) {
    let input1 = include_str!("../input/parts/2.1");

    c.bench_function("ex2::run", |b| b.iter(|| run(black_box(input1))));
    c.bench_function("ex2::naive", |b| b.iter(|| naive(black_box(input1))));
    c.bench_function("ex2::regex", |b| b.iter(|| regexp(black_box(input1))));
    c.bench_function("ex2::run2", |b| b.iter(|| run_2(black_box(input1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);