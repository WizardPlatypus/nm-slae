use crate::{traits::Mapped, Matrix};
use either::{Either, Left, Right};

#[derive(Clone)]
pub struct Meow<T> {
    rows: Either<usize, Vec<usize>>,
    columns: Either<usize, Vec<usize>>,
    concat: Vec<T>,
}

impl<T> Meow<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        self.concat.get(index)
    }
}

impl<M: Matrix<Item = T>, T> Meow<M> {
    pub fn eat(&mut self, snack: M) -> Result<(), M> {
        if self.height() == snack.height() {
            let old = self.width();
            let new = old + snack.width();
            match self.columns.as_mut() {
                Left(width) => *width = new,
                Right(v) => v.append(&mut (old..new).map(usize::from).collect()),
            }
            self.concat.push(snack);
            Ok(())
        } else {
            Err(snack)
        }
    }

    pub fn poop(&mut self, potty: &mut T) -> Option<M> {
        if self.concat.is_empty() {
            return None;
        }

        self.sync_columns(potty);
        self.sync_rows(potty);

        let poop = self.concat.pop().expect("Somehow concat is empty");

        match self.columns.as_mut() {
            Left(width) => {
                *width -= poop.width();
            }
            Right(v) => {
                v.drain((v.len() - poop.width())..);
            }
        };

        self.reset_columns(self.width());
        self.reset_rows(self.height());

        Some(poop)
    }

    pub fn extract(&mut self, index: usize, potty: &mut T) -> Option<M> {
        if index >= self.concat.len() {
            return None;
        }

        self.sync_columns(potty);
        self.sync_rows(potty);

        let poop = self.concat.remove(index);

        let mut before = 0;
        for i in 0..index {
            before += self.concat[i].width();
        }

        match self.columns.as_mut() {
            Left(width) => {
                *width -= poop.width();
            }
            Right(v) => {
                v.drain(before..=(before + poop.width()));
            }
        };

        self.reset_columns(self.width());
        self.reset_rows(self.height());

        Some(poop)
    }

    pub fn sync_rows(&mut self, temp: &mut T) {
        if self.rows.is_right() {
            for row in 0..self.height() {
                self.sync_row(row, temp);
            }
        }
    }

    pub fn sync_columns(&mut self, temp: &mut T) {
        if self.columns.is_right() {
            for column in 0..self.width() {
                self.sync_column(column, temp);
            }
        }
    }
}

impl<M: Matrix<Item = T>, T> Mapped for Meow<M> {
    type Item = T;

    fn row(&self, index: usize) -> Option<usize> {
        match self.rows.as_ref() {
            Left(_) => Some(index),
            Right(v) => v.get(index).cloned(),
        }
    }

    fn column(&self, index: usize) -> Option<usize> {
        match self.columns.as_ref() {
            Left(_) => Some(index),
            Right(v) => v.get(index).cloned(),
        }
    }

    fn cell(&mut self, row: usize, column: usize) -> &mut Self::Item {
        self.at_mut(row, column)
            .expect("Invalid access request from Mapped trait")
    }

    fn reset_rows(&mut self, height: usize) {
        self.rows = Left(height);
    }

    fn reset_columns(&mut self, width: usize) {
        self.columns = Left(width);
    }
}

impl<M: Matrix<Item = T>, T> From<M> for Meow<M> {
    fn from(value: M) -> Self {
        Meow {
            rows: Left(value.height()),
            columns: Left(value.width()),
            concat: vec![value],
        }
    }
}

impl<M: Matrix<Item = T>, T> Matrix for Meow<M> {
    type Item = T;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item> {
        let row = self.row(row)?;
        let mut column = self.column(column)?;

        let mut index = 0;
        for m in self.concat.iter() {
            if column > m.width() {
                column -= m.width();
                index += 1;
            } else {
                break;
            }
        }
        self.concat.get(index)?.at(row, column)
    }

    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item> {
        let row = self.row(row)?;
        let mut column = self.column(column)?;

        let mut index = 0;
        for m in self.concat.iter() {
            if column > m.width() {
                column -= m.width();
                index += 1;
            } else {
                break;
            }
        }
        self.concat.get_mut(index)?.at_mut(row, column)
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
            }
            Right(v) => {
                v.swap(a, b);
            }
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
            }
            Right(v) => {
                v.swap(a, b);
            }
        }
        Some(())
    }
}

impl<M: Matrix<Item = f64>> std::fmt::Display for Meow<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let widths: Vec<usize> = self.concat.iter().map(Matrix::width).collect();
        for row in 0..self.height() {
            write!(f, "|")?;
            let mut total = 0;
            for &width in widths.iter() {
                for i in 0..width {
                    write!(f, " {:.2}", self.at(row, total + i).unwrap())?;
                }
                write!(f, " |")?;
                total += width;
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}
