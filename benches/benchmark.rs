extern crate criterion;
extern crate dungenon;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dungenon::{
    generator::{FactionGen, FactionGen2},
    level::GridLevel,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("1028x1028 with 10 rounds");
    group.bench_function("factiongen1", |b| {
        b.iter_with_setup(
            || {
                let a = GridLevel::new(1028, 1028);
                (FactionGen::new(), a.clone(), a)
            },
            |(mut gen, mut a, mut b)| {
                (0..10).for_each(|_| {
                    gen.generate(&mut a, &mut b);
                });
                black_box((a, b));
            },
        );
    });
    group.bench_function("factiongen2", |b| {
        b.iter_with_setup(
            || {
                let a = GridLevel::new(1028, 1028);
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
