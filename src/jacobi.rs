use super::*;

pub struct Jacobi {
    a: Matrix<f64>,
    x: Vec<f64>,
    epsilon: f64
}

impl Jacobi {
    pub fn converges(&self) -> bool {
        for (i, row) in self.a.rows().enumerate() {
            let d = self.a.at(i, i).abs();
            let sum: f64 = row.map(Clone::clone).map(f64::abs).sum();
            if d < sum - d {
                return false;
            }
        }
        true
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
    
    pub fn stop(&self, old: Vec<f64>, new: Vec<f64>) -> bool {
	std::iter::zip(old, new).map(|(a, b)| (a - b).abs()).max_by(f64::total_cmp);
	true
    }
}
