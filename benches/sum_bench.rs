use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

pub fn prepped_sum(data: &[f32]) -> f32 {
    let mut acc0 = 0.0;
    let mut acc1 = 0.0;
    let mut acc2 = 0.0;
    let mut acc3 = 0.0;
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();
    for chunk in chunks {
        acc0 += chunk[0];
        acc1 += chunk[1];
        acc2 += chunk[2];
        acc3 += chunk[3];
    }
    let mut sum = acc0 + acc1 + acc2 + acc3;
    for &x in remainder {
        sum += x;
    }
    sum
}

#[inline(never)]
pub fn divide(data: &[f32], divisor: f32) -> f32 {
    let mut sum = 0.0;
    for &x in data {
        sum += x / divisor;
    }
    sum
}

#[inline(never)]
pub fn prepped_divide(data: &[f32], divisor: f32) -> f32 {
    let mut acc0 = 0.0;
    let mut acc1 = 0.0;
    let mut acc2 = 0.0;
    let mut acc3 = 0.0;

    for chunk in data.chunks_exact(4) {
        acc0 += chunk[0] / divisor;
        acc1 += chunk[1] / divisor;
        acc2 += chunk[2] / divisor;
        acc3 += chunk[3] / divisor;
    }

    acc0 + acc1 + acc2 + acc3
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 40_000_000;
    let data = vec![1.1f32; size];

    let mut group_sum = c.benchmark_group("Summing elements of [f32]");
    group_sum.bench_function("native sum", |b| b.iter(|| sum(black_box(&data))));
    group_sum.bench_function("prepped sum", |b| b.iter(|| prepped_sum(black_box(&data))));
    group_sum.finish();

    let size = 10_000_000;
    let data = vec![100.0f32; size];
    let divisor = black_box(7.0f32);

    let mut group_div = c.benchmark_group("Division elements of [f32]");

    group_div.bench_function("divide", |b| b.iter(|| divide(black_box(&data), divisor)));

    group_div.bench_function("prepped divide", |b| {
        b.iter(|| prepped_divide(black_box(&data), divisor))
    });

    group_div.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
