#![allow(unused)]
#![allow(dead_code)]

use matrices::{Array2d, Iteratable, Matrix};
use rayon::prelude::*;

fn my_matrix(n: usize, i: usize, j: usize) -> i64 {
    if i == 0 && j == 0 {
        1
    } else if i == j {
        0
    } else if j == n {
        (i + 1) as i64
    } else if i > j {
        -((j + 1) as i64)
    } else {
        // if j > i
        (j + 1) as i64
    }
}

fn main() {
    /*
    let begin = 100;
    let end = 200;

    let (tx, rx) = std::sync::mpsc::channel();

    (begin..=end).into_par_iter().for_each(|n| {
        let report = report(n);
        tx.send((n, report)).unwrap();
    });

    drop(tx);

    let mut collected = Vec::with_capacity(end - begin + 1);
    while let Ok(data) = rx.recv() {
        collected.push(data);
    }
    collected.sort_by_key(|(n, _)| *n);
    // */
}
