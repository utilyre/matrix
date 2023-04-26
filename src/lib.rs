use std::slice::Iter;

#[derive(Clone)]
pub struct Cursor<I> {
    iter: I,

    width: usize,
    i: usize,
    j: usize,
}

impl<I> Cursor<I> {
    fn new(iter: I, width: usize) -> Self {
        Self {
            iter,
            width,
            i: 0,
            j: 0,
        }
    }
}

impl<I> Iterator for Cursor<I>
where
    I: Iterator,
{
    type Item = ((usize, usize), I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next();

        let i = self.i;
        let j = self.j;

        self.j += 1;
        if self.j == self.width {
            self.i += 1;
            self.j = 0;
        }

        a.map(|a| ((i, j), a))
    }
}

trait CursorIter<T> {
    fn cursor(self, width: usize) -> Cursor<Self>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Cursor::new(self, width)
    }
}

impl<I, T> CursorIter<T> for I where I: Iterator<Item = T> {}

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
