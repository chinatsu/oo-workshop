use std::ops::{BitOr, BitAnd, Not};

static CERTAIN: f64 = 1.0;
static DELTA: f64 = 0.000000000000001;

/// Understands the chance for something to happen
#[derive(Debug, Copy, Clone)]
pub struct Chance {
    fraction: f64,
}

#[derive(Debug, PartialEq)]
pub enum ChanceError {
    OutOfBounds
}

impl Chance {
    pub fn new(fraction: f64) -> Result<Chance, ChanceError> {
        if fraction > CERTAIN || fraction < 0.0 {
            return Err(ChanceError::OutOfBounds);
        }
        Ok(Chance {
            fraction: fraction,
        })
    }
}

impl PartialEq for Chance {
    fn eq(&self, rhs: &Self) -> bool {
        (self.fraction - rhs.fraction).abs() < DELTA
    }
}

impl Not for Chance {
    type Output = Chance;
    fn not(self) -> Self::Output {
        Chance {
            fraction: CERTAIN - self.fraction,
        }
    }
}

impl BitAnd for Chance {
    type Output = Chance;
    fn bitand(self, rhs: Self) -> Self::Output {
        Chance {
            fraction: self.fraction * rhs.fraction
        }
    }
}

impl BitOr for Chance {
    type Output = Chance;
    fn bitor(self, rhs: Self) -> Self::Output {
        Chance {
            fraction: (!(!self & !rhs)).fraction
        }
    }
}

#[cfg(test)]
mod chance_test;
