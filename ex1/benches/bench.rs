use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ex1::{naive_1, run};

fn criterion_benchmark(c: &mut Criterion) {
    let input1 = include_str!("../input/parts/1.1");
    let input2 = include_str!("../input/parts/1.2");
    c.bench_function("ex1::naive_1", |b| b.iter(|| naive_1(black_box(input1.to_string()))));
    c.bench_function("ex1::run", |b| b.iter(|| run(black_box(input2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);