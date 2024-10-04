use rayon::prelude::*;

pub struct Gauss {
    a: Matrix<f64>,
    x: Option<Vec<f64>>,
    det: Option<f64>,
    trace: Vec<State>,
}

pub enum State {
    Created {
        matrix: Matrix<f64>,
    },
    Main {
        iter: usize,
        row: usize,
        column: usize,
        value: f64,
    },
    Swapped {
        iter: usize,
        a: usize,
        b: usize,
        n: usize,
    },
    Modified {
        iter: usize,
        matrix: Matrix<f64>,
    },
    Solved {
        x: Vec<f64>,
        det: f64,
    },
}

impl Gauss {
    fn at(&self, row: usize, col: usize) -> &f64 {
        self.a.at(row, col)
    }

    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        self.a.at_mut(row, col)
    }

    fn log(&mut self, state: State) {
        self.trace.push(state)
    }

    pub fn unbox(self) -> (Matrix<f64>, Option<Vec<f64>>, Option<f64>, Vec<State>) {
        let Gauss { a, x, det, trace } = self;
        (a, x, det, trace)
    }
}

impl Gauss {
    fn modify(&mut self, iter: usize) {
        let first = 1.0 / self.at(iter, iter);
        for col in iter..self.a.cols {
            *self.at_mut(iter, col) *= first;
        }

        for row in (iter + 1)..self.a.rows {
            let first = *self.at(row, iter);
            for col in iter..self.a.cols {
                *self.at_mut(row, col) -= self.at(iter, col) * first;
            }
        }
    }

    fn next(&mut self, iter: usize) {
        let main = self.a.max_in_col(iter, Some(iter));
        self.log(State::Main {
            iter,
            row: main,
            column: iter,
            value: *self.at(main, iter),
        });

        if main != iter {
            self.a.swap_rows(main, iter);
            self.log(State::Swapped {
                iter,
                a: main,
                b: iter,
                n: self.a.height(),
            });
        }

        self.modify(iter);
        /*
            self.log(State::Modified {
                iter,
                matrix: self.a.clone(),
            });
        // */
    }

    fn forward_pass(&mut self) {
        for iter in 0..self.a.height() {
            self.next(iter);
        }
    }

    fn backward_pass(&self) -> Vec<f64> {
        let mut x = vec![0.0; self.a.rows];
        for k in 1..=self.a.rows {
            let i = self.a.rows - k;
            x[i] = *self.at(i, self.a.cols - 1);
            for j in (i + 1)..self.a.rows {
                x[i] -= self.at(i, j);
            }
        }
        x
    }

    pub fn solve(&mut self) {
        self.forward_pass();

        let x = self.backward_pass();
        let det = self.det();

        self.x = Some(x.clone());
        self.det = Some(det);

        self.log(State::Solved { x, det });
    }
}

impl Gauss {
    fn det(&self) -> f64 {
        let mut det = 1.0;
        for state in self.trace.iter() {
            det *= match state {
                State::Main {
                    iter: _,
                    row: _,
                    column: _,
                    value,
                } => *value,
                State::Swapped {
                    iter: _,
                    a: _,
                    b: _,
                    n: _,
                } => -1.0,
                _ => 1.0,
            }
        }
        det
    }
}

impl TryFrom<Matrix<f64>> for Gauss {
    type Error = Matrix<f64>;
    fn try_from(value: Matrix<f64>) -> Result<Self, Self::Error> {
        if value.height() + 1 != value.width() {
            Err(value)
        } else {
            Ok(Gauss {
                a: value.clone(),
                x: None,
                det: None,
                trace: vec![State::Created { matrix: value }],
            })
        }
    }
}

use std::fmt::Write;

