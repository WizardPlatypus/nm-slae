use crate::{Iteratable, Matrix};

pub fn converges_for<M: Matrix<Item = f64>>(m: &M) -> bool {
    for (i, row) in m.rows().enumerate() {
        let d = m.at(i, i).expect("Diagonal element was not present").abs();
        let sum: f64 = row.cloned().map(f64::abs).sum();
        if d < sum - d {
            return false;
        }
    }
    true
}

pub fn dx(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    std::iter::zip(a, b)
        .map(|(a, b)| (a - b).abs())
        .max_by(f64::total_cmp)
        .expect("There wasn't a single element")
}

pub fn iterate_on<M: Matrix<Item = f64>>(x: &mut [f64], epsilon: f64, m: &M, b: &[f64]) {
    let mut dx = vec![0.0; x.len()];
    loop {
        for i in 0..x.len() {
            let diag = *m.at(i, i).expect("The diagonal element was not present");
            dx[i] = b[i] / diag;
            for (j, &value) in x.iter().enumerate() {
                if j == i {
                    continue;
                }
                dx[i] -= m.at(i, j).unwrap() / diag * value;
            }
            x[i] += dx[i];
        }
        if dx
            .iter()
            .cloned()
            .map(f64::abs)
            .max_by(f64::total_cmp)
            .expect("There wasn't a single elemnt")
            < epsilon
        {
            break;
        }
    }
}
