use std::cmp::max;
pub mod data;

use nalgebra::{SMatrix, SVector};

/// lattice point
pub type V<const N: usize> = SVector<f64, N>;
/// lattice basis
pub type M<const N: usize> = SMatrix<f64, N, N>;

/// orthogonal projection of v onto the line spanned by u
fn projection<const N: usize>(u: V<N>, v: V<N>) -> V<N> {
    (v.dot(&u) / u.dot(&u)) * u
}

/// computes a new basis orthogonal to the given one generating the same space
fn gram_schmidt_process<const N: usize>(basis: &M<N>) -> M<N> {
    // https://en.wikipedia.org/wiki/Gram–Schmidt_process
    let mut u: Vec<SVector<f64, N>> = vec![SVector::from(basis.column(0))];
    for i in 1..N {
        let vi = SVector::from(basis.column(i));
        let ui = vi - (0..i).map(|j| projection(u[j], vi)).sum::<V<N>>();
        u.push(ui);
    }
    M::from_columns(&u)
}

/// Lenstra–Lenstra–Lovász lattice basis reduction algorithm
pub fn lll<const N: usize>(mut basis: M<N>, delta: f64) -> M<N> {
    // https://en.wikipedia.org/wiki/Lenstra–Lenstra–Lovász_lattice_basis_reduction_algorithm
    let mut orth = gram_schmidt_process(&basis);
    let mu =
        |b: &M<N>, o: &M<N>, i, j| (b.column(i).dot(&o.column(j))) / o.column(j).norm_squared();
    let mut k = 1;
    while k < N {
        for j in (0..k).rev() {
            let m = mu(&basis, &orth, k, j);
            if m.abs() > 0.5 {
                // size reduced
                let bk = basis.column(k) - m.round() * basis.column(j);
                basis.set_column(k, &bk);
                orth = gram_schmidt_process(&basis);
            }
        }
        let ok = orth.column(k);
        let okk = orth.column(k - 1);
        let m = mu(&basis, &orth, k, k - 1);
        if ok.norm_squared() > ((delta - m.powi(2)) * okk.norm_squared()) {
            // lovasz condition
            k = k + 1
        } else {
            basis.swap_columns(k, k - 1);
            orth = gram_schmidt_process(&basis);
            k = max(k - 1, 1);
        }
    }
    basis
}

/// returns shortest vector in the basis
fn smallest<const N: usize>(basis: &M<N>) -> V<N> {
    basis
        .column_iter()
        .min_by(|a, b| a.norm().total_cmp(&b.norm()))
        .unwrap()
        .clone_owned()
}

/// returns bounds for each coordinates from basis and the norm of a known short vector
fn get_bounds<const N: usize>(basis: &M<N>, w: f64) -> Option<SVector<i32, N>> {
    // compute basis of the dual lattice to obtain bounds on coefficients to enumerate
    // https://en.wikipedia.org/wiki/Dual_lattice
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
/// * `cut_space` - cut the search space whenever a new minimum is found
pub fn brute_force<const N: usize>(basis: &M<N>, half_space: bool, cut_space: bool) -> Option<V<N>> {
    // https://www.ams.org/journals/mcom/1975-29-131/S0025-5718-1975-0379386-6/S0025-5718-1975-0379386-6.pdf
    let mut best: V<N> = smallest(&basis);
    get_bounds(&basis, best.norm()).map(|mut bounds| {
        println!("initial bound: {}", bounds);
        let next = |i: i32| if i > 0 { -i } else { -i + 1 };
        let mut coefficients: SVector<i32, N> = SVector::zeros();
        while coefficients[N-1] < bounds[N-1] {
            // println!("coefficients : {}", coefficients);
            let point: V<N> = basis * coefficients.map(|i| i as f64);
            let w = point.norm();
            if 0.0 < w && w < best.norm() {
                best = point;
                if cut_space {
                    bounds = get_bounds(basis, best.norm()).unwrap();
                    println!("bounds: {}coefficients: {}", bounds, coefficients);
                }
            }
            coefficients[0] = if half_space { coefficients[0] + 1 } else { next(coefficients[0]) };
            for i in 0..coefficients.len()-1 {
                if coefficients[i] >= bounds[i] {
                    coefficients[i] = 0;
                    coefficients[i+1] = next(coefficients[i+1]);
                }
            }
        }
        best
    })
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use nalgebra::{debug::RandomSDP, vector, ComplexField, Const};
    use quickcheck::{quickcheck, TestResult};

    use crate::*;

    #[quickcheck]
    fn svp_quickcheck(basis: RandomSDP<f64, Const<3>>) -> TestResult {
        let m = basis.unwrap();
        if !m.determinant().is_normal() {
            return TestResult::discard();
        }
        if m.determinant() > 1000.0 {
            return TestResult::discard();
        }
        let a = brute_force(&m, false, false);
        let b = brute_force(&m, true, true);
        if a.is_none() && b.is_none() {
            return TestResult::discard();
        }
        if let Some(a) = a {
            if let Some(b) = b {
                return TestResult::from_bool((a.norm() - b.norm()).abs() < f64::EPSILON);
            }
        }

        TestResult::failed()
    }

    #[test]
    fn svp_known() {
        let basis = data::ex5();
        let expected = 55.0.sqrt();
        assert_eq!(brute_force(&basis, false, false).map(|v| v.norm()), Some(expected));
        assert_eq!(brute_force(&basis, true, false).map(|v| v.norm()), Some(expected));
        assert_eq!(brute_force(&basis, true, true).map(|v| v.norm()), Some(expected));
        assert_eq!(brute_force(&basis, false, true).map(|v| v.norm()), Some(expected));
        let basis = lll(basis, 0.75);
        assert_eq!(brute_force(&basis, true, true).map(|v| v.norm()), Some(expected));
    }

    #[test]
    fn test_gso() {
        let b1 = vector![3.0, 1.0];
        let b2 = vector![2.0, 2.0];
        let basis: M<2> = M::from_columns(&[b1, b2]);
        let u2 = vector![-2.0 / 5.0, 6.0 / 5.0];
        let expected: M<2> = M::from_columns(&[b1, u2]);
        let actual: M<2> = gram_schmidt_process(&basis);
        assert!((actual.norm() - expected.norm()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_lll() {
        let basis: M<3> = data::ex3();
        let b1 = vector![0.0, 1.0, 0.0];
        let b2 = vector![1.0, 0.0, 1.0];
        let b3 = vector![-1.0, 0.0, 2.0];
        let expected = M::from_columns(&[b1, b2, b3]);
        let actual = lll(basis, 0.75);
        assert_eq!(actual, expected);
    }
}
