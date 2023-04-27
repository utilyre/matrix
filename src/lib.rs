use std::{
    ops::Add,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use cursor::{Cursor, CursorIterator};

mod cursor;

pub struct Matrix<T> {
    width: usize,
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, elements: Vec<T>) -> Self {
        let r = elements.len() % width;
        assert_eq!(r, 0, "missing {} elements", width - r);

        Self { width, elements }
    }

    pub fn iter(&self) -> Cursor<Iter<T>> {
        self.elements.iter().cursor(self.width)
    }

    pub fn iter_mut(&mut self) -> Cursor<IterMut<T>> {
        self.elements.iter_mut().cursor(self.width)
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = ((usize, usize), T);
    type IntoIter = Cursor<IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter().cursor(self.width)
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width, "matrices aren't the same size");
        assert_eq!(
            self.elements.len(),
            rhs.elements.len(),
            "matrices aren't the same size"
        );

        Matrix::new(
            self.width,
            self.elements
                .into_iter()
                .zip(rhs.elements.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "missing 3 elements")]
    fn instantiate() {
        Matrix::new(5, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn iter() {
        let matrix = Matrix::new(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter();
        assert_eq!(iterator.next(), Some(((0, 0), &1)));
        assert_eq!(iterator.next(), Some(((0, 1), &2)));
        assert_eq!(iterator.next(), Some(((1, 0), &3)));
        assert_eq!(iterator.next(), Some(((1, 1), &4)));
    }

    #[test]
    fn iter_mut() {
        let mut matrix = Matrix::new(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter_mut();
        let (_, first) = iterator.next().unwrap();

        *first = 5;

        assert_eq!(matrix.elements, vec![5, 2, 3, 4]);
    }

    #[test]
    fn into_iter() {
        let matrix = Matrix::new(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.into_iter();
        let (_, first) = iterator.next().unwrap();

        assert_eq!(first, 1);
    }

    #[test]
    fn add() {
        let m1 = Matrix::new(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let m2 = Matrix::new(5, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

        assert_eq!(
            (m1 + m2).elements,
            vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
    }
}
