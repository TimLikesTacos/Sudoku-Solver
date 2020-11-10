use crate::sq_element::flag::{Flag, FlagElement};
use crate::sq_element::flag_limits::{FlagLimits, ZeroAndOne};
use crate::sq_element::int::IntValue;
use crate::sq_element::*;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{AddAssign, BitAnd, BitOr, BitXor, Shl, Sub, SubAssign};

pub trait OneZero {
    type Value: ZeroAndOne;
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait SqElement:
    OneZero + Default + PartialEq + PartialOrd + Into<u8> + From<u8> + Copy + Clone
{
    type Item: PartialEq;
    fn inc(&mut self) -> bool;
    fn reset(&mut self);
    fn get(&self) -> Self::Item;
    fn set<V: SqElement>(&mut self, value: V)
    where
        Self: From<V>;
}

pub trait FlElement:
    SqElement
    + AddAssign
    + SubAssign
    + Ord
    + BitAnd<Output = Self>
    + Sub<Output = Self>
    + Shl<Output = Self>
    + From<usize>
    + BitXor<Output = Self>
    + BitOr<Output = Self>
    + Debug
where
    Self: Sized,
    Self::FlagItem: FlagLimits + FlagElement,
{
    type FlagItem;
    fn count_ones(flags: &Self) -> u8;
    fn merge(&self, slice: &[Self]) -> Self;
    fn set_from_value(&mut self, v_slice: &[u8]);
    fn is_flagged(&self, other: &Self) -> bool;
    // fn remove_flag(&mut self, other: Self);
    fn max() -> Self;
}
