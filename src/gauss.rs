use super::{Matrix};

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
        for col in iter..self.a.width {
            *self.at_mut(iter, col) *= first;
        }

        for row in (iter + 1)..self.a.height {
            let first = *self.at(row, iter);
            for col in iter..self.a.width {
                *self.at_mut(row, col) -= self.at(iter, col) * first;
            }
        }
    }

    fn next(&mut self, iter: usize) {
        let main = self
            .a
            .column(iter)
            .enumerate()
            .skip(iter)
            .max_by(|&(_, a), &(_, b)| f64::total_cmp(a, b));
        if main.is_none() {
            return;
        }
        let (main, &value) = main.unwrap();

        self.log(State::Main {
            iter,
            row: main,
            column: iter,
            value,
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
        let mut x = vec![0.0; self.a.height];
        for k in 1..=self.a.height {
            let i = self.a.height - k;
            x[i] = *self.at(i, self.a.width - 1);
            for j in (i + 1)..self.a.height {
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
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Created { matrix } => {
                writeln!(f, "A =\n{:.2}", matrix)?;
            }
            State::Main {
                iter,
                row,
                column,
                value,
            } => {
                writeln!(f, "a{iter} = A[{row},{column}] = {value:.2}")?;
            }
            State::Swapped { iter, a, b, n: _n } => {
                writeln!(f, "P{iter} = E{{{a}, {b}}}")?;
            }
            State::Modified { iter, matrix } => {
                writeln!(f, "A{iter} =\n{:.2}", matrix)?;
            }
            State::Solved { x, det } => {
                writeln!(f, "X = {:.2?}, det = {:.2}", x, det)?;
            }
        }
        Ok(())
    }
}
