use crate::Matrix;
use crate::traits::Mapped;
use either::{Either, Left, Right};

pub struct Array2d<T> {
    rows: Either<usize, Vec<usize>>,
    columns: Either<usize, Vec<usize>>,
    data: Vec<T>
}

impl<T: Default + Clone> Array2d<T> {
    pub fn default(height: usize, width: usize) -> Array2d<T> {
        Array2d {
            rows: Left(height),
            columns: Left(width),
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
            rows: Left(height),
            columns: Left(width),
            data
        }
    }

    pub fn try_from(height: usize, width: usize, data: Vec<T>) -> Result<Array2d<T>, Vec<T>> {
        if height * width == data.len() {
            Ok(Array2d {
            rows: Left(height),
            columns: Left(width),
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
    }

    pub fn sync_columns(&mut self, temp: &mut T) {
        for column in 0..self.width() {
            self.sync_column(column, temp);
        }
    }
}

impl<T> Matrix for Array2d<T> {
    type Item = T;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item> {
        let width = self.width();
        let row = self.row(row)?;
        let column = self.column(column)?;
        self.data.get(row * width + column)
    }

    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item> {
        let width = self.width();
        let row = self.row(row)?;
        let column = self.column(column)?;
        self.data.get_mut(row * width + column)
    }

    fn height(&self) -> usize {
        match self.rows.as_ref() {
            Left(height) => *height,
            Right(v) => v.len(),
        }
    }

    fn width(&self) -> usize {
        match self.columns.as_ref() {
            Left(width) => *width,
            Right(v) => v.len(),
        }
    }

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()> {
        let a = self.row(a)?;
        let b = self.row(b)?;
        match self.rows.as_mut() {
            Left(height) => {
                let mut rows: Vec<usize> = (0..*height).map(usize::from).collect();
                rows.swap(a, b);
                self.rows = Right(rows);
            },
            Right(v) => {
                v.swap(a, b);
            },
        }
        Some(())
    }

    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()> {
        let a = self.column(a)?;
        let b = self.column(b)?;
        match self.columns.as_mut() {
            Left(width) => {
                let mut columns: Vec<usize> = (0..*width).map(usize::from).collect();
                columns.swap(a, b);
                self.columns = Right(columns);
            },
            Right(v) => {
                v.swap(a, b);
            },
        }
        Some(())
    }
}

impl<T> Mapped for Array2d<T> {
    type Item = T;

    fn row(&self, index: usize) -> Option<usize> {
        match self.rows.as_ref() {
            Left(_) => Some(index),
            Right(v) => v.get(index).map(Clone::clone),
        }
    }

    fn column(&self, index: usize) -> Option<usize> {
        match self.columns.as_ref() {
            Left(_) => Some(index),
            Right(v) => v.get(index).map(Clone::clone),
        }
    }

    fn cell(&mut self, row: usize, column: usize) -> &mut Self::Item {
        self.at_mut(row, column).expect("Invalid access request from Mapped trait")
    }
    
    fn reset_rows(&mut self, height: usize) {
        self.rows = Left(height);
    }
    
    fn reset_columns(&mut self, width: usize) {
        self.columns = Left(width);
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