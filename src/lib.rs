use std::fmt::{self, Display, Formatter};

pub struct Matrix<T> {
    row: usize,
    col: usize,
    elements: Vec<T>,
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.row {
            write!(f, "|")?;
            for j in 0..self.col {
                write!(f, "{:<4}", self.elements[i * self.col + j])?;
            }
            write!(f, "|")?;

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Matrix<T> {
    pub fn new(row: usize, col: usize, elements: Vec<T>) -> Self {
        assert_eq!(
            elements.len(),
            row * col,
            "expected {} elements but only got {}",
            row * col,
            elements.len()
        );

        Self { row, col, elements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
