use core::iter::{ExactSizeIterator, Fuse, FusedIterator};

use crate::array;

/// An iterator over all contiguous windows of length `N`. The windows overlap.
/// If the iterator is shorter than `N`, the iterator returns no values.
///
/// This struct is created by the [`array_windows`] method on iterators. See its
/// documentation for more.
///
/// [`array_windows`]: crate::Itermore::array_windows
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayWindows<I, const N: usize>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: Fuse<I>,
    last: Option<[I::Item; N]>,
}

impl<I, const N: usize> ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        assert!(N != 0, "window size must be non-zero");
        Self {
            iter: iter.fuse(),
            last: None,
        }
    }
}

impl<I: Iterator, const N: usize> Iterator for ArrayWindows<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, last } = self;

        match last {
            Some(last) => {
                let item = iter.next()?;
                last.rotate_left(1);
                if let Some(end) = last.last_mut() {
                    *end = item;
                }
                Some(last.clone())
            }
            None => {
                let tmp = array::collect(iter)?;
                *last = Some(tmp.clone());
                Some(tmp)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        // Keep infinite iterator size hint lower bound as `usize::MAX`
        if lower == usize::MAX {
            (lower, upper)
        } else {
            (
                lower.saturating_sub(N - 1),
                upper.map(|n| n.saturating_sub(N - 1)),
            )
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count().saturating_sub(N - 1)
    }
}

impl<I, const N: usize> FusedIterator for ArrayWindows<I, N>
where
    I: FusedIterator,
    I::Item: Clone,
{
}

impl<I, const N: usize> ExactSizeIterator for ArrayWindows<I, N>
where
    I: ExactSizeIterator,
    I::Item: Clone,
{
}
