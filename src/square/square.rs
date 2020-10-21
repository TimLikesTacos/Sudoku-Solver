
use crate::sq_element::*;
use std::ops::Add;
use crate::sq_element::flag_limits::*;
use crate::sq_element::flag_limits::{IntLimits, FlagLimits};
use crate::sq_element::value::Value;
use crate::sq_element::flag::Flag;

use crate::sq_element::*;
use crate::sq_element;


#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FlagSquare<E: SqElement, F: FlElement> {
    pub(crate) value: E,
    pub(crate) fixed: bool,
    pub(crate) flags: F,
    pub(crate) count :u8,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SimpleSquare<E: SqElement> {
    pub(crate)value: E,
    pub(crate)fixed: bool,
}

pub trait  Square  where Self::Value: Sized {
    type Value;
    fn set(&mut self, v: Self::Value);
    fn getv(&self) -> Self::Value;
    fn exportv(&self) -> u8;
    fn has_flags(&self) -> bool;
    fn fixed(&self) -> bool;
    fn new(v: u8, fix: bool) -> Self;
    fn inc(&mut self) -> bool;
    fn reset_value(&mut self);
}


impl<V: Value, F: Flag> Square for FlagSquare<IntType<V>, FlagType<F>>
{
    type Value = V;
    fn set (&mut self, v: Self::Value) {
        self.value.set(v)
    }
    fn fixed(&self) -> bool { self.fixed }
    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }
    fn has_flags(&self) -> bool { true }
    /// Does not set flags
    fn new(v: u8, fix: bool) -> FlagSquare<IntType<V>, FlagType<F>> {
        FlagSquare {
            value: IntType::from(v),
            fixed: fix,
            flags: FlagType::default(),
            count: 0,
        }
    }

    fn inc(&mut self) -> bool {
        // convert int to flag
        let mut f:FlagType<F> = FlagType::from(self.value);
        let old_copy = f;
        // increment once
        let mut not_maxed = f.inc();
        // increment until matches possible in flag
        while (!self.flags.is_flagged(f) && not_maxed) {
            not_maxed = f.inc();
        }

        if not_maxed {
            if old_copy.flags != F::ZERO {
                //put back old value into possiblities
                self.flags += old_copy;
                self.count += 1;
            }
            //remove from possiblities
            self.flags -= f;
            // Remove current value from possibilities
            self.count -= 1;
            self.value = IntType::from(f);
        }
        not_maxed
    }

    fn reset_value(&mut self) {
        // but back into possibilities
        if self.value.get() != V::ZERO {
            self.count += 1;
            self.flags += FlagType::from(self.value);
        }
        // Set to zero
        self.value.reset()
    }
}

impl<F: Flag + From<u8>> Square for FlagSquare<FlagType<F>, FlagType<F>>
{
    type Value = F;
    fn fixed(&self) -> bool { self.fixed }
    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }
    fn has_flags(&self) -> bool { true }
    /// Does not set flags
    fn new(v: u8, fix: bool) -> FlagSquare<FlagType<F>, FlagType<F>> {
        FlagSquare {
            value: FlagType::from(v),
            fixed: fix,
            flags: FlagType::default(),
            count: 0,
        }
    }

    fn inc(&mut self) -> bool {
        // convert int to flag
        let mut f:FlagType<F> = self.value.clone();
        let old_copy = f.clone();
        // increment onces
        let mut not_maxed = f.inc();

        // increment until matches possible in flag
        while (self.flags & f).get() == F::ZERO && not_maxed {
            not_maxed = f.inc();
        }
        if not_maxed {
            if old_copy.flags != F::ZERO {
                //put back into possiblities
                self.flags += old_copy;
                self.count += 1;
            }
            //remove from possiblities
            self.flags -= f;
            self.count -= 1;
            // convert back
            self.value = FlagType::from(f);
        }
        not_maxed
    }

    fn reset_value(&mut self) {
        // but back into possibilities
        if self.value.get() != F::ZERO {
            self.count += 1;
            self.flags += self.value;
        }
        // Set to zero
        self.value.reset()
    }

    fn set(&mut self, v: Self::Value) {
        self.value = FlagType::from(v)
    }
}

impl<V: Value> Square  for SimpleSquare<IntType<V>>
{
    type Value = V;

    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }
    fn fixed(&self) -> bool { self.fixed }

    fn has_flags(&self) -> bool { false }
    fn new(v: u8, fix: bool) -> Self {
        SimpleSquare {
            value: IntType::from(v),
            fixed: fix
        }
    }

    fn inc(&mut self) -> bool {
        self.value.inc()
    }

    fn reset_value(&mut self) {
        self.value.reset()
    }

    fn set(&mut self, v: Self::Value) {
        self.value.set(v)
    }
}

impl<F: Flag> Square for SimpleSquare<FlagType<F>> {
    type Value = F;

    fn set(&mut self, v: Self::Value) {
        self.value.set(v)
    }

