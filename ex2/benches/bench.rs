use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ex2::{run};

fn criterion_benchmark(c: &mut Criterion) {
    let input1 = include_str!("../input/parts/2.1");

    c.bench_function("ex2::run", |b| b.iter(|| run(black_box(input1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);