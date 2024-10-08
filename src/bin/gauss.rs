use matrices::{gauss::*, Matrix};

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
    use std::str::FromStr;
    let n: usize = usize::from_str(&std::env::args().nth(1).expect("Missing argument"))
        .expect("N should be a non negative integer");

    //let example = Matrix::try_from_iter([7.6, 0.5, 2.4, 1.9, 2.2, 9.1, 4.4, 9.7, -1.3, 0.2, 5.8, -1.4], 3, 4);
    let m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    let inv = m.inverse().unwrap();
    println!("A-1 =\n{:.2}", inv);
    println!("||A-1|| = {:.2}", inv.norm());

    println!("A = {:.2}", m);
    println!("||A|| = {:.2}", m.norm());

    println!("cond(A) = {:.2}", m.cond());
    let mut g = Gauss::try_from(m.clone()).unwrap();
    g.solve();
    let (_m, _x, _det, trace) = g.unbox();
    for state in trace {
        println!("{state}");
    }
}
