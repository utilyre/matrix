use std::slice::Iter;

use cursor::{Cursor, CursorIter};

mod cursor;

pub struct Matrix<T> {
    width: usize,
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, elements: Vec<T>) -> Self {
        Self { width, elements }
    }

    pub fn iter(&self) -> Cursor<Iter<T>> {
        self.elements.iter().cursor(self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iteration() {
        let matrix = Matrix::new(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter();
        assert_eq!(iterator.next(), Some(((0, 0), &1)));
        assert_eq!(iterator.next(), Some(((0, 1), &2)));
        assert_eq!(iterator.next(), Some(((1, 0), &3)));
        assert_eq!(iterator.next(), Some(((1, 1), &4)));
    }
}
