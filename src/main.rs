use matrices::{gauss::Gauss, Matrix, Report};
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

fn report(n: usize) -> String {
    let m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    let mut g = Gauss::try_from(m).unwrap();
    g.solve();
    g.latex().unwrap()
}

fn main() {
    let begin = 10;
    let end = 100;

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

    println!(r#"\documentclass[a4paper,12pt]{{article}}"#);
    println!(r#"\usepackage{{amsmath}}"#);
    println!(r#"\begin{{document}}"#);
    for (n, s) in collected {
	println!("\\section{{ $N = {n}$ }}");
	println!("{s}");
    }
    println!(r#"\end{{document}}"#);
}
