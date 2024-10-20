use matrices::{gauss, Array2d, Meow};

fn gen_a(i: usize, j: usize) -> i64 {
    if i == 0 && j == 0 {
        1
    } else if i == j {
        0
    } else if i > j {
        -((j + 1) as i64)
    } else {
        // if j > i
        (j + 1) as i64
    }
}

fn gen_b(i: usize) -> f64 {
    (i + 1) as f64
}

fn main() {
    use std::str::FromStr;
    let n: usize = usize::from_str(&std::env::args().nth(1).expect("Missing argument"))
        .expect("N should be a non negative integer");

    let a = Array2d::gen(n, n, |i, j| gen_a(i, j) as f64);
    let b = Array2d::gen(n, 1, |i, _| gen_b(i));
    let e = Array2d::gen(n, n, |i, j| if i == j { 1.0 } else { 0.0 });

    let norm = matrices::inf_norm(&a);

    println!("A =\n{}", a);
    println!("b =\n{}", b);

    let mut m = Meow::from(a);
    m.eat(b).expect("Failed to consume B");
    m.eat(e).expect("Failed to consume E");

    println!("M =\n{}", m);

    gauss::calc_l(&mut m);
    println!("L =\n{}", m);

    gauss::calc_u(&mut m);
    println!("U =\n{}", m);

    let det = matrices::multiply_diagonal(&m);
    println!("det = {}", det);

    gauss::normalize(&mut m);
    println!("normalized =\n{}", m);

    let inverse = m.calculate(2).unwrap();
    let x = m.calculate(1).unwrap();

    println!("inverse =\n{}", inverse);
    println!("x =\n{}", x);

    let inverse_norm = matrices::inf_norm(&inverse);
    let cond = norm * inverse_norm;
    println!("||A|| = {:.2}", norm);
    println!("||inverse|| = {}", inverse_norm);
    println!("cond(A) = {}", cond);
}
