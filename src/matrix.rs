pub trait Matrix {
    type Item;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item>;
    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item>;

    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()>;
    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()>;
}