impl Report for State {
    fn latex(&self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        match &self {
            Self::Created { matrix } => {
                // writeln!(s, "$A = {}$", matrix.latex()?)?;
                writeln!(s, "$A = \\{{ a _{{ i, j }} | i = \\overline {{ 0..{} }}, j = \\overline {{ 0..{} }} \\}}$", matrix.height(), matrix.width())?;
            }
            Self::Main {
                iter: _,
                row,
                column,
                value,
            } => {
                writeln!(s, "$a _{{ {row}, {column} }} = {value}$")?;
            }
            Self::Swapped { iter, a, b, n: _ } => {
                writeln!(s, "$P _{} = E _{{ {a}, {b} }}$", iter + 1)?;
            }
            Self::Modified { iter, matrix } => {
                writeln!(s, "A _{} = {}", iter + 1, matrix.latex()?)?;
            }
            Self::Solved { x, det } => {
                writeln!(s, "$\\bar {{ x }} = {}$\n", x.latex()?)?;
                writeln!(s, "$\\Delta A = {det:e}$\n")?;
            }
        }

        Ok(s)
    }
}

impl Report for Gauss {
    fn latex(&self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        for state in self.trace.iter() {
            writeln!(s, "{}", state.latex()?)?;
        }
        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new<F: Fn(usize, usize) -> T>(rows: usize, cols: usize, f: F) -> Matrix<T> {
        let mut data = Vec::with_capacity(rows * cols);
        for row in 0..rows {
            for col in 0..cols {
                data.push(f(row, col));
            }
        }
        Matrix { rows, cols, data }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn safe(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn at(&self, row: usize, col: usize) -> &T {
        let index = self.index(row, col);
        &self.data[index]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        let index = self.index(row, col);
        &mut self.data[index]
    }

    pub fn height(&self) -> usize {
        self.rows
    }

    pub fn width(&self) -> usize {
        self.cols
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        for col in 0..self.cols {
            let x = self.index(a, col);
            let y = self.index(b, col);
            self.data.swap(x, y);
        }
    }

    pub fn swap_cols(&mut self, a: usize, b: usize) {
        for row in 0..self.rows {
            let x = self.index(row, a);
            let y = self.index(row, b);
            self.data.swap(x, y);
        }
    }
}

impl Matrix<f64> {
    pub fn e(n: usize) -> Matrix<f64> {
        let data = vec![0.0; n];
        let mut e = Matrix {
            rows: n,
            cols: n,
            data,
        };
        for i in 0..n {
            *e.at_mut(i, i) = 1.0;
        }
        e
    }
}

impl<T: PartialOrd> Matrix<T> {
    pub fn max_in_row(&self, row: usize, skip: Option<usize>) -> usize {
        let mut max = skip.unwrap_or(0);
        for col in (max + 1)..self.cols {
            if self.at(row, col) >= self.at(row, max) {
                max = col;
            }
        }
        max
    }

    pub fn max_in_col(&self, col: usize, skip: Option<usize>) -> usize {
        let mut max = skip.unwrap_or(0);
        for row in (max + 1)..self.rows {
            if self.at(row, col) >= self.at(max, col) {
                max = row;
            }
        }
        max
    }
}

impl<T> FromIterator<T> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data: Vec<T> = iter.into_iter().collect();
        let mut n = 1;
        while n * (n + 1) <= data.len() {
            n += 1;
        }
        Matrix {
            rows: n - 1,
            cols: n,
            data,
        }
    }
}

pub trait Report {
    fn latex(&self) -> Result<String, std::fmt::Error>;
}

impl Report for Matrix<f64> {
    fn latex(&self) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "\\begin{{pmatrix}}")?;
        for row in 0..self.height() {
            let mut values = Vec::with_capacity(self.width());
            for col in 0..self.width() {
                values.push(format!("{:.2}", self.at(row, col)));
            }
            write!(s, "{}", values.join(" & "))?;
            if row + 1 != self.height() {
                write!(s, " \\\\")?;
            }
            writeln!(s)?;
        }
        writeln!(s, "\\end{{pmatrix}}")?;

        Ok(s)
    }
}

impl Report for Vec<f64> {
    fn latex(&self) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "\\begin{{pmatrix}}")?;
        for value in self {
            writeln!(s, "{:.2} \\\\", value)?;
        }
        writeln!(s, "\\end{{pmatrix}}")?;

        Ok(s)
    }
}

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

    println!(r#"\documentclass[a4paper,12pt]{{article}}"#);
    println!(r#"\usepackage{{amsmath}}"#);
    println!(r#"\begin{{document}}"#);
    for (n, s) in collected {
        println!("\\section{{ $N = {n}$ }}");
        println!("{s}");
    }
    println!(r#"\end{{document}}"#);
}
