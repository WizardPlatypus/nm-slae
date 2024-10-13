use crate::Matrix;
use crate::traits::Mapped;

pub struct Meow<T> {
    rows: Vec<usize>,
    columns: Vec<usize>,
    concat: Vec<T>
}

impl<T> Meow<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        self.concat.get(index)
    }
}

impl<M: Matrix<Item=T>, T> Meow<M> {
    pub fn eat(&mut self, snack: M) -> Result<(), M> {
        if self.height() == snack.height() {
            let old = self.width();
            let new = old + snack.width();
            self.columns.append(&mut (old..new).map(usize::from).collect());
            self.concat.push(snack);
            Ok(())
        } else {
            Err(snack)
        }
    }

    pub fn poop(&mut self, potty: &mut T) -> Option<M> {
        self.sync_columns(potty);
        self.sync_rows(potty);
        let poop = self.concat.pop();
        self.columns.drain(self.width()..);
        poop
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

impl<M: Matrix<Item=T>, T> Mapped for Meow<M> {
    type Item = T;

    fn row(&self, index: usize) -> usize {
        self.rows[index]
    }

    fn column(&self, index: usize) -> usize {
        self.columns[index]
    }

    fn cell(&mut self, row: usize, column: usize) -> &mut Self::Item {
        self.at_mut(row, column).expect("Invalid access request from Mapped trait")
    }
}

impl<M: Matrix<Item=T>, T> From<M> for Meow<M> {
    fn from(value: M) -> Self {
        Meow {
            rows: (0..value.height()).map(usize::from).collect(),
            columns: (0..value.width()).map(usize::from).collect(),
            concat: vec![value]
        }
    }
}

impl<M: Matrix<Item=T>, T> Matrix for Meow<M> {
    type Item = T;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item> {
        let row = *self.rows.get(row)?;
        let mut column = *self.columns.get(column)?;

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
        let row = *self.rows.get(row)?;
        let mut column = *self.columns.get(column)?;

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
        self.concat.first().map(Matrix::height).unwrap_or(0)
    }

    fn width(&self) -> usize {
        self.columns.len()
    }

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()> {
        let a = *self.rows.get(a)?;
        let b = *self.rows.get(b)?;
        self.columns.swap(a, b);
        Some(())
    }

    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()> {
        let a = *self.columns.get(a)?;
        let b = *self.columns.get(b)?;
        self.columns.swap(a, b);
        Some(())
    }
}