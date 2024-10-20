use crate::Matrix;

pub struct Transposed<T> {
    origin: T,
}

impl<M: Matrix<Item = T>, T> Matrix for Transposed<M> {
    type Item = T;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item> {
        self.origin.at(column, row)
    }

    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item> {
        self.origin.at_mut(column, row)
    }

    fn height(&self) -> usize {
        self.origin.width()
    }

    fn width(&self) -> usize {
        self.origin.height()
    }

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()> {
        self.origin.swap_columns(a, b)
    }

    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()> {
        self.origin.swap_rows(a, b)
    }
}
