use itertools::Itertools;
use nalgebra::{SMatrix, SVector};

/// returns norm of shortest vector in the basis
fn smallest_norm<const N: usize>(basis: &SMatrix<f32, N, N>) -> f32 {
    basis
        .column_iter()
        .map(|col| col.norm())
        .min_by(f32::total_cmp)
        .unwrap()
}

/// returns bounds for each coordinates from basis and the norm of a known short vector
fn get_bounds<const N: usize>(basis: &SMatrix<f32, N, N>, w: f32) -> Option<SVector<i32, N>> {
    // compute basis of the dual lattice to obtain bounds on coefficients to enumerate
    (basis.transpose() * basis).try_inverse().map(|b| {
        SVector::from_iterator(
            (basis * b)
                .column_iter()
                .map(|col| (col.norm() * w).ceil() as i32),
        )
    })
}

/// find the shortest vector (and its coefficients) in a lattice of given basis
pub fn brute_force<const N: usize>(
    basis: &SMatrix<f32, N, N>,
) -> Option<(SVector<i32, N>, SVector<f32, N>)> {
    // ref: https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
    // compute basis of the dual lattice to obtain bounds on coefficients to enumerate
    get_bounds(basis, smallest_norm(basis)).and_then(|bounds| {
        bounds
            .iter()
            .map(|ci| -ci..=*ci)
            .multi_cartesian_product()
            .map(|cs| SVector::from_iterator(cs) as SVector<i32, N>)
            .map(|cs| (cs, basis * cs.map(|ci| ci as f32)))
            .filter(|(_, v)| v.norm() > 0.0)
            .min_by(|(_, a), (_, b)| a.norm().total_cmp(&b.norm()))
    })
}

/// find the shortest vector (and its coefficients) in a lattice of given basis.
/// this one cuts the search space in half by respecting ||v|| = ||-v||
pub fn half_brute<const N: usize>(
    basis: &SMatrix<f32, N, N>,
) -> Option<(SVector<i32, N>, SVector<f32, N>)> {
    get_bounds(basis, smallest_norm(basis)).and_then(|bounds| {
        bounds
            .iter()
            .enumerate()
            // first coordinate is cut in half
            .map(|(i, ci)| if i > 0 { -ci..=*ci } else { 0..=*ci })
            .multi_cartesian_product()
            .map(|cs| SVector::from_iterator(cs) as SVector<i32, N>)
            .map(|cs| (cs, basis * cs.map(|ci| ci as f32)))
            .filter(|(_, v)| v.norm() > 0.0)
            .min_by(|(_, a), (_, b)| a.norm().total_cmp(&b.norm()))
    })
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use nalgebra::{debug::RandomSDP, Const};
    use quickcheck::{quickcheck, TestResult};

    use crate::{brute_force, half_brute};

    #[quickcheck]
    fn svp_is_correct(basis: RandomSDP<f32, Const<3>>) -> TestResult {
        let m = basis.unwrap();
        if !m.determinant().is_normal() {
            return TestResult::discard();
        }
        let a = brute_force(&m);
        let b = half_brute(&m);
        if a.is_none() && b.is_none() {
            return TestResult::passed();
        }
        if let Some((_, a)) = a {
            if let Some((_, b)) = b {
                println!("{} == {}", a, b);
                return TestResult::from_bool(a.norm() == b.norm());
            }
        }

        TestResult::failed()
    }
}
