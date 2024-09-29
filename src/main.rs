use matrices::{gauss::Gauss, Matrix};
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

#[derive(Clone, Debug)]
struct Solution {
    x: Vec<f64>,
    det: f64,
}

fn run(n: usize) -> Solution {
    let m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    let mut g = Gauss::try_from(m).unwrap();
    g.solve();
    let (_, x, det, _) = g.unbox();
    Solution {
        x: x.unwrap(),
        det: det.unwrap(),
    }
}

fn main() {
    let begin = 10;
    let end = 100;

    let (tx, rx) = std::sync::mpsc::channel();
    
    (begin..=end).into_par_iter().for_each(|n| {
	let solution = run(n);
	tx.send((n, solution)).unwrap();
    });
    
    drop(tx);
    
    let mut collected = Vec::with_capacity(end - begin + 1);
    while let Ok(data) = rx.recv() {
	collected.push(data);
    }
    collected.sort_by_key(|(n, _)| *n);

    for (n, s) in collected {
	println!("N = {n}");
	println!("b = ({})", s.x.iter().map(|t| format!("{t:.2}")).collect::<Vec<String>>().join(", "));
	println!("det = {:e}", s.det);
    }
}
