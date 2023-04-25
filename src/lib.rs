#[derive(Clone)]
pub struct Cursor {
    width: usize,

    pub i: usize,
    pub j: usize,
}

impl Cursor {
    fn new(width: usize) -> Self {
        Self { width, i: 0, j: 0 }
    }

    fn tick(&mut self) -> usize {
        self.j += 1;
        if self.j == self.width {
            self.i += 1;
            self.j = 0;
        }

        self.i * self.width + self.j
    }
}

pub struct MatrixIter<'a, T> {
    elements: &'a [T],
    cursor: Cursor,
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = (Cursor, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor.clone();
        self.elements
            .get(self.cursor.tick() - 1)
            .map(|element| (cursor, element))
    }
}

pub struct Matrix<T> {
    width: usize,
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, elements: Vec<T>) -> Self {
        Self { width, elements }
    }

    pub fn iter(&self) -> MatrixIter<T> {
        MatrixIter {
            elements: &self.elements,
            cursor: Cursor::new(self.width),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let matrix = Matrix::new(3, vec![1, 2, 3, 4, 5, 6]);
    }
}
