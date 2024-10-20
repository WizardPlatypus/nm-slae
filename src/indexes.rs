#[derive(Clone)]
pub struct Row {
    height: usize,
    width: usize,
    row: usize,
    left: usize,
    right: usize,
}

impl Row {
    pub fn new(height: usize, width: usize, row: usize) -> Row {
        Row {
            height,
            width,
            row,
            left: 0,
            right: 0,
        }
    }
}

impl std::iter::Iterator for Row {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.width - self.right {
            let cell = (self.row, self.left);
            self.left += 1;
            Some(cell)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.width - self.left - self.right;
        (rest, Some(rest))
    }
}

impl std::iter::ExactSizeIterator for Row {}

impl std::iter::DoubleEndedIterator for Row {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left < self.width - self.right {
            let cell = (self.row, self.width - self.right - 1);
            self.right += 1;
            Some(cell)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Rows {
    height: usize,
    width: usize,
    left: usize,
    right: usize,
}

impl Rows {
    pub fn new(height: usize, width: usize) -> Rows {
        Rows {
            height,
            width,
            left: 0,
            right: 0,
        }
    }
}

impl std::iter::Iterator for Rows {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.height - self.right {
            let row = Row::new(self.height, self.width, self.left);
            self.left += 1;
            Some(row)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.height - self.left - self.right;
        (rest, Some(rest))
    }
}

impl std::iter::ExactSizeIterator for Rows {}

impl std::iter::DoubleEndedIterator for Rows {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left < self.height - self.right {
            let row = Row::new(self.height, self.width, self.height - self.right - 1);
            self.right += 1;
            Some(row)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Column {
    height: usize,
    width: usize,
    column: usize,
    left: usize,
    right: usize,
}

impl Column {
    pub fn new(height: usize, width: usize, column: usize) -> Column {
        Column {
            height,
            width,
            column,
            left: 0,
            right: 0,
        }
    }
}

impl std::iter::Iterator for Column {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.height - self.right {
            let cell = (self.left, self.column);
            self.left += 1;
            Some(cell)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.height - self.left - self.right;
        (rest, Some(rest))
    }
}

impl std::iter::ExactSizeIterator for Column {}

impl std::iter::DoubleEndedIterator for Column {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left < self.height - self.right {
            let cell = (self.height - self.right - 1, self.column);
            self.right += 1;
            Some(cell)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Columns {
    height: usize,
    width: usize,
    left: usize,
    right: usize,
}

impl Columns {
    pub fn new(height: usize, width: usize) -> Columns {
        Columns {
            height,
            width,
            left: 0,
            right: 0,
        }
    }
}

impl std::iter::Iterator for Columns {
    type Item = Column;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left < self.width - self.right {
            let column = Column::new(self.height, self.width, self.left);
            self.left += 1;
            Some(column)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.width - self.left - self.right;
        (rest, Some(rest))
    }
}

impl std::iter::ExactSizeIterator for Columns {}

impl std::iter::DoubleEndedIterator for Columns {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left < self.width - self.right {
            let column = Column::new(self.height, self.width, self.width - self.right - 1);
            self.right += 1;
            Some(column)
        } else {
            None
        }
    }
}

pub trait Indexable {
    fn iheight(&self) -> usize;
    fn iwidth(&self) -> usize;

    fn irow(&self, row: usize) -> Row {
        Row::new(self.iheight(), self.iwidth(), row)
    }
    fn icolumn(&self, column: usize) -> Column {
        Column::new(self.iheight(), self.iwidth(), column)
    }

    fn irows(&self) -> Rows {
        Rows::new(self.iheight(), self.iwidth())
    }
    fn icolumns(&self) -> Columns {
        Columns::new(self.iheight(), self.iwidth())
    }
}

impl<M: crate::Matrix<Item = T>, T> Indexable for M {
    fn iheight(&self) -> usize {
        self.height()
    }

    fn iwidth(&self) -> usize {
        self.width()
    }
}
