use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svp::{brute_force, example};

fn bench_brute_force(c: &mut Criterion) {
    let basis = example();
    c.bench_function("brute force", |b| {
        b.iter(|| brute_force(&black_box(basis), false))
    });
    c.bench_function("half brute", |b| {
        b.iter(|| brute_force(&black_box(basis), true))
    });
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
