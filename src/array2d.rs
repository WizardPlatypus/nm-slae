use crate::traits::Mapped;
use crate::Matrix;
use either::{Either, Left, Right};

#[derive(Debug, Clone)]
pub struct Array2d<T> {
    rows: Either<usize, Vec<usize>>,
    columns: Either<usize, Vec<usize>>,
    data: Vec<T>,
}

impl<T: Default + Clone> Array2d<T> {
    pub fn default(height: usize, width: usize) -> Array2d<T> {
        Array2d {
            rows: Left(height),
            columns: Left(width),
            data: vec![T::default(); height * width],
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
            data,
        }
    }

    pub fn try_from(height: usize, width: usize, data: Vec<T>) -> Result<Array2d<T>, Vec<T>> {
        if height * width == data.len() {
            Ok(Array2d {
                rows: Left(height),
                columns: Left(width),
                data,
            })
        } else {
            Err(data)
        }
    }

    pub fn sync_rows(&mut self, temp: &mut T) {
        for row in 0..self.height() {
            self.sync_column(row, temp);
        }
    }

    pub fn sync_columns(&mut self, temp: &mut T) {
        for column in 0..self.width() {
            self.sync_row(column, temp);
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

impl<T> Mapped for Array2d<T> {
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
        // let width = self.width();
        // self.data.get_mut(row * width + column).expect("Invalid access request from Mapped trait")
    }

    fn reset_rows(&mut self, height: usize) {
        self.rows = Left(height);
    }

    fn reset_columns(&mut self, width: usize) {
        self.columns = Left(width);
    }
}

impl<T: PartialEq> PartialEq for Array2d<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.height() != other.height() {
            return false;
        }
        if self.width() != other.width() {
            return false;
        }

        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.at(i, j) != other.at(i, j) {
                    return false;
                }
            }
        }

        true
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

#[cfg(test)]
mod test {
    use super::Array2d;
    use crate::{traits::Mapped, Iteratable, Matrix};

    #[test]
    fn gen_matches_try_from() {
        let data: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        let a = Array2d::try_from(3, 3, data).expect("Dimensions were incorrect");
        let b = Array2d::gen(3, 3, |i, j| (i, j));
        assert_eq!(a, b);
    }

    #[test]
    fn rows_access_ok() {
        let m = Array2d::gen(3, 3, |i, j| (i, j));
        for (row_id, row) in m.rows().enumerate() {
            for (column_id, &(i, j)) in row.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn columns_access_ok() {
        let m = Array2d::gen(3, 3, |i, j| (i, j));
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
        let mut temp = (0, 0);

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

        // m.sync_rows(&mut temp);
        // m.reset_rows(size);
        // assert_eq!(m, m21034, "Row sync #0 (0, 2) failed");

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

        // m.sync_rows(&mut temp);
        // m.reset_rows(size);
        // assert_eq!(m, m21034, "Row sync #2 (0, 2) failed");

        m.swap_rows(0, 3);
        let m01234 = Array2d::gen(size, size, |i, j| (i, j));
        assert_eq!(m, m01234, "Row swap #3 (0, 3) failed");

        // m.sync_rows(&mut temp);
        // m.reset_rows(size);
        // assert_eq!(m, m21034, "Row sync #3 (0, 3) failed");
    }

    #[test]
    fn swap_columns_ok() {
        let size = 5;

        let mut m = Array2d::gen(size, size, |i, j| (i, j));
        let mut temp = (0, 0);

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

        // m.sync_columns(&mut temp);
        // m.reset_columns(size);
        // assert_eq!(m, m21034, "Column sync #0 (0, 2) failed");

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

        // m.sync_columns(&mut temp);
        // m.reset_columns(size);
        // assert_eq!(m, m21034, "Column sync #2 (0, 2) failed");

        m.swap_columns(0, 3);
        let m01234 = Array2d::gen(size, size, |i, j| (i, j));
        assert_eq!(m, m01234, "Column swap #3 (0, 3) failed");

        // m.sync_columns(&mut temp);
        // m.reset_columns(size);
        // assert_eq!(m, m21034, "Column sync #3 (0, 3) failed");
    }
}
