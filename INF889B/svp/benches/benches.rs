use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::SMatrix;
use svp::brute_force;

fn bench_brute_force(c: &mut Criterion) {
    let base: SMatrix<f32, 5, 5> = SMatrix::from_row_iterator((0..).map(|i| i as f32));
    c.bench_function("brute force", |b| b.iter(|| brute_force(&black_box(base))));
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
