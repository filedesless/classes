use itertools::Itertools;
use nalgebra::{SMatrix, SVector, vector};

/// known instance with SVP of norm sqrt(55)
pub fn example() -> SMatrix<f32, 5, 5> {
    // [11  0  0  0  2]
    // [ 0 11  0  0  4]
    // [ 0  0 11  0  3]
    // [ 0  0  0 11  5]
    // [ 0  0  0  0  1]
    let b1 = vector![11.0, 0.0, 0.0, 0.0, 0.0];
    let b2 = vector![0.0, 11.0, 0.0, 0.0, 0.0];
    let b3 = vector![0.0, 0.0, 11.0, 0.0, 0.0];
    let b4 = vector![0.0, 0.0, 0.0, 11.0, 0.0];
    let b5 = vector![2.0, 4.0, 3.0, 5.0, 1.0];
    SMatrix::from_columns(&[b1, b2, b3, b4, b5])
}

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
///
/// * `basis` - matrix whose columns are linearly independant
/// * `half_space` - cut the search space in half
pub fn brute_force<const N: usize>(
    basis: &SMatrix<f32, N, N>,
    half_space: bool,
) -> Option<(SVector<i32, N>, SVector<f32, N>)> {
    // ref: https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
    // compute basis of the dual lattice to obtain bounds on coefficients to enumerate
    get_bounds(basis, smallest_norm(basis)).and_then(|bounds| {
        bounds
            .iter()
            .enumerate()
            .map(|(i, ci)| {
                if half_space && i == 0 {
                    0..=*ci
                } else {
                    -ci..=*ci
                }
            })
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
    use nalgebra::{debug::RandomSDP, Const, ComplexField};
    use quickcheck::{quickcheck, TestResult};

    use crate::{brute_force, example};

    #[quickcheck]
    fn svp_quickcheck(basis: RandomSDP<f32, Const<3>>) -> TestResult {
        let m = basis.unwrap();
        if !m.determinant().is_normal() {
            return TestResult::discard();
        }
        let a = brute_force(&m, false);
        let b = brute_force(&m, true);
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

    #[test]
    fn svp_known() {
        let basis = example();
        let expected = 55.0.sqrt();
        assert_eq!(
            brute_force(&basis, true).map(|(_, v)| v.norm()),
            Some(expected)
        )
    }
}
