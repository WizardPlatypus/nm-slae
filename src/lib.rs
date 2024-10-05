pub mod columns;
pub mod gauss;
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
}
