use crate::Matrix;
use crate::traits::Mapped;

pub struct Array2d<T> {
    rows: Vec<usize>,
    columns: Vec<usize>,
    data: Vec<T>
}

impl<T: Default + Clone> Array2d<T> {
    pub fn default(height: usize, width: usize) -> Array2d<T> {
        Array2d {
            rows: (0..height).map(usize::from).collect(),
            columns: (0..width).map(usize::from).collect(),
            data: vec![T::default(); height * width]
        }
    }
}

impl<T> Array2d<T> {
    pub fn gen<F: Fn(usize, usize) -> T>(height: usize, width: usize, f: F) -> Array2d<T> {
        let mut data = Vec::with_capacity(height * width);
        for i in 0..height {
            for j in 0..width {
                data.push(f(i, j));
            }
        }
        Array2d {
            rows: (0..height).map(usize::from).collect(),
            columns: (0..width).map(usize::from).collect(),
            data
        }
    }

    pub fn try_from(height: usize, width: usize, data: Vec<T>) -> Result<Array2d<T>, Vec<T>> {
        if height * width == data.len() {
            Ok(Array2d {
            rows: (0..height).map(usize::from).collect(),
            columns: (0..width).map(usize::from).collect(),
            data
            })
        } else {
            Err(data)
        }
    }

    pub fn sync_rows(&mut self, temp: &mut T) {
        for row in 0..self.height() {
            self.sync_row(row, temp);
        }
        for i in 0..self.width() {
            self.rows[i] = i;
        }
    }

    pub fn sync_columns(&mut self, temp: &mut T) {
        for column in 0..self.width() {
            self.sync_column(column, temp);
        }
        for i in 0..self.height() {
            self.columns[i] = i;
        }
    }
}

impl<T> crate::Matrix for Array2d<T> {
    type Item = T;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item> {
        self.data.get(self.rows.get(row)? * self.width() + self.columns.get(column)?)
    }

    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item> {
        let width = self.width();
        self.data.get_mut(self.rows.get(row)? * width + self.columns.get(column)?)
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.columns.len()
    }

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()> {
        let a = *self.rows.get(a)?;
        let b = *self.rows.get(b)?;
        self.rows.swap(a, b);
        Some(())
    }

    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()> {
        let a = *self.columns.get(a)?;
        let b = *self.columns.get(b)?;
        self.columns.swap(a, b);
        Some(())
    }
}

impl<T> Mapped for Array2d<T> {
    type Item = T;

    fn row(&self, index: usize) -> usize {
        self.rows[index]
    }

    fn column(&self, index: usize) -> usize {
        self.columns[index]
    }

    fn cell(&mut self, row: usize, column: usize) -> &mut Self::Item {
        use crate::Matrix;
        self.at_mut(row, column).expect("Invalid access request from Mapped trait")
    }
}


impl std::fmt::Display for Array2d<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::Iteratable;
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