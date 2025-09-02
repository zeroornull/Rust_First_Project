use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[allow(dead_code)]
fn sum_for(x: &[f64]) -> f64 {
    let mut result = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

#[allow(dead_code)]
fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[allow(dead_code)]
fn bench_sum(c: &mut Criterion) {
    let data: Vec<f64> = (0..1024*1024).map(|_| rand::random::<f64>()).collect();
    c.bench_function("sum_for", |b| b.iter(|| sum_for(black_box(&data))));
    c.bench_function("sum_iter", |b| b.iter(|| sum_iter(black_box(&data))));
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
