use crate::sq_element::*;
use crate::sq_element::flag::{Flag, FlagElement};
use crate::sq_element::int::{ IntValue};
use std::fmt::{Display, Formatter};
use crate::sq_element::flag_limits::{ZeroAndOne, FlagLimits};
use std::fmt;
use std::ops::{AddAssign, SubAssign};

pub trait OneZero {
    type Value: ZeroAndOne;
    fn zero() -> Self;
    fn one() -> Self;
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
        Self::FlagItem: FlagLimits + FlagElement,
{
    type FlagItem;
    fn count_ones(flags: &Self::FlagItem) -> u8;
    fn merge(&self, slice: &[Self]) -> Self;
    fn set_from_value(&mut self, v_slice: &[u8]);
    fn is_flagged(&self, other: &Self) -> bool;
    // fn remove_flag(&mut self, other: Self);
    fn max() -> Self;
}

