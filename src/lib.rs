pub mod gauss;

#[derive(Debug)]
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
