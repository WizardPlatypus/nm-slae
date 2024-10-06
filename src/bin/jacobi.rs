use matrices::{
    jacobi::{self, Jacobi},
    Matrix,
};

pub fn main() {
    let m = Matrix::try_from_iter(
        [
            3.0, -1.0, 1.0, 1.0, -1.0, 2.0, 0.5, 1.75, 1.0, 0.5, 3.0, 2.5,
        ],
        3,
        4,
    )
    .unwrap();
    println!("converges: {:?}", jacobi::converges(&m));
    let mut g = Jacobi::new(m).unwrap();
    let e = 0.5;
    let solution = g.solve(e);
    for (i, jacobi::Trace { x, precision }) in g.trace().iter().enumerate() {
        println!("{} -> {precision:.2}: {x:.2?}", i + 1);
    }
    println!("{:.2?}", solution);
}
