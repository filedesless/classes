use itertools::Itertools;
use nalgebra::{vector, Matrix3, SMatrix, SVector};

// find the shortest vector in a lattice of given base
// ref: https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
fn svp<const N: usize>(base: &SMatrix<f32, N, N>) -> (SVector<i32, N>, SVector<f32, N>) {
    // euclidean norm of smallest vector in the base
    let l: f32 = base
        .column_iter()
        .map(|col| col.norm())
        .min_by(f32::total_cmp)
        .unwrap();
    // compute base of the dual lattice to obtain bounds on coefficients to enumerate
    (base * (base.transpose() * base).try_inverse().unwrap())
        .column_iter()
        .map(|col| (col.norm() * l).ceil() as i32)
        .map(|ci| -ci..=ci)
        .multi_cartesian_product()
        .map(|cs| SVector::from_iterator(cs) as SVector<i32, N>)
        .map(|cs| (cs, base * cs.map(|ci| ci as f32)))
        .filter(|(_, v)| v.norm() > 0.0)
        .min_by(|(_, a), (_, b)| a.norm().total_cmp(&b.norm()))
        .unwrap()
}

fn main() {
    let b1 = vector![2.0, 3.0, 5.0];
    let b2 = vector![7.0, 11.0, 13.0];
    let b3 = vector![17.0, 19.0, 23.0];
    let base = Matrix3::from_columns(&[b1, b2, b3]);
    println!("base of lattice: {}", base);

    let (cs, v) = svp(&base);
    println!("shortest vector of lattice: λ = Ba");
    println!("λ = {}B = {}a = {}", v, base, cs);

    // batching computes n points in the lattice from n coefficient vectors using a single matrix multiplication
    let a1 = vector![0.0, 0.0, 0.0];
    let a2 = vector![1.0, 1.0, 1.0];
    let a3 = vector![2.0, 2.0, 2.0];
    let coeffs = Matrix3::from_columns(&[a1, a2, a3]);
    println!("C = BA");
    println!("C = {}B = {}A = {}", base * coeffs, base, coeffs);
}
