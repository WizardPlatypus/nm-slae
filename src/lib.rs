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

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(&self.data[self.mapped(row, col)])
        }
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

pub trait Gauss {
    fn max_in_row(&self, row: usize) -> usize;
    fn max_in_column(&self, col: usize) -> usize;
    fn modify(&mut self, step: usize);
    fn reverse(&self) -> Vec<f64>;
    fn solve(&mut self) -> f64;
}

impl Gauss for Matrix<f64> {
    fn max_in_row(&self, row: usize) -> usize {
        self.max_in_row(row, Some(row))
    }

    fn max_in_column(&self, col: usize) -> usize {
        self.max_in_col(col, Some(col))
    }

    fn modify(&mut self, step: usize) {
        let first = 1.0 / self[(step, step)];
        for col in step..self.cols {
            self[(step, col)] *= first;
        }

        for row in (step + 1)..self.rows {
            let first = self[(row, step)];
            for col in step..self.cols {
                self[(row, col)] -= self[(step, col)] * first;
            }
        }
    }

    fn reverse(&self) -> Vec<f64> {
        let mut x = vec![0.0; self.rows];
        for k in 1..=self.rows {
            let i = self.rows - k;
            x[i] = self[(i, self.cols - 1)];
            for j in (i + 1)..self.rows {
                x[i] -= self[(i, j)];
            }
        }
        x
    }

    fn solve(&mut self) -> f64 {
        let mut det = 1.0;
        for step in 0..self.rows {
	    log::debug!("Step {step}");
            let main = self.max_in_column(step);
	    log::debug!("Main ({main}, {step}) = {}", self[(main, step)]);
            det *= self[(main, step)];
            if main != step {
		log::debug!("Swapping rows {main} and {step}");
                self.swap_rows(main, step);
		log::debug!("After swap:");
		log::debug!("\n{:?}", self);
                det *= -1.0;
            }
	    log::debug!("Applying M{step}");
            self.modify(step);
	    log::debug!("After M{step}:");
	    log::debug!("\n{:?}", self);
        }
        det
    }
}
