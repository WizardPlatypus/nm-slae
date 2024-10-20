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

pub fn multiply_diagonal<M: Matrix<Item = f64>>(m: &M) -> f64 {
    let mut product = 1.0;
    let mut i = 0;
    while let Some(value) = m.at(i, i) {
        product *= value;
        i += 1;
    }
    product
}

pub fn inf_norm<M: Matrix<Item = f64>>(m: &M) -> f64 {
    m.rows()
        .map(|row| row.cloned().map(f64::abs).sum())
        .max_by(f64::total_cmp)
        .expect("There was less than one element")
}
