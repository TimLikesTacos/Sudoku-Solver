
pub(crate) mod value;
pub(crate) mod flag;
pub(crate) mod flag_limits;

use std::convert::TryFrom;
use flag_limits::{IntLimits, FlagLimits};
use std::ops::{Add, AddAssign, SubAssign};
use crate::sq_element::value::NormalInt;
use crate::sq_element::flag::Flag;
use crate::sq_element::flag_limits::ZeroAndOne;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FlagType<F: Flag> {
    pub(crate)flags: F,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct IntType<V: NormalInt> {
    pub(crate)value: V
}

pub trait OneZero {
    type Value: ZeroAndOne;
    fn zero () -> Self;
    fn one () -> Self;
}

impl <V: NormalInt> OneZero for IntType<V>{
    type Value = V;
    fn zero() -> Self {
        Self {value: V::ZERO}
    }
    fn one() -> Self {
        Self {value: V::ONE}
    }
}
impl <F: Flag> OneZero for FlagType<F> {
    type Value = F;
    fn zero() -> Self {
        Self {flags: F::ZERO}
    }
    fn one() -> Self {
        Self {flags: F::ONE}
    }
}

pub trait SqElement: OneZero + Default + PartialEq + Into<u8> +From<u8> + Copy + Clone
{
    type Item: PartialEq;
    fn inc(&mut self) -> bool;
    fn reset (&mut self);
    fn get(&self) -> Self::Item;
    fn set(&mut self, value: Self::Item);

}

pub trait FlElement: SqElement + AddAssign + SubAssign
where Self: Sized, Self::FlagItem: FlagLimits + Flag  {
    type FlagItem;
    fn count_ones(flags: &Self::FlagItem) -> u8;
    fn merge (&self, slice:&[Self]) -> Self;
    fn set_from_value <V: NormalInt> (&mut self, v_slice: &[IntType<V>]);
    fn is_flagged (&self, other: Self) -> bool;
    fn max()-> Self;
}


impl <F:Flag> FlElement for FlagType<F> {
    type FlagItem = F;
    fn count_ones (flags: &Self::FlagItem) -> u8 {
        let mut f = *flags;
        let mut count = 0;
        while f > F::ZERO {
            f = f & (f - F::ONE);
            count += 1;
        }
        count
    }

    fn merge (&self, slice: &[Self]) -> Self {
        Self {
            flags: slice.iter().fold(self.flags, |acc, x| acc | x.flags)
        }
    }

    fn set_from_value <V: NormalInt> (&mut self, v_slice: &[IntType<V>]) {
        self.flags = v_slice.iter()
            .fold(F::ZERO, |acc, x| acc | Self::from(*x).flags);
    }

    fn is_flagged( &self, other: Self) -> bool{
        if self.flags & other.flags > F::ZERO {
            true
        } else {false}
    }

    fn max() -> Self {
        Self {flags: F::FMAX}
    }


}

//
//
// #[cfg(test)]
// mod flag_tests {
//     use super::*;
//
//     #[test]
//     fn set_test() {
//         let t1: FlagType<u16> = FlagType::new(0);
//         let mut t1 = t1.add_flag(2usize.into());
//
//         assert_eq!(usize::from(t1), 2);
//         t1 = t1.add_num(8);
//         dbg!(&t1);
//         assert_eq!(t1.flags, 0b10000010);
//     }
//
//     #[test]
//     fn remove_test() {
//         let mut t1 = FlagType::from(8usize);
//         t1 = t1.add_flag(FlagType::from(4usize));
//         assert_eq!(t1.count, 2);
//         assert_eq!(t1.flags, 0b10001000);
//         t1.remove_flag(FlagType::from(8usize));
//         assert_eq!(t1.flags, 0b1000);
//         assert_eq!(t1.count, 1);
//         t1.remove_num(1);
//         assert_eq!(t1.count, 1);
//         assert_eq!(t1.flags, 0b1000);
//         t1.remove_flag(FlagType::from(4usize));
//         assert!(t1.count == 0);
//         assert!(t1.flags == 0);
//
//         let mut t1 = FlagType::from(1usize);
//         let mut t2 = FlagType::from(4usize);
//         t1 = t1.add_flag(t2);
//         assert_eq!(t1.flags, 0b1001);
//         assert_eq!(t1.count, 2);
//         let mut t3 = FlagType::from(9usize);
//         assert_eq!(t3.flags, 0b100000000);
//         t3 = t3.add_flag(t1);
//         assert_eq!(t3.flags, 0b100001001);
//         assert_eq!(t3.count, 3);
//         t3 = t3.remove_flag(t2);
//         assert_eq!(t3.flags, 0b100000001);
//         assert_eq!(t3.count, 2);
//         t3 = t3.add_flag(t1);
//         assert_eq!(t3.flags, 0b100001001);
//         assert_eq!(t3.count(), 3);
//     }
//
//     #[test]
//     fn clear_test() {
//         let mut t1 = FlagType::new(0b1100);
//         assert_eq!(t1.count(), 2);
//         t1.clear();
//         assert_eq!(t1, 0usize.into());
//         assert_eq!(t1.count(), 0);
//     }
//
//     #[test]
//     fn from_into() {
//         let mut t1: FlagType<u16> = FlagType::from(4usize);
//         assert_eq!(t1.flags, 0b1000);
//         assert_eq!(t1.add_flag(FlagType::from(8usize)).flags, 0b10001000);
//     }
//
//     #[test]
//     fn merge_test() {
//         let flags = [
//             FlagType::new(0b10000),
//             FlagType::new(0b00000),
//             FlagType::new(0b00100),
//             FlagType::new(0b10010),
//             FlagType::new(0b00001),
//         ];
//         let res = FlagType::merge(&flags[2..]);
//         assert_eq!(res.flags, 0b10111);
//         assert_eq!(res.count, 4);
//         let res = FlagType::merge(&flags[0..=2]);
//         assert_eq!(res.flags, 0b10100);
//         assert_eq!(res.count, 2);
//     }
//
//     #[test]
//     fn flag16neg() {
//         let res: u16 = FlagType::NEG;
//         assert_eq!(res, 0b111111111);
//     }
//
//     #[test]
//     fn get_initial_test() {
//         let a = FlagType::new(0b0101);
//         let res = FlagType::set_initial(a);
//         assert_eq!(res.flags, 0b111111010);
//     }
// }
