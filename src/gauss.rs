use super::{Matrix, Report};

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
        self.log(State::Modified {
            iter,
            matrix: self.a.clone(),
        });
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
                writeln!(s, "A = {}", matrix.latex()?)?;
            }
            Self::Main {
                iter,
                row,
                column,
                value,
            } => {
                writeln!(s, "a _{{ {row}, {column} }} = \\underset {{ i }} {{ \\max |a _{{ i, {column} }} ^{{ ({} - 1) }} | }} = {value}", iter + 1)?;
            }
            Self::Swapped { iter, a, b, n } => {
                let mut p = Matrix::e(*n);
                p.swap_rows(*a, *b);
                writeln!(s, "P _{} = {}", iter + 1, p.latex()?)?;
            }
            Self::Modified { iter, matrix } => {
                writeln!(s, "A _{} = {}", iter + 1, matrix.latex()?)?;
            }
            Self::Solved { x, det } => {
                let mut values = Vec::with_capacity(x.len());
                for value in x {
                    values.push(format!("{:.2}", value));
                }
                writeln!(s, "\\hbar {{ x }} = ({})", values.join(", "))?;
                writeln!(s, "\\Delta A = {det}")?;
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
