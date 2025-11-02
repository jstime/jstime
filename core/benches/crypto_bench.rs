use criterion::{Criterion, criterion_group, criterion_main};
use ring::rand::SecureRandom;
use std::hint::black_box;

fn bench_system_random(c: &mut Criterion) {
    let system_random = ring::rand::SystemRandom::new();

    c.bench_function("system_random_16", |b| {
        let mut data = vec![0u8; 16];
        b.iter(|| {
            system_random.fill(black_box(&mut data)).unwrap();
        });
    });

    c.bench_function("system_random_1024", |b| {
        let mut data = vec![0u8; 1024];
        b.iter(|| {
            system_random.fill(black_box(&mut data)).unwrap();
        });
    });

    c.bench_function("system_random_65536", |b| {
        let mut data = vec![0u8; 65536];
        b.iter(|| {
            system_random.fill(black_box(&mut data)).unwrap();
        });
    });
}

criterion_group!(benches, bench_system_random);
criterion_main!(benches);
