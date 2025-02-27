extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use isomorphic_graph_creation::{create_random_graph, generate_subgraph_parallel, unrank, unrank_parallel};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

fn _create_random_input(size: u32) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(32);
    (0..size)
        .into_iter()
        .map(|_| rng.gen_range(0..100))
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_unrank");
    group.measurement_time(Duration::new(10, 0));
    group.sample_size(10);

    const GRAPH_SIZE: u32 = 20;
    const SUBSET_SIZE: u64 = 10;
    const SUBGRAPH_COUNT: u64 = 1000;
    let input = _create_random_input(GRAPH_SIZE);
    //group.bench_function("single_unrank", |b| b.iter(|| unrank(&input, SUBSET_SIZE)));
    //group.bench_function("parallel_unrank", |b| {
    //    b.iter(|| unrank_parallel(&input, SUBSET_SIZE))
    //});
    group.bench_function("parallel_subgraph", |b| {
        b.iter(|| generate_subgraph_parallel(&create_random_graph(GRAPH_SIZE as usize, Some(100)), SUBSET_SIZE, SUBGRAPH_COUNT))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
