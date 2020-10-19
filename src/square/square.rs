
use crate::flag::*;
use std::ops::Add;
use crate::square::flag_limits::*;
//use crate::square::square_ops::{Flag, ValueTr};
use crate::square::flag_limits::{IntLimits, FlagLimits};
//use crate::square::square::{Flag, ValueTr};


#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FlagSquare<F: Flag> {
    pub(crate) value: F,
    pub(crate) fixed: bool,
    pub(crate) pencil: F,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SimpleSquare<V: ValueTr> {
    pub(crate)value: V,
    pub(crate)fixed: bool,
}

pub trait ValueTr: Default + Copy + Clone + IntLimits + Add<Output = Self> + PartialEq + TryFrom<usize>{}
impl ValueTr for u8 {}
impl ValueTr for u16 {}

pub trait Flag: Default + Copy + Clone + FlagLimits + Add<Output = Self> + PartialEq + TryFrom<usize>{}
impl Flag for u16 {}
impl Flag for u32 {}

pub trait IncReset<V: ValueTr>{
    fn inc(&mut self) -> bool;
    fn reset (&mut self);
}



// impl<F: Flag, V: Value> PartialEq<FlagSquare<F>> for SimpleSquare<V> {
//     fn eq(&self, other: &FlagSquare<V, F>) -> bool {
//         self.value == other.value
//     }
// }

// impl<V: Clone + PartialEq + From<u8> + Add<Output=V>, F: FlagTrait> PartialEq<SimpleSquare<V>> for FlagSquare<V, F> {
//     fn eq(&self, other: &SimpleSquare<V>) -> bool {
//         self.value == other.value
//     }
// }
//
pub trait SquareTrait: Default + Clone + PartialEq {
    type Value: Valu;
    fn setv(&mut self, v: Self::Value);
    fn getv(&self) -> Self::Value;
    fn input_convert(int: u8) -> Self::Value;
    fn has_flags() -> bool;
    fn fixed(&self) -> bool;
    fn new(v: Self::Value, fix: bool) -> Self;
    fn reset_value(&mut self) {
        if !self.fixed() {
            self.setv(Self::Value::from(0u8))
        }
    }
    fn inc(&mut self) {
        if self.getv() == Self::Value::MAX{}
        let new = self.getv() + Self::Value::from(1);
        self.setv(new);
    }
}
//
// impl<V: Clone + PartialEq + From<u8> + Default + Add<Output=V>, F: FlagTrait + Default + Clone> SquareTrait
// for FlagSquare<V, F>
// {
//     type Value = V;
//
//     fn setv(&mut self, v: Self::Value) {
//         self.value = v;
//     }
//     fn fixed(&self) -> bool { self.fixed }
//     fn getv(&self) -> Self::Value {
//         self.value.clone()
//     }
//     fn input_convert(int: u8) -> Self::Value {
//         V::from(int)
//     }
//     fn has_flags() -> bool { true }
//     fn new(v: V, fix: bool) -> FlagSquare<V, F> {
//         FlagSquare {
//             value: v,
//             fixed: fix,
//             pencil: F::default(),
//         }
//     }
// }
//
// impl<V: Clone + PartialEq + From<u8> + Default + Add<Output=V>> SquareTrait for SimpleSquare<V> {
//     type Value = V;
//
//     fn setv(&mut self, v: Self::Value) {
//         self.value = v;
//     }
//     fn getv(&self) -> Self::Value {
//         self.value.clone()
//     }
//     fn fixed(&self) -> bool { self.fixed }
//     fn input_convert(int: u8) -> Self::Value {
//         V::from(int)
//     }
//     fn has_flags() -> bool { false }
//     fn new(v: V, fix: bool) -> Self {
//         SimpleSquare {
//             value: v,
//             fixed: fix
//         }
//     }
// }
// // fn is_all_values(&self, v: Vec<Self::Value>) -> bool {
// //     if v.len() != MAX_NUM {
// //         return false;
// //     }
// //     for i in 1..=MAX_NUM {
// //         if !v.contains(Self::Value::try_from(i).unwrap()) {
// //             return false;
// //         }
// //     }
// //     true
// // }
//
//
// pub trait SquareFlagTrait: SquareTrait {
//     type FlagType;
//
//     fn fix(&mut self);
//     fn setp(&mut self, p: Self::FlagType);
//     fn getp(&self) -> &Self::FlagType;
//     fn getp_mut(&mut self) -> &mut Self::FlagType;
//     fn update_flags(&mut self, slice: &[Self::FlagType]);
//
//     fn initial_setp(&mut self, slice: &[Self::Value]);
// }
//
// impl<V: Clone + PartialEq + From<u8> + Default + Add<Output=V>, F: Clone + From<V>> SquareFlagTrait for FlagSquare<V, F>
//     where
//         F: FlagTrait + Default,
// {
//     type FlagType = F;
//
//     fn fix(&mut self) { self.fixed = true; }
//     fn getp(&self) -> &Self::FlagType {
//         &self.pencil
//     }
//     fn getp_mut(&mut self) -> &mut Self::FlagType {
//         &mut self.pencil
//     }
//
//     fn setp(&mut self, p: Self::FlagType) {
//         self.pencil = p;
//     }
//
//
//     fn update_flags(&mut self, slice: &[Self::FlagType]) {
//         let values = F::merge(&slice);
//         self.pencil = F::set_initial(values);
//     }
//
//     fn initial_setp(&mut self, slice: &[V]) {
//         let change_type = slice.iter().fold(Vec::new(), |mut acc, x: &V| {
//             acc.push(<F>::from(x.clone()));
//             acc
//         });
//         self.update_flags(&change_type);
//     }
// }
//
// ///* `From` is implemented for FlagSquare to SimpleSquare, but not the other way as data would
// /// be lost for the flag values.  Keep this in mind when using other functions that have `from()`
// /// for different square functions *///
// impl<V: Clone + From<u8> + Add<Output=V>, F: FlagTrait> From<&FlagSquare<V, F>> for SimpleSquare<V> {
//     fn from(other: &FlagSquare<V, F>) -> SimpleSquare<V> {
//         SimpleSquare {
//             value: other.value.clone(),
//             fixed: other.fixed,
//         }
//     }
// }
//
// impl<V: Clone + From<u8> + Add<Output=V>, F: FlagTrait + Clone> From<&FlagSquare<V, F>> for FlagSquare<V, F> {
//     fn from(other: &FlagSquare<V, F>) -> FlagSquare<V, F> {
//         FlagSquare {
//             value: other.value.clone(),
//             fixed: other.fixed,
//             pencil: other.pencil.clone(),
//         }
//     }
// }

