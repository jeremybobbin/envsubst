use std::iter::{
    Iterator,
    Peekable,
};

pub struct PeekWhile<I, P>
    where P: Fn(&I::Item) -> bool,
          I: PeekableIterator
{
    iter: I,
    predicate: P,
}

pub trait PeekableIterator: Iterator {

    fn peek(&mut self) -> Option<&Self::Item>;

    fn peek_while<P>(self, predicate: P) -> PeekWhile<Self, P>
        where P: Fn(&Self::Item) -> bool,
              Self: Sized
    {
        PeekWhile::new(self, predicate)
    }
}



impl<I, P> PeekWhile<I, P>
    where P: Fn(&I::Item) -> bool,
          I: PeekableIterator
{
    fn new(iter: I, predicate: P) -> PeekWhile<I, P> {
        PeekWhile {
            iter,
            predicate,
        }
    }
}

impl<I, P> Iterator for PeekWhile<I, P>
    where P: Fn(&I::Item) -> bool,
          I: PeekableIterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let item = self.iter.peek()?;
        if !(self.predicate)(item) {
            return None
        }
        self.iter.next()
    }
}


// BoilerTRAIT
impl<I: Iterator> PeekableIterator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

impl<'a, I: PeekableIterator + ?Sized> PeekableIterator for &'a mut I {
    fn peek(&mut self) -> Option<&I::Item> {
        (**self).peek()
    }
}
