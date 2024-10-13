use crate::Matrix;

pub struct Row<'a, T> {
    origin: &'a T,
    row: usize,
    cursor: usize,
}

impl<'a, T> Row<'a, T> {
    pub fn new(origin: &'a T, row: usize) -> Row<'a, T> {
        Row {
            origin,
            row,
            cursor: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::Iterator for Row<'a, M> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.origin.width() {
            let cell = self.origin.at(self.row, self.cursor);
            self.cursor += 1;
            cell
        } else {
            None
        }
    }
}

pub struct Rows<'a, T> {
    origin: &'a T,
    cursor: usize,
}

impl<'a, T> Rows<'a, T> {
    pub fn new(
        origin: &'a T,
    ) -> Rows<'a, T> {
        Rows {
            origin,
            cursor: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T> std::iter::Iterator for Rows<'a, M> {
    type Item = Row<'a, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.origin.height() {
            let row = Row::new(self.origin, self.cursor);
            self.cursor += 1;
            Some(row)
        } else {
            None
        }
    }
}

pub struct Column<'a, T> {
    origin: &'a T,
    column: usize,
    cursor: usize,
}

impl<'a, T> Column<'a, T> {
    pub fn new(origin: &'a T, column: usize) -> Column<'a, T> {
        Column {
            origin,
            column,
            cursor: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T: 'a> std::iter::Iterator for Column<'a, M> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.origin.height() {
            let cell = self.origin.at(self.cursor, self.column);
            self.cursor += 1;
            cell
        } else {
            None
        }
    }
}

pub struct Columns<'a, T> {
    origin: &'a T,
    cursor: usize,
}

impl<'a, T> Columns<'a, T> {
    pub fn new(
        origin: &'a T,
    ) -> Columns<'a, T> {
        Columns {
            origin,
            cursor: 0,
        }
    }
}

impl<'a, M: Matrix<Item=T>, T> std::iter::Iterator for Columns<'a, M> {
    type Item = Column<'a, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.origin.width() {
            let column = Column::new(self.origin, self.cursor);
            self.cursor += 1;
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