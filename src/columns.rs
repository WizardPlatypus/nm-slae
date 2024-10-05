pub struct Column<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
    column: usize,
    ptr: usize,
}

impl<'a, T> Column<'a, T> {
    pub(super) fn new<S: AsRef<[T]> + ?Sized>(
        data: &'a S,
        width: usize,
        height: usize,
        column: usize,
    ) -> Column<'a, T> {
        Column {
            data: data.as_ref(),
            width,
            height,
            column,
            ptr: 0,
        }
    }

    fn cell(&self) -> &'a T {
        let index = self.ptr * self.width + self.column;
        &self.data[index]
    }
}

impl<'a, T> std::iter::Iterator for Column<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.height {
            let cell = self.cell();
            self.ptr += 1;
            Some(cell)
        } else {
            None
        }
    }
}

pub struct Columns<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
    ptr: usize,
}

impl<'a, T> Columns<'a, T> {
    pub(super) fn new<S: AsRef<[T]> + ?Sized>(
        data: &'a S,
        width: usize,
        height: usize,
    ) -> Columns<'a, T> {
        Columns {
            data: data.as_ref(),
            width,
            height,
            ptr: 0,
        }
    }

    fn column(&self) -> Column<'a, T> {
        Column::new(self.data, self.width, self.height, self.ptr)
    }
}

impl<'a, T> std::iter::Iterator for Columns<'a, T> {
    type Item = Column<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.width {
            let col = self.column();
            self.ptr += 1;
            Some(col)
        } else {
            None
        }
    }
}
