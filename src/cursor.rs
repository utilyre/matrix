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

pub(crate) trait CursorIterator<T> {
    fn cursor(self, width: usize) -> Cursor<Self>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Cursor::new(self, width)
    }
}

impl<I, T> CursorIterator<T> for I where I: Iterator<Item = T> {}
