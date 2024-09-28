use matrices::*;

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
    pretty_env_logger::init();

    let n = 5;
    let mut m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    log::debug!("\n{:?}", m);

    let det = <Matrix<f64> as Gauss>::solve(&mut m);
    let x = <Matrix<f64> as Gauss>::reverse(&m);
    log::debug!("\n{:?}", m);
    log::debug!("{:?}", det);
    log::debug!("{:?}", x);
}
