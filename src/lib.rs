pub mod gauss;
pub mod jacobi;

mod array2d;
mod iterators;
mod meow;
mod traits;
mod transposed;

pub mod indexes;

pub use array2d::Array2d;
pub use indexes::Indexable;
pub use iterators::*;
pub use meow::Meow;
pub use traits::Matrix;
pub use transposed::Transposed;

pub fn multiply_diagonal<M: Matrix<Item=f64>>(m: &M) -> f64 {
    let mut product = 1.0;
    let mut i = 0;
    while let Some(value) = m.at(i, i) {
        product *= value;
        i += 1;
    }
    product
}

pub fn inf_norm<M: Matrix<Item=f64>>(m: &M) -> f64 {
    m.rows()
        .map(|row| row.cloned().map(f64::abs).sum())
        .max_by(f64::total_cmp)
        .expect("There was less than one element")
}

/*
impl Matrix<f64> {
    pub fn e(n: usize) -> Matrix<f64> {
        let data = vec![0.0; n * n];
        let mut e = Matrix {
            height: n,
            width: n,
            data,
        };
        for i in 0..n {
            *e.at_mut(i, i) = 1.0;
        }
        e
    }

    pub fn inverse(&self) -> Result<Matrix<f64>, String> {
        let e = Matrix::e(self.height());
        let mut m = self.append_right(&e).unwrap();
        for iter in 0..m.height() {
            let (main, _) = m
                .column(iter)
                .enumerate()
                .skip(iter)
                .max_by(|&(_, a), &(_, b)| f64::total_cmp(a, b))
                .unwrap();

            if main != iter {
                m.swap_rows(main, iter);
            }

            let first = 1.0 / m.at(iter, iter);
            for col in iter..m.width() {
                *m.at_mut(iter, col) *= first;
            }

            for row in (iter + 1)..m.height() {
                let first = *m.at(row, iter);
                for col in iter..m.width() {
                    *m.at_mut(row, col) -= m.at(iter, col) * first;
                }
            }
        }
        for iter in 0..m.height() {
            let first = 1.0 / m.at(m.height() - iter - 1, m.height() - iter - 1);
            for col in iter..m.width() {
                *m.at_mut(m.height() - iter - 1, col) *= first;
            }

            for row in (iter + 1)..m.height() {
                let first = *m.at(m.height() - row - 1, m.height() - iter - 1);
                for col in iter..m.width() {
                    *m.at_mut(m.height() - row - 1, col) -=
                        m.at(m.height() - iter - 1, col) * first;
                }
            }
        }
        m.window(0, m.height(), m.height(), m.height())
    }

    pub fn cond(&self) -> f64 {
        let me = self.norm();
        let inv = self.inverse().unwrap().norm();
        me * inv
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen(i: usize, j: usize) -> (usize, usize) {
        (i, j)
    }

    #[test]
    fn rows_ok() {
        let size = 5;
        let m = Matrix::new(size, size, gen);
        for (row_id, row) in m.rows().enumerate() {
            for (column_id, &(i, j)) in row.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn columns_ok() {
        let size = 5;
        let m = Matrix::new(size, size, gen);
        for (column_id, column) in m.columns().enumerate() {
            for (row_id, &(i, j)) in column.enumerate() {
                assert_eq!(i, row_id);
                assert_eq!(j, column_id);
            }
        }
    }

    #[test]
    fn inverse_ok() {
        let m = Matrix::try_from_iter(
            [1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0]
                .iter()
                .map(std::clone::Clone::clone),
            3,
            3,
        )
        .unwrap();
        let inv = m.inverse().unwrap();
        let expected = Matrix::try_from_iter(
            [-0.375, 0.5, 0.125, 0.5, -1.0, 0.5, 0.125, 0.5, -0.375],
            3,
            3,
        )
        .unwrap();
        assert_eq!(inv.height(), expected.height());
        assert_eq!(inv.width(), expected.width());
        println!("{:?}", inv.data);
        println!("{:?}", expected.data);
        for i in 0..3 {
            for j in 0..3 {
                assert!((inv.at(i, j) - expected.at(i, j)).abs() < 10e-6);
            }
        }
    }

    #[test]
    fn transposed_ok() {
        let a = Matrix::try_from_iter([1, 2, 3, 4, 5, 6, 7, 8].iter(), 2, 4).unwrap();
        let b = Matrix::try_from_iter([1, 5, 2, 6, 3, 7, 4, 8].iter(), 4, 2).unwrap();
        assert_eq!(a.data, b.transposed().data);
        assert_eq!(b.data, a.transposed().data);
    }
}
*/
