use crate::Matrix;

pub struct Row<'a, T> {
    origin: &'a T,
    row: usize,
    left: usize,
    right: usize,
}

impl<'a, T> Row<'a, T> {
    pub fn new(origin: &'a T, row: usize) -> Row<'a, T> {
        Row {
            origin,
            row,
            left: 0,
            right: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::Iterator for Row<'a, M> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.origin.width() - self.right {
            let cell = self.origin.at(self.row, self.left);
            self.left += 1;
            cell
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.origin.width() - self.left - self.right;
        (rest, Some(rest))
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::ExactSizeIterator for Row<'a, M> {}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::DoubleEndedIterator for Row<'a, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let w = self.origin.width();
        if self.left < w - self.right {
            let cell = self.origin.at(self.row, w - self.right - 1);
            self.right += 1;
            cell
        } else {
            None
        }
    }
}

pub struct Rows<'a, T> {
    origin: &'a T,
    left: usize,
    right: usize,
}

impl<'a, T> Rows<'a, T> {
    pub fn new(
        origin: &'a T,
    ) -> Rows<'a, T> {
        Rows {
            origin,
            left: 0,
            right: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T> std::iter::Iterator for Rows<'a, M> {
    type Item = Row<'a, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.origin.height() - self.right {
            let row = Row::new(self.origin, self.left);
            self.left += 1;
            Some(row)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.origin.height() - self.left - self.right;
        (rest, Some(rest))
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::ExactSizeIterator for Rows<'a, M> {}

impl<'a, M: Matrix<Item=T>, T> std::iter::DoubleEndedIterator for Rows<'a, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let h = self.origin.height();
        if self.left < h - self.right {
            let row = Row::new(self.origin, h - self.right - 1);
            self.right += 1;
            Some(row)
        } else {
            None
        }
    }
}

pub struct Column<'a, T> {
    origin: &'a T,
    column: usize,
    left: usize,
    right: usize,
}

impl<'a, T> Column<'a, T> {
    pub fn new(origin: &'a T, column: usize) -> Column<'a, T> {
        Column {
            origin,
            column,
            left: 0,
            right: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::Iterator for Column<'a, M> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.origin.height() - self.right {
            let cell = self.origin.at(self.left, self.column);
            self.left += 1;
            cell
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.origin.height() - self.left - self.right;
        (rest, Some(rest))
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::ExactSizeIterator for Column<'a, M> {}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::DoubleEndedIterator for Column<'a, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let h = self.origin.height();
        if self.left < h - self.right {
            let cell = self.origin.at(h - self.right - 1, self.column);
            self.right += 1;
            cell
        } else {
            None
        }
    }
}

pub struct Columns<'a, T> {
    origin: &'a T,
    left: usize,
    right: usize,
}

impl<'a, T> Columns<'a, T> {
    pub fn new(
        origin: &'a T,
    ) -> Columns<'a, T> {
        Columns {
            origin,
            left: 0,
            right: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T> std::iter::Iterator for Columns<'a, M> {
    type Item = Column<'a, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.origin.width() - self.right {
            let column = Column::new(self.origin, self.left);
            self.left += 1;
            Some(column)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.origin.width() - self.left - self.right;
        (rest, Some(rest))
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::ExactSizeIterator for Columns<'a, M> {}

impl<'a, M: Matrix<Item=T>, T> std::iter::DoubleEndedIterator for Columns<'a, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let w = self.origin.width();
        if self.left < w - self.right {
            let column = Column::new(self.origin, w - self.right - 1);
            self.right += 1;
            Some(column)
        } else {
            None
        }
    }
}

pub trait Iteratable<M> {
    fn origin(&self) -> &M;

    fn row(&self, row: usize) -> Row<'_, M> {
        Row::new(self.origin(), row)
    }
    fn column(&self, column: usize) -> Column<'_, M> {
        Column::new(self.origin(), column)
    }

    fn rows(&self) -> Rows<'_, M> {
        Rows::new(self.origin())
    }
    fn columns(&self) -> Columns<'_, M> {
        Columns::new(self.origin())
    }
}

impl<M: Matrix<Item=T>, T> Iteratable<M> for M {
    fn origin(&self) -> &M {
        self
    }
}