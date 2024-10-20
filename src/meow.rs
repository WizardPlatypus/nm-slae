use crate::{traits::Mapped, Array2d, Matrix};
use either::{Either, Left, Right};

#[derive(Clone)]
pub struct Meow<T> {
    rows: Either<usize, Vec<usize>>,
    columns: Either<usize, Vec<usize>>,
    concat: Vec<T>,
}

#[allow(dead_code)]
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

    // does not work correctly
    // sync_columns/rows does not properly sync
    fn poop(&mut self, potty: &mut T) -> Option<M> {
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

    // does not work correctly
    // sync_columns/rows does not properly sync
    fn extract(&mut self, index: usize, potty: &mut T) -> Option<M> {
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

    fn sync_rows(&mut self, temp: &mut T) {
        if self.rows.is_right() {
            for row in 0..self.height() {
                self.sync_column(row, temp);
            }
        }
    }

    fn sync_columns(&mut self, temp: &mut T) {
        if self.columns.is_right() {
            for column in 0..self.width() {
                self.sync_row(column, temp);
            }
        }
    }
}

impl<T: Clone> Meow<Array2d<T>> {
    pub fn calculate(&self, index: usize) -> Option<Array2d<T>> {
        if index >= self.concat.len() {
            None
        } else {
            let total: usize = self.concat.iter().take(index).map(Matrix::width).sum();
            let a = Array2d::gen(
                self.concat[index].height(),
                self.concat[index].width(),
                |i, j| self.at(i, total + j).unwrap().clone(),
            );
            Some(a)
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
        let mut col = self.column(column)?;

        let mut index = 0;
        for m in self.concat.iter() {
            if col >= m.width() {
                col -= m.width();
                index += 1;
            } else {
                break;
            }
        }
        self.concat.get(index)?.at(row, col)
    }

    fn at_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Item> {
        let row = self.row(row)?;
        let mut col = self.column(column)?;

        let mut index = 0;
        for m in self.concat.iter() {
            let w = m.width();
            if col >= w {
                col -= w;
                index += 1;
            } else {
                break;
            }
        }
        self.concat.get_mut(index)?.at_mut(row, col)
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
        // let a = self.row(a)?;
        // let b = self.row(b)?;
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
        // let a = self.column(a)?;
        // let b = self.column(b)?;
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
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Array2d, Iteratable, Matrix, Meow};

    #[test]
    fn rows_access_ok() {
        let a = Array2d::gen(3, 3, |i, j| (i, j));
        let b = Array2d::gen(3, 3, |i, j| (i, j + 3));
        let mut m = Meow::from(a);
        m.eat(b).unwrap();

        for (row_id, row) in m.rows().enumerate() {
            println!("{} {:?}", row_id, row.size_hint());
            for (column_id, &(i, j)) in row.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn columns_access_ok() {
        let a = Array2d::gen(3, 3, |i, j| (i, j));
        let b = Array2d::gen(3, 3, |i, j| (i, j + 3));
        let mut m = Meow::from(a);
        m.eat(b).unwrap();

        for (column_id, column) in m.columns().enumerate() {
            for (row_id, &(i, j)) in column.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn swap_rows_ok() {
        let size = 5;

        let mut m = Array2d::gen(size, size, |i, j| (i, j));

        m.swap_rows(0, 2);
        let m21034 = Array2d::try_from(
            size,
            size,
            vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m21034");
        assert_eq!(m, m21034, "Row swap #0 (0, 2) failed");

        m.swap_rows(2, 3);
        let m21304 = Array2d::try_from(
            size,
            size,
            vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m21304");
        assert_eq!(m, m21304, "Row swap #1 (2, 3) failed");

        m.swap_rows(0, 2);
        let m31204 = Array2d::try_from(
            size,
            size,
            vec![
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m31204");
        assert_eq!(m, m31204, "Row swap #2 (0, 2) failed");

        m.swap_rows(0, 3);
        let m01234 = Array2d::gen(size, size, |i, j| (i, j));
        assert_eq!(m, m01234, "Row swap #3 (0, 3) failed");
    }

    #[test]
    fn swap_columns_ok() {
        let size = 5;

        let mut m = Array2d::gen(size, size, |i, j| (i, j));

        m.swap_columns(0, 2);
        let m21034 = Array2d::try_from(
            size,
            size,
            vec![
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 3),
                (0, 4),
                (1, 2),
                (1, 1),
                (1, 0),
                (1, 3),
                (1, 4),
                (2, 2),
                (2, 1),
                (2, 0),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 1),
                (3, 0),
                (3, 3),
                (3, 4),
                (4, 2),
                (4, 1),
                (4, 0),
                (4, 3),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m21034");
        assert_eq!(m, m21034, "Column swap #0 (0, 2) failed");

        m.swap_columns(2, 3);
        let m21304 = Array2d::try_from(
            size,
            size,
            vec![
                (0, 2),
                (0, 1),
                (0, 3),
                (0, 0),
                (0, 4),
                (1, 2),
                (1, 1),
                (1, 3),
                (1, 0),
                (1, 4),
                (2, 2),
                (2, 1),
                (2, 3),
                (2, 0),
                (2, 4),
                (3, 2),
                (3, 1),
                (3, 3),
                (3, 0),
                (3, 4),
                (4, 2),
                (4, 1),
                (4, 3),
                (4, 0),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m21304");
        assert_eq!(m, m21304, "Column swap #1 (2, 3) failed");

        m.swap_columns(0, 2);
        let m31204 = Array2d::try_from(
            size,
            size,
            vec![
                (0, 3),
                (0, 1),
                (0, 2),
                (0, 0),
                (0, 4),
                (1, 3),
                (1, 1),
                (1, 2),
                (1, 0),
                (1, 4),
                (2, 3),
                (2, 1),
                (2, 2),
                (2, 0),
                (2, 4),
                (3, 3),
                (3, 1),
                (3, 2),
                (3, 0),
                (3, 4),
                (4, 3),
                (4, 1),
                (4, 2),
                (4, 0),
                (4, 4),
            ],
        )
        .expect("Wrong dimensions for m31204");
        assert_eq!(m, m31204, "Column swap #2 (0, 2) failed");

        m.swap_columns(0, 3);
        let m01234 = Array2d::gen(size, size, |i, j| (i, j));
        assert_eq!(m, m01234, "Column swap #3 (0, 3) failed");
    }
}
