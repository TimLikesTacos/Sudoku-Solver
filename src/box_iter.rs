use crate::constants::*;
use std::iter::{Skip};
use std::slice::{Iter, IterMut};

/// Struct used to handle iterating over a 3x3 box (for standard game size grid).
pub struct  BoxIter<'a, T> {
    pub(crate) it: Skip<Iter<'a, T>>,
    pub(crate) index: usize,
}

/// Mutable version of BoxIter
pub struct BoxIterMut<'a, T> {
    pub(crate) it: Skip<IterMut<'a, T>>,
    pub(crate) index: usize,
}

impl<'a, T> Iterator for BoxIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.index += 1;
        // This check prevents skipping to next row in the upcoming match statement
        if self.index == 1 {
            return self.it.next();
        }

        match self.index {
            v if v > MAX_NUM => None,
            v if v % BOX_DIMEN == 1 => self.it.nth(MAX_NUM - BOX_DIMEN),
            _ => self.it.next(),
        }
    }
}

impl<'a, T> Iterator for BoxIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.index += 1;
        // This check prevents skipping to next row in the upcoming match statement
        if self.index == 1 {
            return self.it.next();
        }

        match self.index {
            v if v > MAX_NUM => None,
            v if v % BOX_DIMEN == 1 => self.it.nth(MAX_NUM - BOX_DIMEN),
            _ => self.it.next(),
        }
    }
}