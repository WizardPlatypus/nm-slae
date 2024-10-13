pub trait Matrix {
    type Item;

    fn at(&self, row: usize, column: usize) -> Option<&Self::Item>;
    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item>;

    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn swap_rows(&mut self, a: usize, b: usize) -> Option<()>;
    fn swap_columns(&mut self, a: usize, b: usize) -> Option<()>;
}

pub trait Mapped {
    type Item;

    fn row(&self, index: usize) -> usize;
    fn column(&self, index: usize) -> usize;
    fn cell(&mut self, row: usize, column: usize) -> &mut Self::Item;

    fn sync_row(&mut self, row: usize, temp: &mut Self::Item) {
        let mut cursor = 0;
        loop {
            std::mem::swap(self.cell(row, cursor), temp);
            cursor = self.column(cursor);
            if cursor == 0 {
                std::mem::swap(self.cell(row, cursor), temp);
                break;
            }
        }
    }

    fn sync_column(&mut self, column: usize, temp: &mut Self::Item) {
        let mut cursor = 0;
        loop {
            std::mem::swap(self.cell(cursor, column), temp);
            cursor = self.row(cursor);
            if cursor == 0 {
                std::mem::swap(self.cell(cursor, column), temp);
                break;
            }
        }
    }
}