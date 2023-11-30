use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svp::{brute_force, lll, data};


fn bench_brute_force(c: &mut Criterion) {
    let basis = data::ex10();
    c.bench_function("brute force", |b| {
        b.iter(|| brute_force(&black_box(basis), false, false))
    });
    c.bench_function("half brute", |b| {
        b.iter(|| brute_force(&black_box(basis), true, false))
    });
    c.bench_function("brute force lll", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), false, false)
        })
    });
    c.bench_function("half brute lll", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), true, false)
        })
    });
    c.bench_function("brute force cut", |b| {
        b.iter(|| brute_force(&black_box(basis), false, true))
    });
    c.bench_function("half brute cut", |b| {
        b.iter(|| brute_force(&black_box(basis), true, true))
    });
    c.bench_function("brute force lll cut", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), false, true)
        })
    });
    c.bench_function("half brute lll cut", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), true, true)
        })
    });
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
