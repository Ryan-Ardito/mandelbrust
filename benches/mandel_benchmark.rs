use mandelbrust::rendering::{MetaData, render};
use mandelbrust::imaging::{PostProc, upscale_buffer};

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let data = MetaData::new(
        1024,
        1024,
        -1.76,
        0.0,
        32.0,
        2u32.pow(10)
    );
    let buffer = render(data);

    c.bench_function("render", |b|
        b.iter(|| {
            render(data);
        })
    );

    c.bench_function("upscale buffer", |b|
        b.iter(|| {
            upscale_buffer(&buffer, data.width, data.height, 4);
        })
    );

    c.bench_function("gradient coloring", |b| {
        let post_proc = PostProc::new();
        b.iter(|| {
            post_proc.process(&buffer);
        })
    });

    c.bench_function("fast coloring", |b| {
        let mut post_proc = PostProc::new();
        post_proc.fast_color = true;
        b.iter(|| {
            post_proc.process(&buffer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);