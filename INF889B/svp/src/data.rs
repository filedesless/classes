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

pub fn ex5easy() -> M<5> {
    // svp [0, 1, 0, 0, 0], norm 1
    let b1 = vector![-1, -2, -3, -3, -1].map(|i| i as f64);
    let b2 = vector![0, 0, -3, 0, -3].map(|i| i as f64);
    let b3 = vector![0, 1, 2, 8, 0].map(|i| i as f64);
    let b4 = vector![1, 0, -3, 1, 0].map(|i| i as f64);
    let b5 = vector![-1, -1, 0, -3, 2].map(|i| i as f64);
    SMatrix::from_columns(&[b1, b2, b3, b4, b5])
}

pub fn ex10easy() -> M<10> {
    // svp 3*sqrt(7); det = 11
    let b1 = vector![10, 11, 1, 4, 3, 7, 10, 0, 10, 8].map(|i| i as f64);
    let b2 = vector![0, 4, 11, 4, 8, 7, 8, 0, 11, 1].map(|i| i as f64);
    let b3 = vector![5, 0, 11, 5, 2, 1, 5, 4, 11, 11].map(|i| i as f64);
    let b4 = vector![5, 11, 12, 7, 7, 5, 8, 10, 2, 12].map(|i| i as f64);
    let b5 = vector![0, 5, 2, 11, 10, 6, 4, 6, 4, 9].map(|i| i as f64);
    let b6 = vector![6, 9, 2, 3, 11, 1, 2, 5, 7, 10].map(|i| i as f64);
    let b7 = vector![7, 2, 3, 4, 11, 6, 12, 9, 9, 0].map(|i| i as f64);
    let b8 = vector![1, 0, 1, 2, 1, 0, 1, 6, 4, 6].map(|i| i as f64);
    let b9 = vector![5, 8, 0, 11, 4, 7, 11, 3, 7, 2].map(|i| i as f64);
    let b10 = vector![6, 4, 2, 7, 4, 1, 8, 1, 1, 7].map(|i| i as f64);
    SMatrix::from_columns(&[b1, b2, b3, b4, b5, b6, b7, b8, b9, b10])
}

pub fn ex15easy() -> M<15> {
    // svp sqrt(37)
    let b1 = vector![-1, 1, -3, 0, 1, 0, -2, -3, -1, 1, 1, -3, -20, -2, -1].map(|i| i as f64);
    let b2 = vector![0, -1, 0, 2, 1, -1, 1, 0, 127, 3, -5, 0, 2, -5, 1].map(|i| i as f64);
    let b3 = vector![1, -1, 0, -2, 1, 0, -1, 0, 4, -1, 0, 2, 0, -1, 0].map(|i| i as f64);
    let b4 = vector![-1, -6, -1, -2, -1, 0, 0, 0, -1, -5, 1, -3, -68, 1, 3].map(|i| i as f64);
    let b5 = vector![1, 2, 2, -3, -2, 3, 2, 19, -1, 2, -9, 0, 1, 0, -1].map(|i| i as f64);
    let b6 = vector![-8, -1, -5, 0, 3, -2, 1, -1, 8, -5, -1, 1, 0, 1, -1].map(|i| i as f64);
    let b7 = vector![-3, 6, 10, -1, -5, -1, -2, -5, 0, 1, 1, 0, -1, 0, -1].map(|i| i as f64);
    let b8 = vector![6, 0, -3, 1, 8, 1, 12, -2, -2, 1, 1, -1, 1, 4, -1].map(|i| i as f64);
    let b9 = vector![1, 1, 2, -1, 15, 0, -3, 0, 1, 9, -2, -21, -2, 1, 0].map(|i| i as f64);
    let b10 = vector![0, -5, -1, 1, 0, 3, 1, 0, 0, 0, -1, 1, 8, -2, -1].map(|i| i as f64);
    let b11 = vector![-13, 1, 1, 3, -9, 6, 0, -1, 0, 5, -2, 0, -5, 1, 4].map(|i| i as f64);
    let b12 = vector![3, -1, -1, -2, 0, -1, 0, -5, 4, 2, 1, 1, -1, 5, 0].map(|i| i as f64);
    let b13 = vector![2, 1, -2, -3, -4, -9, 1, -4, 0, 1, -1, -1, 4, 0, 0].map(|i| i as f64);
    let b14 = vector![-2, -10, -1, 5, -35, 4, -3, -4, -2, 1, -1, -1, 0, 6, 1].map(|i| i as f64);
    let b15 = vector![25, 0, 1, 1, 1, -1, 2, 1, -1, 1, 32, -42, 2, -1, 2].map(|i| i as f64);
    SMatrix::from_columns(&[
        b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15,
    ])
}

/// large lattice basis for benching
pub fn ex10() -> M<10> {
    // from sage.crypto.lattice import gen_lattice
    // B = gen_lattice(m=10, q=13, n=1, seed=42)
    // svp: (1, 0, 1, 0, 0, 0, 0, 0, 0, 0) norm: sqrt(2)
    let b1 = vector![13, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b2 = vector![4, 1, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b3 = vector![1, 0, 1, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b4 = vector![-3, 0, 0, 1, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b5 = vector![-3, 0, 0, 0, 1, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b6 = vector![-2, 0, 0, 0, 0, 1, 0, 0, 0, 0].map(|i| i as f64);
    let b7 = vector![6, 0, 0, 0, 0, 0, 1, 0, 0, 0].map(|i| i as f64);
    let b8 = vector![-5, 0, 0, 0, 0, 0, 0, 1, 0, 0].map(|i| i as f64);
    let b9 = vector![-2, 0, 0, 0, 0, 0, 0, 0, 1, 0].map(|i| i as f64);
    let b10 = vector![-5, 0, 0, 0, 0, 0, 0, 0, 0, 1].map(|i| i as f64);
    SMatrix::from_columns(&[b1, b2, b3, b4, b5, b6, b7, b8, b9, b10])
}

pub fn ex10hard() -> M<10> {
    // from sage.crypto.lattice import gen_lattice
    // B = gen_lattice(m=10, q=13, seed=42)
    // svp: (0, -1, -1, 0, 0, -1, -1, 0, 1, 1) norm: sqrt(6) det 28561
    let b1 = vector![13, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b2 = vector![0, 13, 0, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b3 = vector![0, 0, 13, 0, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b4 = vector![0, 0, 0, 13, 0, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b5 = vector![4, 1, -3, -3, 1, 0, 0, 0, 0, 0].map(|i| i as f64);
    let b6 = vector![-2, 6, -5, -2, 0, 1, 0, 0, 0, 0].map(|i| i as f64);
    let b7 = vector![-5, 2, -2, -3, 0, 0, 1, 0, 0, 0].map(|i| i as f64);
    let b8 = vector![-5, 0, 6, 6, 0, 0, 0, 1, 0, 0].map(|i| i as f64);
    let b9 = vector![-5, 3, 5, -1, 0, 0, 0, 0, 1, 0].map(|i| i as f64);
    let b10 = vector![-2, 4, 0, -4, 0, 0, 0, 0, 0, 1].map(|i| i as f64);
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
