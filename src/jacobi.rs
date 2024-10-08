use super::*;

pub struct Jacobi {
    a: Matrix<f64>,
    x: Vec<f64>,
    trace: Vec<Trace>,
}

pub struct Trace {
    pub x: Vec<f64>,
    pub precision: f64,
}

impl Jacobi {
    pub fn new(a: Matrix<f64>) -> Result<Jacobi, Matrix<f64>> {
        /*
            if converges(&a) {
                let x = vec![0.0; a.height()];
                Ok(Jacobi {
                    a,
                    x,
                    trace: vec![],
                })
            } else {
                Err(a)
            }
        // */
        let x = vec![0.0; a.height()];
        Ok(Jacobi {
            a,
            x,
            trace: vec![],
        })
    }

    pub fn next(&self) -> Vec<f64> {
        let mut x = vec![0.0; self.x.len()];
        for i in 0..self.x.len() {
            for j in 0..self.a.height() {
                if j == i {
                    continue;
                }
                x[i] -= self.a.at(i, j) / self.a.at(i, i) * self.x[j]
            }
            x[i] += self.a.at(i, self.a.width() - 1) / self.a.at(i, i);
        }
        x
    }

    pub fn solve(&mut self, epsilon: f64) -> Vec<f64> {
        let mut next = self.next();
        let mut p = precision(&self.x, &next);
        self.trace.push(Trace {
            x: next.clone(),
            precision: p,
        });
        while p > epsilon {
            std::mem::swap(&mut self.x, &mut next);
            next = self.next();
            p = precision(&self.x, &next);
            self.trace.push(Trace {
                x: next.clone(),
                precision: p,
            });
        }
        next
    }

    pub fn trace(&self) -> &Vec<Trace> {
        &self.trace
    }
}

pub fn converges(m: &Matrix<f64>) -> bool {
    for (i, row) in m.rows().enumerate() {
        let d = m.at(i, i).abs();
        let sum: f64 = row.take(m.height()).map(Clone::clone).map(f64::abs).sum();
        if d < sum - d {
            return false;
        }
    }
    true
}

pub fn precision(old: &Vec<f64>, new: &Vec<f64>) -> f64 {
    std::iter::zip(old, new)
        .map(|(a, b)| (a - b).abs())
        .max_by(f64::total_cmp)
        .unwrap()
}
