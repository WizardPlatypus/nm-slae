use matrices::{jacobi, Array2d};

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

    let a = Array2d::gen(n, n, |i, j| gen_a(i, j) as f64);
    let b = Vec::from_iter((0..n).map(gen_b));
    let mut x = vec![0.0; n];

    println!("A = {}", a);
    println!("B = {:?}", b);
    if jacobi::converges_for(&a) {
        println!("Jacobi converges on A");
    } else {
        return;
    }

    jacobi::iterate_on(&mut x, epsilon, &a, &b);
    println!("X = {:?}", x);
}
