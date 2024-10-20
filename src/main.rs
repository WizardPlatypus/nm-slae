#![allow(unused)]
#![allow(dead_code)]

use matrices::{Meow, gauss, Array2d, Iteratable, Matrix};
use rayon::prelude::*;

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


fn main() {
    let begin = 10;
    let end = 100;

    let (tx, rx) = std::sync::mpsc::channel();

    (begin..=end).into_par_iter().for_each(|n| {
        let a = Array2d::gen(n, n, |i, j| gen_a(i, j) as f64);
        let b = Array2d::gen(n, 1, |i, _| gen_b(i));
        let e = Array2d::gen(n, n, |i, j| if i == j { 1.0 } else { 0.0 });

        let norm = matrices::inf_norm(&a);

        let mut m = Meow::from(a);
        m.eat(b).expect("Failed to consume B");
        m.eat(e).expect("Failed to consume E");

        gauss::calc_l(&mut m);
        gauss::calc_u(&mut m);
        let det = matrices::multiply_diagonal(&m);
        gauss::normalize(&mut m);

        let inverse = m.calculate(2).unwrap();
        let x: Vec<f64> = m.calculate(1).unwrap().column(0).cloned().collect();

        let inverse_norm = matrices::inf_norm(&inverse);
        let cond = norm * inverse_norm;

        tx.send((n, x, cond)).unwrap();
    });

    drop(tx);

    let mut collected = Vec::with_capacity(end - begin + 1);
    while let Ok(data) = rx.recv() {
        collected.push(data);
    }
    collected.sort_by_key(|(n, _, _)| *n);

    for (i, x, c) in collected {
        println!("{} | {:.2} | {:?}", i, c, x);
    }
    // */
}
