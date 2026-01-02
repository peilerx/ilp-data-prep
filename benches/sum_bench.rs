use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn native_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

pub fn native_ilp_sum(data: &[f32]) -> f32 {
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

pub fn idiomatic_ilp_sum(data: &[f32]) -> f32 {
    data.chunks_exact(4)
        .fold([0.0; 4], |mut acc, chunk| {
            acc[0] += chunk[0];
            acc[1] += chunk[1];
            acc[2] += chunk[2];
            acc[3] += chunk[3];
            acc
        })
        .iter()
        .sum::<f32>()
        + data.chunks_exact(4).remainder().iter().sum::<f32>()
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 40_000_000;
    let data = vec![1.1f32; size];

    let mut group = c.benchmark_group("Summing");

    group.bench_function("native", |b| b.iter(|| native_sum(black_box(&data))));

    group.bench_function("native-ilp-style", |b| {
        b.iter(|| native_ilp_sum(black_box(&data)))
    });

    group.bench_function("idiomatic-ilp-style", |b| {
        b.iter(|| idiomatic_ilp_sum(black_box(&data)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
