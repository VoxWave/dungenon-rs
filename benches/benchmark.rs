extern crate criterion;
extern crate dungenon;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dungenon::{
    generator::{FactionGen, FactionGen2},
    level::GridLevel,
};

pub fn compare_implementations(c: &mut Criterion) {
    let (width, height, rounds) = (1024, 1024, 10);
    let mut group = c.benchmark_group("by algorithm");
    group.bench_function("factiongen1", |b| {
        b.iter_with_setup(
            || {
                let a = GridLevel::new(width, height);
                (FactionGen::new(), a.clone(), a)
            },
            |(mut gen, mut a, mut b)| {
                (0..rounds).for_each(|_| {
                    gen.generate(&mut a, &mut b);
                });
                black_box((a, b));
            },
        );
    });
    group.bench_function("factiongen2", |b| {
        b.iter_with_setup(
            || {
                let a = GridLevel::new(width, height);
                (FactionGen2::new(), a.clone(), a)
            },
            |(mut gen, mut a, mut b)| {
                (0..rounds).for_each(|_| {
                    gen.generate(&mut a, &mut b);
                });
                black_box((a, b));
            },
        );
    });
    group.finish();
}

pub fn compare_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("by size");
    for i in 1..=10 {
        let side = i * 1024;
        group.bench_with_input(BenchmarkId::from_parameter(side), &side, |b, &side| {
            b.iter_with_setup(
                || {
                    let a = GridLevel::new(side, side);
                    (FactionGen2::new(), a.clone(), a)
                },
                |(mut gen, mut a, mut b)| {
                    (0..10).for_each(|_| {
                        gen.generate(&mut a, &mut b);
                    });
                    black_box((a, b));
                },
            );
        });
    }
    group.finish();
}

pub fn compare_aspect_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("by aspect ratio");
    let steps = 12;
    let size = 1 << steps;
    for i in 0..=steps {
        let width = size >> i;
        let height = 1 << i;
        group.bench_with_input(
            BenchmarkId::from_parameter(width as f32 / height as f32),
            &(width, height),
            |b, &(width, height)| {
                b.iter_with_setup(
                    || {
                        let a = GridLevel::new(width, height);
                        (FactionGen2::new(), a.clone(), a)
                    },
                    |(mut gen, mut a, mut b)| {
                        (0..10).for_each(|_| {
                            gen.generate(&mut a, &mut b);
                        });
                        black_box((a, b));
                    },
                );
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    compare_implementations,
    compare_aspect_ratio,
    compare_sizes
);
criterion_main!(benches);
