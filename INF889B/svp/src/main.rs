use std::time::Instant;

fn main() {
    // let bounds = vec![2, 3, 4];
    // let mut v = vec![0, 0, 0];

    // while v.last() != bounds.last() {
    //     println!("{:?}", v);
    //     v[0] += 1;
    //     for i in 0..v.len()-1 {
    //         if v[i] == bounds[i] {
    //             v[i] = 0;
    //             v[i+1] += 1;
    //         }
    //     }
    // }

    let now = Instant::now();
    let mut basis = svp::data::ex10hard();
    println!("basis of lattice: {}", basis);

    let v = svp::brute_force(&basis, true, true).unwrap();
    println!("shortest vector of lattice: {}, norm = {}", v, v.norm());

    println!("Elapsed {:?}", now.elapsed());

    basis = svp::lll(basis, 0.75);
    println!("lll reduced basis: {}", basis);

    // println!("λ = {}B = {}a = {}", v, basis, cs);
    // println!("len λ = {}", v.norm());

    // // batching computes n points in the lattice from n coefficient vectors using a single matrix multiplication
    // let a1 = vector![0.0, 0.0, 0.0];
    // let a2 = vector![1.0, 1.0, 1.0];
    // let a3 = vector![2.0, 2.0, 2.0];
    // let coeffs = Matrix3::from_columns(&[a1, a2, a3]);
    // println!("C = BA");
    // println!("C = {}B = {}A = {}", basis * coeffs, basis, coeffs);
}
