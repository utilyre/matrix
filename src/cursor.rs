#[derive(Debug)]
pub struct Cursor<I> {
    iter: I,

    breakat: usize,
    i: usize,
    j: usize,
}

impl<I> Cursor<I> {
    fn new(iter: I, breakat: usize) -> Self {
        Self {
            iter,
            breakat,
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
        if self.j == self.breakat {
            self.i += 1;
            self.j = 0;
        }

        a.map(|a| ((i, j), a))
    }
}

pub(crate) trait CursorIterator<T> {
    fn cursor(self, breakat: usize) -> Cursor<Self>
    where
        Self: Sized + Iterator<Item = T>,
    {
        Cursor::new(self, breakat)
    }
}

impl<I, T> CursorIterator<T> for I where I: Iterator<Item = T> {}
