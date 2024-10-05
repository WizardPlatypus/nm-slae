pub struct Row<'a, T> {
    data: &'a [T],
    width: usize,
    row: usize,
    ptr: usize,
}

impl<'a, T> Row<'a, T> {
    pub(super) fn new<S: AsRef<[T]> + ?Sized>(data: &'a S, width: usize, row: usize) -> Row<'a, T> {
        Row {
            data: data.as_ref(),
            width,
            row,
            ptr: 0,
        }
    }

    fn cell(&self) -> &'a T {
        let index = self.row * self.width + self.ptr;
        &self.data[index]
    }
}

impl<'a, T> std::iter::Iterator for Row<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.width {
            let cell = self.cell();
            self.ptr += 1;
            Some(cell)
        } else {
            None
        }
    }
}

pub struct Rows<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
    ptr: usize,
}

impl<'a, T> Rows<'a, T> {
    pub(super) fn new<S: AsRef<[T]> + ?Sized>(
        data: &'a S,
        width: usize,
        height: usize,
    ) -> Rows<'a, T> {
        Rows {
            data: data.as_ref(),
            width,
            height,
            ptr: 0,
        }
    }

    fn row(&self) -> Row<'a, T> {
        Row::new(self.data, self.width, self.ptr)
    }
}

impl<'a, T> std::iter::Iterator for Rows<'a, T> {
    type Item = Row<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.height {
            let row = self.row();
            self.ptr += 1;
            Some(row)
        } else {
            None
        }
    }
}
