use std::{
    ops::{Add, AddAssign},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use cursor::{Cursor, CursorIterator};

mod cursor;

pub struct Diagonal<I>(I)
where
    I: Iterator;

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    entries: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn with_dimensions(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            rows,
            cols,
            entries: vec![T::default(); rows * cols],
        }
    }

    pub fn with_entries(breakat: usize, entries: Vec<T>) -> Self {
        assert!(breakat > 0, "`breakat` cannot be zero");
        let r = entries.len() % breakat;
        assert_eq!(r, 0, "missing {} entries", breakat - r);

        Self {
            rows: entries.len() / breakat,
            cols: breakat,
            entries,
        }
    }

    pub fn diagonal(&self) -> Vec<&T> {
        self.entries
            .iter()
            .cursor(self.cols)
            .filter_map(|((i, j), entry)| (i == j).then_some(entry))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.rows == 0 || self.cols == 0
    }

    pub fn is_row_vector(&self) -> bool {
        self.rows == 1
    }

    pub fn is_column(&self) -> bool {
        self.cols == 1
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn iter(&self) -> Cursor<Iter<T>> {
        self.entries.iter().cursor(self.cols)
    }

    pub fn iter_mut(&mut self) -> Cursor<IterMut<T>> {
        self.entries.iter_mut().cursor(self.cols)
    }
}

// TODO: implement for all integer and decimal types
impl Matrix<i32> {
    pub fn is_identity(&self) -> bool {
        self.entries
            .iter()
            .cursor(self.cols)
            .all(|((i, j), entry)| {
                if i == j {
                    *entry == 1
                } else {
                    *entry == 0
                }
            })
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = ((usize, usize), T);
    type IntoIter = Cursor<IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter().cursor(self.cols)
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
            self.entries.len(),
            rhs.entries.len(),
            "matrices aren't the same size"
        );

        Matrix::with_entries(
            self.cols,
            self.entries
                .into_iter()
                .zip(rhs.entries.into_iter())
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
            self.entries.len(),
            rhs.entries.len(),
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
    #[should_panic(expected = "missing 3 entries")]
    fn instantiate() {
        Matrix::with_entries(5, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn diagonal() {
        let matrix = Matrix::with_entries(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(matrix.diagonal(), vec![&1, &5, &9]);
    }

    #[test]
    fn matrix_types() {
        let empty: Matrix<()> = Matrix::with_entries(1, vec![]);
        assert!(empty.is_empty());

        let row_vector = Matrix::with_entries(4, vec![1, 2, 3, 4]);
        assert!(row_vector.is_row_vector());

        let col_vector = Matrix::with_entries(1, vec![1, 2, 3, 4]);
        assert!(col_vector.is_column());

        let square = Matrix::with_entries(2, vec![1, 2, 3, 4]);
        assert!(square.is_square());
    }

    #[test]
    fn iter() {
        let matrix = Matrix::with_entries(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter();
        assert_eq!(iterator.next(), Some(((0, 0), &1)));
        assert_eq!(iterator.next(), Some(((0, 1), &2)));
        assert_eq!(iterator.next(), Some(((1, 0), &3)));
        assert_eq!(iterator.next(), Some(((1, 1), &4)));
    }

    #[test]
    fn iter_mut() {
        let mut matrix = Matrix::with_entries(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.iter_mut();
        let (_, first) = iterator.next().unwrap();

        *first = 5;

        assert_eq!(matrix.entries, vec![5, 2, 3, 4]);
    }

    #[test]
    fn into_iter() {
        let matrix = Matrix::with_entries(2, vec![1, 2, 3, 4]);

        let mut iterator = matrix.into_iter();
        let (_, first) = iterator.next().unwrap();

        assert_eq!(first, 1);
    }

    #[test]
    fn add() {
        let m1 = Matrix::with_entries(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let m2 = Matrix::with_entries(5, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

        assert_eq!(
            (m1 + m2).entries,
            vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
    }

    #[test]
    fn add_assign() {
        let mut m1 = Matrix::with_entries(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let m2 = Matrix::with_entries(5, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

        let a: Matrix<i32> = Matrix::with_dimensions(5, 10);
        a.is_identity();

        m1 += m2;
        assert_eq!(m1.entries, vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);
    }
}
