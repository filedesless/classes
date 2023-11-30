use nalgebra::{vector, SMatrix};

use crate::M;

/// known example for basis reduction
pub fn ex3() -> M<3> {
    // details: https://www.math.ru.nl/~bosma/onderwijs/voorjaar07/compalg7.pdf
    let b1 = vector![1.0, 1.0, 1.0];
    let b2 = vector![-1.0, 0.0, 2.0];
    let b3 = vector![3.0, 5.0, 6.0];
    SMatrix::from_columns(&[b1, b2, b3])
}

/// known instance with SVP of norm sqrt(55)
pub fn ex5() -> M<5> {
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

/// large lattice basis for benching
pub fn ex10() -> M<10> {
    // from sage.crypto.lattice import gen_lattice
    // B = gen_lattice(m=10, seed=42)
    let b1 = vector![11, 0, 0, 0, 2, 1, -4, -2, -5, -4].map(|i| i as f64);
    let b2 = vector![0, 11, 0, 0, 4, -5, 3, -3, -5, -3].map(|i| i as f64);
    let b3 = vector![0, 0, 11, 0, 3, -4, -1, -4, 3, 2].map(|i| i as f64);
    let b4 = vector![0, 0, 0, 11, 5, 2, 1, -1, 3, -5].map(|i| i as f64);
    let b5 = vector![0, 0, 0, 0, 1, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b6 = vector![0, 0, 0, 0, 0, 1, 0, 0, 0, 0].map(|i| i as f64);
    let b7 = vector![0, 0, 0, 0, 0, 0, 1, 0, 0, 0].map(|i| i as f64);
    let b8 = vector![0, 0, 0, 0, 0, 0, 0, 1, 0, 0].map(|i| i as f64);
    let b9 = vector![0, 0, 0, 0, 0, 0, 0, 0, 1, 0].map(|i| i as f64);
    let b10 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 1].map(|i| i as f64);
    SMatrix::from_columns(&[b1, b2, b3, b4, b5, b6, b7, b8, b9, b10])
}

pub fn ex15() -> M<15> {
    let b1 = vector![11, 0, 0, 0, 2, 1, -4, -2, -5, -4, 3, -1, -5, 1, 1].map(|i| i as f64);
    let b2 = vector![0, 11, 0, 0, 4, -5, 3, -3, -5, -3, 5, -3, 0, 2, -3].map(|i| i as f64);
    let b3 = vector![0, 0, 11, 0, 3, -4, -1, -4, 3, 2, -5, 5, -4, 1, 0].map(|i| i as f64);
    let b4 = vector![0, 0, 0, 11, 5, 2, 1, -1, 3, -5, 0, 5, 3, 5, 0].map(|i| i as f64);
    let b5 = vector![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b6 = vector![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b7 = vector![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b8 = vector![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b9 = vector![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b10 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b11 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0].map(|i| i as f64);
    let b12 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0].map(|i| i as f64);
    let b13 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0].map(|i| i as f64);
    let b14 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0].map(|i| i as f64);
    let b15 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].map(|i| i as f64);
    SMatrix::from_columns(&[
        b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15,
    ])
}

pub fn ex20() -> M<20> {
    let b1 = vector![11, 0, 0, 0, 2, 1, -4, -2, -5, -4, 3, -1, -5, 1, 1, 4, 3, 1, 2, 5]
        .map(|i| i as f64);
    let b2 = vector![0, 11, 0, 0, 4, -5, 3, -3, -5, -3, 5, -3, 0, 2, -3, 3, -1, 5, -1, -1]
        .map(|i| i as f64);
    let b3 = vector![0, 0, 11, 0, 3, -4, -1, -4, 3, 2, -5, 5, -4, 1, 0, -3, 3, 0, 1, -1]
        .map(|i| i as f64);
    let b4 = vector![0, 0, 0, 11, 5, 2, 1, -1, 3, -5, 0, 5, 3, 5, 0, -1, -3, -5, -4, 0]
        .map(|i| i as f64);
    let b5 = vector![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b6 = vector![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b7 = vector![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b8 = vector![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b9 = vector![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b10 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b11 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b12 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b13 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b14 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b15 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b16 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0].map(|i| i as f64);
    let b17 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0].map(|i| i as f64);
    let b18 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0].map(|i| i as f64);
    let b19 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0].map(|i| i as f64);
    let b20 = vector![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].map(|i| i as f64);
    SMatrix::from_columns(&[
        b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18, b19, b20,
    ])
}
