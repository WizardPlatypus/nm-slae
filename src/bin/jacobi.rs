use matrices::{jacobi, Array2d, Matrix, Iteratable};

fn gen_a(i: usize, j: usize) -> i64 {
    if i == 0 && j == 0 {
        1
    } else if i == j {
        0
    } else if i > j {
        -((j + 1) as i64)
    } else {
        // if j > i
        (j + 1) as i64
    }
}

fn gen_b(i: usize) -> f64 {
    (i + 1) as f64
}

pub fn main() {
    use std::str::FromStr;
    let mut args = std::env::args();
    let _exec = args.next();
    let n = usize::from_str(&args.next().expect("Missing argument #1: n"))
        .expect("Failed to parse argument #1: n");
    let epsilon = f64::from_str(&args.next().expect("Missing argument #2: epsilon"))
        .expect("Failed to parse argument #2: epsilon");

    let mut a = Array2d::gen(n, n, |i, j| gen_a(i, j) as f64);
    for i in 0..n {
        let sum = a.row(i).cloned().map(f64::abs).sum();
        *a.at_mut(i, i).unwrap() = sum;
    }
    let b = Vec::from_iter((0..n).map(gen_b));
    let mut x = vec![0.0; n];

    println!("A =\n{}", a);
    println!("B = {:?}", b);
    if jacobi::converges_for(&a) {
        println!("Jacobi converges on A");
    } else {
        println!("Jacobi does not converge on A");
        return;
    }

    let iterations = jacobi::iterate_on(&mut x, epsilon, &a, &b);
    println!("X_{iterations} = {:?}", x);
}
