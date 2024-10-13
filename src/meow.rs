use crate::Matrix;

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
            self.columns.append((old..new).map(usize::from).collect());
            self.concat.push(snack);
            Ok(())
        } else {
            Err(snack)
        }
    }

    pub fn poop(&mut self) -> Option<M> {
        self.concat.pop()
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

        let index;
        for (i, m) in self.concat.iter().enumerate() {
            if column > m.width() {
                column -= m.width();
            } else {
                index = i;
                break;
            }
        }
        self.concat.get(index)?.at(row, column)
    }

    fn at_mut(&mut self, row: usize, mut column: usize) -> Option<&mut Self::Item> {
        let row = *self.rows.get(row)?;
        let mut column = *self.columns.get(column)?;

        let index;
        for (i, m) in self.concat.iter().enumerate() {
            if column > m.width() {
                column -= m.width();
            } else {
                index = i;
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