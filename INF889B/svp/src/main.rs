use nalgebra::{vector, Matrix3};

fn main() {
    let b1 = vector![2.0, 3.0, 5.0];
    let b2 = vector![7.0, 11.0, 13.0];
    let b3 = vector![17.0, 19.0, 23.0];
    let basis = Matrix3::from_columns(&[b1, b2, b3]);
    println!("basis of lattice: {}", basis);

    let (cs, v) = svp::brute_force(&basis, false).unwrap();
    println!("shortest vector of lattice: λ = Ba");
    println!("λ = {}B = {}a = {}", v, basis, cs);

    // batching computes n points in the lattice from n coefficient vectors using a single matrix multiplication
    let a1 = vector![0.0, 0.0, 0.0];
    let a2 = vector![1.0, 1.0, 1.0];
    let a3 = vector![2.0, 2.0, 2.0];
    let coeffs = Matrix3::from_columns(&[a1, a2, a3]);
    println!("C = BA");
    println!("C = {}B = {}A = {}", basis * coeffs, basis, coeffs);
}
