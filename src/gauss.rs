use crate::{Iteratable, Matrix};

fn total_cmp(a: &(usize, f64), b: &(usize, f64)) -> std::cmp::Ordering {
    f64::total_cmp(&a.1, &b.1)
}

pub fn calc_l<M: Matrix<Item = f64>>(m: &mut M) -> Option<()> {
    let h: usize = m.height();
    let w: usize = m.width();

    for i in 0..h {
        let diag = *m.at(i, i)?;
        let value = if diag == 0.0 {
            let (lead, value) = m
                .column(i)
                .map(Clone::clone)
                .enumerate()
                .skip(i)
                .max_by(total_cmp)?;

            if lead != i {
                m.swap_rows(lead, i)?;
            }

            value
        } else {
            diag
        };

        /* less readable
        for mut row in m.irows().skip(i + 1) {
            let (r, c) = row.next()?;
            let factor = m.at(r, c)? / value;
            *m.at_mut(r, c)? = 0.0;
            for (r, c) in row {
                *m.at_mut(r, c)? -= m.at(i, c)? * factor;
            }
        }
        // */

        for row in (i + 1)..h {
            let factor = m.at(row, i)? / value;
            *m.at_mut(row, i)? = 0.0;
            for column in (i + 1)..w {
                let src = m.at(i, column)? * factor;
                let dst = m.at_mut(row, column)?;
                *dst -= src;
                // *m.at_mut(row, column)? -= m.at(i, column)? * factor;
            }
        }
    }
    Some(())
}

pub fn calc_u<M: Matrix<Item = f64>>(m: &mut M) -> Option<()> {
    let h: usize = m.height();
    let w: usize = m.width();

    for i in 0..h {
        let t = h - i - 1;
        let diag = *m.at(t, t)?;
        let value = if diag == 0.0 {
            let (lead, value) = m
                .column(t)
                .map(Clone::clone)
                .enumerate()
                .rev()
                .skip(i)
                .max_by(total_cmp)?;

            if lead != t {
                m.swap_rows(lead, t)?;
            }

            value
        } else {
            diag
        };

        /* less readable
        for (j, row) in m.irows().enumerate().rev().skip(i + 1) {
            let factor = m.at(j, t)? / value;
            *m.at_mut(j, t)? = 0.0;

            let row = row.skip(j + 1);
            for (r, c) in row.clone().take(t - j - 1) {
                *m.at_mut(r, c)? -= m.at(i, c)? * factor;
            }

            let row = row.skip(1);
            for (r, c) in row {
                *m.at_mut(r, c)? -= m.at(i, c)? * factor;
            }
        }
        // */

        for row in (i + 1)..h {
            let row = h - row - 1;
            let factor = m.at(row, t)? / value;
            *m.at_mut(row, t)? = 0.0;
            for column in row..t {
                *m.at_mut(row, column)? -= m.at(t, column)? * factor;
            }
            for column in (t + 1)..w {
                *m.at_mut(row, column)? -= m.at(t, column)? * factor;
            }
        }
    }
    Some(())
}

pub fn normalize<M: Matrix<Item = f64>>(m: &mut M) {
    for i in 0..m.height() {
        let diag = *m.at(i, i).unwrap();
        for j in 0..m.width() {
            *m.at_mut(i, j).unwrap() /= diag;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Array2d, Iteratable, Meow};

    #[test]
    fn l_good_diagonal() {
        let mut a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 4.0, 3.0, 2.0, 1.0,
            ],
        )
        .unwrap();
        crate::gauss::calc_l(&mut a);
        println!("{}", a);

        for (i, row) in a.rows().enumerate().skip(1) {
            for cell in row.take(i) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

    #[test]
    fn l_bad_diagonal() {
        let mut a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 0.0, 2.0, 3.0, 3.0, 2.0, 0.0, 2.0, 4.0, 3.0, 2.0, 0.0,
            ],
        )
        .unwrap();
        crate::gauss::calc_l(&mut a);
        println!("{}", a);

        for (i, row) in a.rows().enumerate().skip(1) {
            for cell in row.take(i) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

    #[test]
    fn u_good_diagonal() {
        let mut a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 4.0, 3.0, 2.0, 1.0,
            ],
        )
        .unwrap();
        crate::gauss::calc_u(&mut a);
        println!("{}", a);

        for (i, row) in a.rows().enumerate().rev().skip(1) {
            for cell in row.skip(i + 1) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

    #[test]
    fn u_bad_diagonal() {
        let mut a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 0.0, 2.0, 3.0, 3.0, 2.0, 0.0, 2.0, 4.0, 3.0, 2.0, 0.0,
            ],
        )
        .unwrap();
        crate::gauss::calc_u(&mut a);
        println!("{}", a);

        for (i, row) in a.rows().enumerate().rev().skip(1) {
            for cell in row.skip(i + 1) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

    #[test]
    fn l_with_meow() {
        let a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 4.0, 3.0, 2.0, 1.0,
            ],
        )
        .unwrap();
        let b = Array2d::try_from(4, 1, vec![1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut m = Meow::from(a);
        m.eat(b).unwrap();

        crate::gauss::calc_l(&mut m);
        println!("{}", m);

        for (i, row) in m.rows().enumerate().skip(1) {
            for cell in row.take(i) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

    #[test]
    fn u_with_meow() {
        let a = Array2d::try_from(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 4.0, 3.0, 2.0, 1.0,
            ],
        )
        .unwrap();
        let b = Array2d::try_from(4, 1, vec![1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut m = Meow::from(a);
        m.eat(b).unwrap();

        crate::gauss::calc_u(&mut m);
        println!("{}", m);

        for (i, row) in m.rows().enumerate().skip(1) {
            for cell in row.take(4).skip(i + 1) {
                assert_eq!(*cell, 0.0);
            }
        }
    }

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

    #[test]
    fn l_my_example() {
        let mut a = Array2d::gen(5, 5, |i, j| gen_a(i, j) as f64);
        crate::gauss::calc_l(&mut a);

        let expected = Array2d::try_from(
            5,
            5,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, //
                0.0, 2.0, 6.0, 8.0, 10.0, //
                0.0, 0.0, 3.0, 8.0, 10.0, //
                0.0, 0.0, 0.0, 4.0, 10.0, //
                0.0, 0.0, 0.0, 0.0, 5.0, //
            ],
        )
        .unwrap();

        assert_eq!(a, expected);
    }
}
