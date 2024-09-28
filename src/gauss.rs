use super::Matrix;

pub struct Gauss {
    a: Matrix<f64>,
    step: usize,
    trace: Vec<Label>,
}

pub enum Label {
    Modified { index: usize, matrix: Matrix<f64> },
}

impl Gauss {
    pub fn at(&self, row: usize, col: usize) -> &f64 {
	self.a.at(row, col)
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
	self.a.at_mut(row, col)
    }
}

impl Gauss {
    pub fn modify(&mut self) {
        let step = self.step;

        let first = 1.0 / self.at(step, step);
        for col in step..self.a.cols {
            *self.at_mut(step, col) *= first;
        }

        for row in (step + 1)..self.a.rows {
            let first = *self.at(row, step);
            for col in step..self.a.cols {
                *self.at_mut(row, col) -= self.at(step, col) * first;
            }
        }
    }

    pub fn reverse(&self) -> Vec<f64> {
        let mut x = vec![0.0; self.a.rows];
        for k in 1..=self.a.rows {
            let i = self.a.rows - k;
            x[i] = *self.at(i, self.a.cols - 1);
            for j in (i + 1)..self.a.rows {
                x[i] -= self.at(i, j);
            }
        }
        x
    }

    pub fn solve(&mut self) -> f64 {
        let mut det = 1.0;
        for step in 0..self.a.rows {
            log::debug!("Step {step}");
            let main = self.a.max_in_col(step, Some(step));
            log::debug!("Main ({main}, {step}) = {}", self.at(main, step));
            det *= self.at(main, step);
            if main != step {
                log::debug!("Swapping rows {main} and {step}");
                self.a.swap_rows(main, step);
                log::debug!("After swap:");
                log::debug!("\n{:?}", self.a);
                det *= -1.0;
            }
            log::debug!("Applying M{step}");
            self.modify();
            log::debug!("After M{step}:");
            log::debug!("\n{:?}", self.a);
            self.step += 1;
        }
        det
    }
}

impl TryFrom<Matrix<f64>> for Gauss {
    type Error = Matrix<f64>;
    fn try_from(value: Matrix<f64>) -> Result<Self, Self::Error> {
	if value.height() + 1 != value.width() {
	    Err(value)
	} else {
	    Ok(Gauss { a: value, step: 0, trace: vec![]})
	}
    }
}
