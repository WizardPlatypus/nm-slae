pub mod columns;
pub mod gauss;
pub mod jacobi;
pub mod rows;

pub use columns::*;
pub use rows::*;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
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

    pub fn row(&self, row: usize) -> Row<'_, T> {
        Row::new(&self.data, self.width(), row)
    }

    pub fn column(&self, column: usize) -> Column<'_, T> {
        Column::new(&self.data, self.width(), self.height(), column)
    }

    pub fn rows(&self) -> Rows<'_, T> {
        Rows::new(&self.data, self.width(), self.height())
    }

    pub fn columns(&self) -> Columns<'_, T> {
        Columns::new(&self.data, self.width(), self.height())
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

impl<T: Clone> Matrix<T> {
    pub fn append_right(&self, other: &Matrix<T>) -> Result<Matrix<T>, String> {
        if self.height() != other.height() {
            return Err(format!(
                "Expected height {}, found {}",
                self.height(),
                other.height()
            ));
        }
        let width = self.width() + other.width();
        let height = self.height();
        let gen = |i, j| {
            if j < self.width() {
                self.at(i, j).clone()
            } else {
                other.at(i, j - self.width()).clone()
            }
        };
        Ok(Matrix::new(height, width, gen))
    }

    pub fn window(
        &self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) -> Result<Matrix<T>, String> {
        let gen = |i, j| self.at(row + i, col + j).clone();
        if row + height <= self.height() && col + width <= self.width() {
            Ok(Matrix::new(height, width, gen))
        } else {
            Err(format!(
                "({} + {}, {} + {}) does not fit in ({}, {})",
                row,
                height,
                col,
                width,
                self.height(),
                self.width()
            ))
        }
    }

    pub fn transposed(&self) -> Matrix<T> {
        let height = self.width();
        let width = self.height();

        let gen = |i, j| self.at(j, i).clone();

        Matrix::new(height, width, gen)
    }
}

impl Matrix<f64> {
    pub fn e(n: usize) -> Matrix<f64> {
        let data = vec![0.0; n * n];
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

    pub fn inverse(&self) -> Result<Matrix<f64>, String> {
        let e = Matrix::e(self.height());
        let mut m = self.append_right(&e).unwrap();
        for iter in 0..m.height() {
            let (main, _) = m
                .column(iter)
                .enumerate()
                .skip(iter)
                .max_by(|&(_, a), &(_, b)| f64::total_cmp(a, b))
                .unwrap();

            if main != iter {
                m.swap_rows(main, iter);
            }

            let first = 1.0 / m.at(iter, iter);
            for col in iter..m.width() {
                *m.at_mut(iter, col) *= first;
            }

            for row in (iter + 1)..m.height() {
                let first = *m.at(row, iter);
                for col in iter..m.width() {
                    *m.at_mut(row, col) -= m.at(iter, col) * first;
                }
            }
        }
        for iter in 0..m.height() {
            let first = 1.0 / m.at(m.height() - iter - 1, m.height() - iter - 1);
            for col in iter..m.width() {
                *m.at_mut(m.height() - iter - 1, col) *= first;
            }

            for row in (iter + 1)..m.height() {
                let first = *m.at(m.height() - row - 1, m.height() - iter - 1);
                for col in iter..m.width() {
                    *m.at_mut(m.height() - row - 1, col) -=
                        m.at(m.height() - iter - 1, col) * first;
                }
            }
        }
        m.window(0, m.height(), m.height(), m.height())
    }

    pub fn cond(&self) -> f64 {
        let me = self.norm();
        let inv = self.inverse().unwrap().norm();
        me * inv
    }

    pub fn norm(&self) -> f64 {
        self.rows()
            .map(|row| row.map(Clone::clone).map(f64::abs).sum())
            .max_by(f64::total_cmp)
            .expect("There was less than one element")
    }
}

impl<T> Matrix<T> {
    pub fn try_from_iter<I: IntoIterator<Item = T>>(
        iter: I,
        height: usize,
        width: usize,
    ) -> Result<Matrix<T>, usize> {
        let data: Vec<T> = iter.into_iter().collect();
        let expected = width * height;
        if data.len() < expected {
            Err(expected - data.len())
        } else {
            Ok(Matrix {
                data,
                rows: height,
                cols: width,
            })
        }
    }

    pub fn new<F: Fn(usize, usize) -> T>(rows: usize, cols: usize, f: F) -> Matrix<T> {
        let mut data = Vec::with_capacity(rows * cols);
        for row in 0..rows {
            for col in 0..cols {
                data.push(f(row, col));
            }
        }
        Matrix { rows, cols, data }
    }
}

impl std::fmt::Display for Matrix<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            write!(f, "|")?;
            for cell in row {
                write!(f, " {:.2}", cell)?;
            }
            writeln!(f, " |")?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn gen(i: usize, j: usize) -> (usize, usize) {
        (i, j)
    }

    #[test]
    fn rows_ok() {
        let size = 5;
        let m = Matrix::new(size, size, gen);
        for (row_id, row) in m.rows().enumerate() {
            for (column_id, &(i, j)) in row.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn columns_ok() {
        let size = 5;
        let m = Matrix::new(size, size, gen);
        for (column_id, column) in m.columns().enumerate() {
            for (row_id, &(i, j)) in column.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn inverse_ok() {
        let m = Matrix::try_from_iter(
            [1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0]
                .iter()
                .map(std::clone::Clone::clone),
            3,
            3,
        )
        .unwrap();
        let inv = m.inverse().unwrap();
        let expected = Matrix::try_from_iter(
            [-0.375, 0.5, 0.125, 0.5, -1.0, 0.5, 0.125, 0.5, -0.375],
            3,
            3,
        )
        .unwrap();
        assert_eq!(inv.height(), expected.height());
        assert_eq!(inv.width(), expected.width());
        println!("{:?}", inv.data);
        println!("{:?}", expected.data);
        for i in 0..3 {
            for j in 0..3 {
                assert!((inv.at(i, j) - expected.at(i, j)).abs() < 10e-6);
            }
        }
    }

    #[test]
    fn transposed_ok() {
        let a = Matrix::try_from_iter([1, 2, 3, 4, 5, 6, 7, 8].iter(), 2, 4).unwrap();
        let b = Matrix::try_from_iter([1, 5, 2, 6, 3, 7, 4, 8].iter(), 4, 2).unwrap();
        assert_eq!(a.data, b.transposed().data);
        assert_eq!(b.data, a.transposed().data);
    }
}
