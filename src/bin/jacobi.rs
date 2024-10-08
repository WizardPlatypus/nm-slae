use matrices::{
    jacobi::{self, Jacobi},
    Matrix,
};

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

pub fn main() {
    use std::str::FromStr;
    let n: usize = usize::from_str(&std::env::args().nth(1).expect("Missing argument"))
        .expect("N should be a non negative integer");

    let m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    println!("A =\n{}", &m);
    println!("||A|| = {}", m.norm());
    println!("cond(A) = {}", m.cond());
    let converges = jacobi::converges(&m);
    println!("converges: {:?}", converges);

    let mut g = Jacobi::new(m).unwrap();
    let e = 0.5;
    let solution = g.solve(e);
    for (i, jacobi::Trace { x, precision }) in g.trace().iter().enumerate() {
        println!("{} -> {precision:.2}: {x:.2?}", i + 1);
    }
    println!("{:.2?}", solution);
}
