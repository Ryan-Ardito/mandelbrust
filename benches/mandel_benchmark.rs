use mandelbrust::rendering::{render, MetaData};

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render", |b|
        b.iter(|| {
            let data = MetaData::new(
                1024,
                1024,
                -1.8,
                0.0,
                2f64.powi(12),
                2u32.pow(12)
            );
            render(data);
        })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);