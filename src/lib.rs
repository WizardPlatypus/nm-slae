pub mod gauss;

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

    fn mapped(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn safe(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn height(&self) -> usize {
        self.rows
    }

    pub fn width(&self) -> usize {
        self.cols
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        for col in 0..self.cols {
            let x = self.mapped(a, col);
            let y = self.mapped(b, col);
            self.data.swap(x, y);
        }
    }

    pub fn swap_cols(&mut self, a: usize, b: usize) {
        for row in 0..self.rows {
            let x = self.mapped(row, a);
            let y = self.mapped(row, b);
            self.data.swap(x, y);
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let max = self
            .data
            .iter()
            .map(|t| format!("{t:?}").len())
            .max()
            .unwrap();

        for row in 0..self.rows {
            f.write_char('|')?;
            for col in 0..self.cols {
                write!(f, " {:>width$?}", self[(row, col)], width = max)?;
            }
            f.write_str(" |\n")?;
        }
        Ok(())
    }
}

impl<T: PartialOrd> Matrix<T> {
    pub fn max_in_row(&self, row: usize, skip: Option<usize>) -> usize {
        let mut max = skip.unwrap_or(0);
        for col in (max + 1)..self.cols {
            if self[(row, col)] >= self[(row, max)] {
                max = col;
            }
        }
        max
    }

    pub fn max_in_col(&self, col: usize, skip: Option<usize>) -> usize {
        let mut max = skip.unwrap_or(0);
        for row in (max + 1)..self.rows {
            if self[(row, col)] >= self[(max, col)] {
                max = row;
            }
        }
        max
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[self.mapped(row, col)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let mapped = self.mapped(row, col);
        &mut self.data[mapped]
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