// impl<V:Clone, F: FlagTrait> From <&SimpleSquare<V>> for F>lagSquare<V, F> {
//
// }
// #[cfg(test)]
// mod square_tests {
//     use super::*;
//     use crate::flag::FlagTrait;
//
//     #[test]
//     fn new_test() {
//         let s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(5usize).get_flags(), true);
//         let t: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(7usize).get_flags(), false);
//         assert_eq!(s.pencil.count(), 0);
//         assert_eq!(usize::from(s.value), 2usize.pow(5 - 1));
//         assert_eq!(s.fixed, true);
//         assert_eq!(t.pencil.count(), 0);
//         assert_eq!(t.fixed, false);
//         assert_eq!(usize::from(t.value), 2usize.pow(7 - 1));
//     }
//
//     #[test]
//     fn setv_test() {
//         let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0usize).get_flags(), false);
//         s.setv(Flag::from(4usize).get_flags());
//         assert_eq!(usize::from(s.getv()), 2usize.pow(4 - 1));
//     }
//
//     #[test]
//     fn setp_test() {
//         let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0usize).get_flags(), false);
//         s.setp(Flag::from(0b1110usize));
//         assert_eq!(*s.getp(), Flag::from(0b1110usize));
//     }
//
//     #[test]
//     fn mutp_test() {
//         let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0usize).get_flags(), false);
//         s.setp(Flag::new(0b1110));
//         let mut p = s.getp_mut();
//         *p = p.remove_flag(Flag::new(0b10000010));
//         assert_eq!(*s.getp(), Flag::new(0b1100));
//     }
// }


