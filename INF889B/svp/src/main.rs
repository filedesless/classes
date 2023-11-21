use itertools::Itertools;
use nalgebra::{vector, Matrix3, Vector3};

// find the shortest vector in a lattice of given base
// ref: https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
// TODO: generalise to higher dimension
fn svp(base: &Matrix3<f32>) -> Vector3<f32> {
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
        .map(Vector3::from_vec)
        .map(|cs| base * cs.map(|ci| ci as f32))
        .filter(|v| v.norm() > 0.0)
        .min_by(|a, b| a.norm().total_cmp(&b.norm()))
        .unwrap()
}

fn main() {
    let b1 = vector![2.0, 3.0, 5.0];
    let b2 = vector![7.0, 11.0, 13.0];
    let b3 = vector![17.0, 19.0, 23.0];
    let base = Matrix3::from_columns(&[b1, b2, b3]);
    println!("{}", base);
    println!("{}", svp(&base));
}
