use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svp::{brute_force, lll, data};


fn bench_brute_force(c: &mut Criterion) {
    let basis = data::ex10hard();
    c.bench_function("brute", |b| {
        b.iter(|| brute_force(&black_box(basis), false, false))
    });
    c.bench_function("brute half", |b| {
        b.iter(|| brute_force(&black_box(basis), true, false))
    });
    c.bench_function("brute cut", |b| {
        b.iter(|| brute_force(&black_box(basis), false, true))
    });
    c.bench_function("brute half cut", |b| {
        b.iter(|| brute_force(&black_box(basis), true, true))
    });
    c.bench_function("brute lll", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), false, false)
        })
    });
    c.bench_function("brute all", |b| {
        b.iter(|| {
            let basis = lll(black_box(basis), 0.75);
            brute_force(&black_box(basis), true, true)
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_brute_force
}
criterion_main!(benches);
