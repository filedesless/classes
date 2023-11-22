use itertools::Itertools;
use nalgebra::{SMatrix, SVector};

/// find the shortest vector (and its coefficients) in a lattice of given base
pub fn brute_force<const N: usize>(base: &SMatrix<f32, N, N>) -> (SVector<i32, N>, SVector<f32, N>) {
    // ref: https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
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