    fn getv(&self) -> Self::Value {
        self.value.get()
    }

    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }

    fn has_flags(&self) -> bool {
        false
    }

    fn fixed(&self) -> bool {
        self.fixed
    }

    fn new(v: u8, fix: bool) -> Self {
        SimpleSquare {
            value: FlagType::from(v),
            fixed: fix
        }
    }

    fn inc(&mut self) -> bool {
        self.value.inc()
    }

    fn reset_value(&mut self) {
        self.value.reset()
    }
}
#[cfg(test)]
mod square_tests {
    use super::*;


    #[test]
    fn new_test() {
        let a: SimpleSquare<IntType<u16>> = SimpleSquare {
            value: <IntType<u16>>::from(4),
            fixed: true
        };
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), true);
        let a: SimpleSquare<IntType<u16>> = SimpleSquare::new(4u8, false);
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), false);
        let a: SimpleSquare<FlagType<u16>> = SimpleSquare::new(4u8, true);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), true);
        assert_eq!(a.has_flags(), false);
        let a: FlagSquare<IntType<u16>, FlagType<u16>> = FlagSquare::new(4u8, true);
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), true);
        assert_eq!(a.has_flags(), true);
        let a: FlagSquare<FlagType<u32>, FlagType<u32>> = FlagSquare::new(4u8, false);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), false);
        assert_eq!(a.has_flags(), true);
    }

    #[test]
    fn inc_reset_test() {
        let mut s: FlagSquare<IntType<u8>, FlagType<u16>> = FlagSquare {
            value: IntType { value: 0 },
            fixed: false,
            flags: FlagType { flags: 0b100010010 },
            count: 3,
        };
        assert_eq!(s.getv(), 0);
        assert_eq!(s.flags.get(), 0b100010010);
        assert_eq!(s.count, 3);

        assert!(s.inc());
        assert_eq!(s.getv(), 2);
        assert_eq!(s.flags.get(), 0b100010000);
        assert_eq!(s.count, 2);

        assert!(s.inc());
        assert_eq!(s.getv(), 5);
        assert_eq!(s.flags.get(), 0b100000010);
        assert_eq!(s.count, 2);

        assert!(s.inc());
        assert_eq!(s.getv(), 9);
        assert_eq!(s.flags.get(), 0b000010010);
        assert_eq!(s.count, 2);

        // Check that false is returned when maxxed out.
        assert!(!s.inc());
        assert_eq!(s.getv(), 9);
        assert_eq!(s.flags.get(), 0b000010010);
        assert_eq!(s.count, 2);

        s.reset_value();
        assert_eq!(s.getv(), 0);
        assert_eq!(s.count, 3);
        assert_eq!(s.flags.get(), 0b100010010);

        let mut s: FlagSquare<IntType<u8>, FlagType<u16>> = FlagSquare {
            value: IntType { value: 0 },
            fixed: false,
            flags: FlagType { flags: 0b010010010 },
            count: 3,
        };
        assert!(s.inc());
        assert!(s.inc());
        assert!(s.inc());

        assert_eq!(s.getv(), 8);
        assert_eq!(s.flags.get(), 0b000010010);
        assert_eq!(s.count, 2);

        // maxed
        assert!(!s.inc());
        assert_eq!(s.getv(), 8);
        assert_eq!(s.count, 2);
        assert_eq!(s.flags.get(), 0b000010010);

        s.reset_value();
        assert_eq!(s.getv(), 0);
        assert_eq!(s.count, 3);
        assert_eq!(s.flags.get(), 0b010010010);

        // Different types of flagsquare
        let mut s: FlagSquare<FlagType<u16>, FlagType<u16>> = FlagSquare {
            value: FlagType { flags: 0 },
            fixed: false,
            flags: FlagType { flags: 0b010010010 },
            count: 3,
        };
        assert!(s.inc());
        assert_eq!(s.getv(), 0b10);
        assert!(s.inc());
        assert_eq!(s.getv(),0b10000);
        assert!(s.inc());

        assert_eq!(s.getv(), 0b10000000);
        assert_eq!(s.flags.get(), 0b000010010);
        assert_eq!(s.count, 2);

        // maxed
        assert!(!s.inc());
        assert_eq!(s.getv(), 0b10000000);
        assert_eq!(s.count, 2);
        assert_eq!(s.flags.get(), 0b000010010);

        s.reset_value();
        assert_eq!(s.getv(), 0);
        assert_eq!(s.count, 3);
        assert_eq!(s.flags.get(), 0b010010010);

    }

    //
    // #[test]
    // fn setp_test() {
    //     let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0usize).get_flags(), false);
    //     s.setp(Flag::from(0b1110usize));
    //     assert_eq!(*s.getp(), Flag::from(0b1110usize));
    // }
    //
    // #[test]
    // fn mutp_test() {
    //     let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0usize).get_flags(), false);
    //     s.setp(Flag::new(0b1110));
    //     let mut p = s.getp_mut();
    //     *p = p.remove_flag(Flag::new(0b10000010));
    //     assert_eq!(*s.getp(), Flag::new(0b1100));
    // }
}

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
// /// be lost for the sq_element values.  Keep this in mind when using other functions that have `from()`
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
