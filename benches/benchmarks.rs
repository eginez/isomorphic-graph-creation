extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use isomorphic_graph_creation::{binomial_coefficient, unrank, unrank_combination_single};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

fn _create_random_input(size: u32) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(32);
    (0..size)
        .into_iter()
        .map(|_| rng.gen_range(0..100))
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_unrank");
    group.measurement_time(Duration::new(10, 0));

    let input = _create_random_input(20);
    group.bench_function("single_unrank", |b| b.iter(|| unrank(&input, 4)));
    group.bench_function("parallel_unrank", |b| b.iter(|| unrank(&input, 4)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
