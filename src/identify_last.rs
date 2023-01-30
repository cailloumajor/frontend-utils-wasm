// https://users.rust-lang.org/t/iterator-need-to-identify-the-last-element/8836/9

use std::iter::Peekable;

pub trait IdentifyLast: Iterator + Sized {
    fn identify_last(self) -> Iter<Self>;
}

impl<I> IdentifyLast for I
where
    I: Iterator,
{
    fn identify_last(self) -> Iter<Self> {
        Iter(self.peekable())
    }
}

pub struct Iter<I: Iterator>(Peekable<I>);

impl<I> Iterator for Iter<I>
where
    I: Iterator,
{
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|e| (e, self.0.peek().is_none()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identify_last() {
        let mut iterator = [1, 2, 3, 4].into_iter().identify_last();
        assert_eq!(iterator.next(), Some((1, false)));
        assert_eq!(iterator.next(), Some((2, false)));
        assert_eq!(iterator.next(), Some((3, false)));
        assert_eq!(iterator.next(), Some((4, true)));
        assert_eq!(iterator.next(), None);
    }
}
