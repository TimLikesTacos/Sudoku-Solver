mod flag;
mod value;
pub mod flag_limits;

use std::convert::TryFrom;
use flag_limits::{IntLimits, FlagLimits};
use std::ops::Add;
use crate::sq_element::value::Value;
use crate::sq_element::flag::Flag;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagType<F: Flag> {
    flags: F,
    count: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ValueType<V: Value> {
    value: V
}

pub trait IncReset {
    fn inc(&mut self) -> bool;
    fn reset (&mut self);
}


impl <F:Flag> FlagType<F> {
    fn count_ones (flags: F) -> u8 {
        let mut f = flags;
        let mut count = 0;
        while f > F::ZERO {
            f = f & (f - F::ONE);
            count += 1;
        }
        count
    }

    fn merge (&self, slice: &[Self]) -> Self {
        let res = slice.iter().fold(self.flags, |acc, x| acc | x.flags);
        Self {
            flags: res,
            count: Self::count_ones(res),
        }
    }

    fn set_from_value <V: Value> (&mut self, v_slice: &[ValueType<V>]) {
        let merged = v_slice.iter()
            .fold(F::ZERO, |mut acc, x| acc | Self::from(*x).flags);

        self.flags = merged;
        self.count = Self::count_ones(merged);
    }

    fn set (&mut self, f: F) {
        self.flags = f;
        self.count = Self::count_ones(f);
    }
}
//
// pub trait FlagTrait: PartialEq
// where
//     Self: Sized,
// {
//     type IntForFlag;
//     /// Contains all 1's in the size from 1 to MAX_NUM. Used for bitwise negation.
//     const NEG: Self::IntForFlag;
//
//     fn get_flags(&self) -> Self::IntForFlag;
//     fn add_flag(&self, v: Self) -> Self;
//     fn add_num(&mut self, v: Self::IntForFlag) -> Self;
//     fn remove_flag(&mut self, v: Self) -> Self;
//     fn remove_num(&mut self, v: Self::IntForFlag) -> Self;
//     fn clear(&mut self);
//     fn new(v: Self::IntForFlag) -> Self;
//     fn count(&self) -> u8;
//     fn bits() -> u8;
//     fn merge(slice: &[Self]) -> Self;
//     fn is_single(&self) -> bool;
//     fn set_initial(present_values: Self) -> Self;
// }
//
// /**
// todo: macro-ize
// **/
//
// impl FlagTrait for FlagType<u16> {
//     type IntForFlag = u16;
//     const NEG: u16 = 0b111111111;
//
//     fn get_flags(&self) -> Self::IntForFlag {
//         self.flags
//     }
//
//     fn add_flag(&self, v: FlagType<u16>) -> Self {
//         let mut f = self.flags;
//         f |= v.flags;
//         let mut count: u8 = 0;
//         let mut n = f;
//         while n != 0 {
//             n = n & (n - 1);
//             count += 1;
//         }
//
//         FlagType {
//             flags: f,
//             count: count,
//         }
//     }
//
//     fn add_num(&mut self, v: u16) -> Self {
//         let add = FlagType::from(v as usize).flags;
//         if add & self.flags != add {
//             self.count += 1;
//             self.flags |= add;
//         }
//         self.clone()
//     }
//
//     fn remove_flag(&mut self, v: FlagType<u16>) -> Self {
//         self.flags &= !v.flags;
//         let mut n = self.flags;
//         let mut count: u8 = 0;
//         while n != 0 {
//             n = n & (n - 1);
//             count += 1;
//         }
//         self.count = count;
//         self.clone()
//     }
//
//     fn remove_num(&mut self, v: u16) -> Self {
//         let sub = FlagType::from(v as usize).flags;
//         if self.flags & sub == sub {
//             self.flags ^= sub;
//             self.count -= 1;
//         }
//         self.clone()
//     }
//
//     fn clear(&mut self) {
//         self.flags = 0;
//         self.count = 0;
//     }
//
//     fn new(v: u16) -> Self {
//         FlagType {
//             flags: v,
//             count: v.count_ones() as u8,
//         }
//     }
//
//     fn count(&self) -> u8 {
//         self.count
//     }
//
//     fn bits() -> u8 {
//         std::mem::size_of::<u16>() as u8 * 8
//     }
//
//     fn merge(slice: &[Self]) -> Self {
//         // Start with 0, bitwise OR each flags value in slice
//         let flags: u16 = slice.iter().fold(0, |acc, x| acc | x.flags);
//         FlagType::new(flags)
//     }
//
//     fn is_single(&self) -> bool {
//         if self.count == 1 {
//             true
//         } else {
//             false
//         }
//     }
//
//     fn set_initial(present_values: Self) -> Self {
//         Self::new(present_values.flags ^ Self::NEG)
//     }
// }
//
// impl Default for FlagType<u16> {
//     fn default() -> FlagType<u16> {
//         FlagType::new(0)
//     }
// }
//
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
