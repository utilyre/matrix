use std::{
    ops::{Add, AddAssign},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use cursor::{Cursor, CursorIterator};

mod cursor;

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn with_dimensions(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            rows,
            cols,
            elements: vec![T::default(); rows * cols],
        }
    }

    pub fn with_elements(breakat: usize, elements: Vec<T>) -> Self {
        let r = elements.len() % breakat;
        assert_eq!(r, 0, "missing {} elements", breakat - r);

        Self {
            rows: elements.len() / breakat,
            cols: breakat,
            elements,
        }
    }

    pub fn iter(&self) -> Cursor<Iter<T>> {
        self.elements.iter().cursor(self.cols)
    }

    pub fn iter_mut(&mut self) -> Cursor<IterMut<T>> {
        self.elements.iter_mut().cursor(self.cols)
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = ((usize, usize), T);
    type IntoIter = Cursor<IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter().cursor(self.cols)
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.cols, "matrices aren't the same size");
        assert_eq!(
            self.elements.len(),
            rhs.elements.len(),
            "matrices aren't the same size"
        );

        Matrix::with_elements(
            self.cols,
            self.elements
                .into_iter()
                .zip(rhs.elements.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.cols, rhs.cols, "matrices aren't the same size");
        assert_eq!(
            self.elements.len(),
            rhs.elements.len(),
            "matrices aren't the same size"
        );

        for ((_, a), (_, b)) in self.iter_mut().zip(rhs.into_iter()) {
            *a += b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "missing 3 elements")]
    fn instantiate() {
        Matrix::with_elements(5, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn iter() {
        let matrix = Matrix::with_elements(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter();
        assert_eq!(iterator.next(), Some(((0, 0), &1)));
        assert_eq!(iterator.next(), Some(((0, 1), &2)));
        assert_eq!(iterator.next(), Some(((1, 0), &3)));
        assert_eq!(iterator.next(), Some(((1, 1), &4)));
    }

    #[test]
    fn iter_mut() {
        let mut matrix = Matrix::with_elements(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter_mut();
        let (_, first) = iterator.next().unwrap();

        *first = 5;

        assert_eq!(matrix.elements, vec![5, 2, 3, 4]);
    }

    #[test]
    fn into_iter() {
        let matrix = Matrix::with_elements(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.into_iter();
        let (_, first) = iterator.next().unwrap();

        assert_eq!(first, 1);
    }

    #[test]
    fn add() {
        let m1 = Matrix::with_elements(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let m2 = Matrix::with_elements(5, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

        assert_eq!(
            (m1 + m2).elements,
            vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
    }

    #[test]
    fn add_assign() {
        let mut m1 = Matrix::with_elements(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let m2 = Matrix::with_elements(5, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

        m1 += m2;
        assert_eq!(m1.elements, vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);
    }
}
