use crate::sq_element::*;
use crate::sq_element::flag::{FlagType, Flag};
use crate::sq_element::int::NormalInt;
use std::fmt::{Display, Formatter};
use crate::sq_element::flag_limits::{ZeroAndOne, FlagLimits};
use std::fmt;
use std::ops::{AddAssign, SubAssign};


#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct IntType<V: NormalInt> {
    pub(crate) value: V,
}

pub trait OneZero {
    type Value: ZeroAndOne;
    fn zero() -> Self;
    fn one() -> Self;
}

impl <V: NormalInt> Display for IntType<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl <V: Flag> Display for FlagType<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.flags)
    }
}

impl<V: NormalInt> OneZero for IntType<V> {
    type Value = V;
    fn zero() -> Self {
        Self { value: V::ZERO }
    }
    fn one() -> Self {
        Self { value: V::ONE }
    }
}
impl<F: Flag> OneZero for FlagType<F> {
    type Value = F;
    fn zero() -> Self {
        Self { flags: F::ZERO }
    }
    fn one() -> Self {
        Self { flags: F::ONE }
    }
}

pub trait SqElement: OneZero + Default + PartialEq + Into<u8> + From<u8> + Copy + Clone {
    type Item: PartialEq;
    fn inc(&mut self) -> bool;
    fn reset(&mut self);
    fn get(&self) -> Self::Item;
    fn set<V: SqElement>(&mut self, value: V)
        where Self: From<V>;
}

pub trait FlElement: SqElement + AddAssign + SubAssign
    where
        Self: Sized,
        Self::FlagItem: FlagLimits + Flag,
{
    type FlagItem;
    fn count_ones(flags: &Self::FlagItem) -> u8;
    fn merge(&self, slice: &[Self]) -> Self;
    fn set_from_value<V: NormalInt>(&mut self, v_slice: &[IntType<V>]);
    fn is_flagged(&self, other: Self) -> bool;
    fn max() -> Self;
}

impl<F: Flag> FlElement for FlagType<F> {
    type FlagItem = F;
    fn count_ones(flags: &Self::FlagItem) -> u8 {
        let mut f = *flags;
        let mut count = 0;
        while f > F::ZERO {
            f = f & (f - F::ONE);
            count += 1;
        }
        count
    }

    fn merge(&self, slice: &[Self]) -> Self {
        Self {
            flags: slice.iter().fold(self.flags, |acc, x| acc | x.flags),
        }
    }

    fn set_from_value<V: NormalInt>(&mut self, v_slice: &[IntType<V>]) {
        self.flags = v_slice
            .iter()
            .fold(F::ZERO, |acc, x| acc | Self::from(*x).flags);
    }

    fn is_flagged(&self, other: Self) -> bool {
        if self.flags & other.flags > F::ZERO {
            true
        } else {
            false
        }
    }

    fn max() -> Self {
        Self { flags: F::FMAX }
    }
}