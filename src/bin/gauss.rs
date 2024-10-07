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

    let m = Matrix::new(n, n + 1, |i, j| my_matrix(n, i, j) as f64);
    let mut g = Gauss::try_from(m.clone()).unwrap();
    g.solve();
    let (_m, _x, _det, trace) = g.unbox();
    for state in trace {
        match state {
            State::Created { matrix } => {
		println!("A =\n{:.2}", matrix);
	    },
	    State::Main { iter, row, column, value } => {
		println!("a{iter} = A[{row},{column}] = {value:.2}");
	    },
	    State::Swapped { iter, a, b, n: _n } => {
		println!("P{iter} = E{{{a}, {b}}}");
	    },
	    State::Modified { iter, matrix } => {
		println!("A{iter} =\n{:.2}", matrix);
	    },
	    State::Solved { x, det } => {
		println!("X = {:.2?}, det = {:.2}", x, det);
	    },
        }
    }
}
