use crate::sq_element::*;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{Shl, Shr, BitAnd, Sub, BitOr, BitXor};


pub trait Flag: Default + Copy + Debug + Clone + FlagLimits + Add<Output = Self>
     + Sub<Output = Self> + PartialEq + PartialOrd + Into<u64> + TryFrom<usize> + BitAnd<Output = Self>
     + Shr<Output = Self> + Shl<Output = Self> + BitOr<Output = Self> + BitXor <Output = Self> {}

impl Flag for u16 {}
impl Flag for u32 {}

// /**
// todo: macro-ize this to allow for different primitive integers
// **/
// // Convert from usize to flag.  Useful for input
// impl From<usize> for FlagType<u16> {
//     fn from(item: usize) -> FlagType<u16> {
//         if item == 0 {
//             FlagType { flags: 0, count: 0 }
//         } else {
//             FlagType {
//                 flags: 1 << (item - 1),
//                 count: 1,
//             }
//         }
//     }
// }

impl <F: Flag> From<usize> for FlagType<F> {
    fn from(item: usize) -> FlagType<F> {

        /*
        todo: clean this up correctly, ensure values within bands
         */
        assert!(item as u64 <= F::FMAX.into()) ;
        if item == 0 {
            FlagType { flags: F::ZERO, count: 0 }
        } else {
            FlagType {
                flags: F::try_from(1 << (item - 1)).unwrap_or_default(),
                count: 1,
            }
        }
    }
}

// Convert back to usize
impl <F: Flag> From<FlagType<F>> for usize {
    fn from(item: FlagType<F>) -> usize {
        if item.count != 1 {
            0
        } else {
            let mut count: usize = 0;
            let mut f = item.flags;
            while f != F::ZERO {
                f = f >> F::ONE;
                count += 1;
            }
            count
        }

    }
}
// From u16 for Flagtype
impl From<u16> for FlagType<u16> {
    fn from(item: u16) -> FlagType<u16> {
        if item == 0 {
            FlagType { flags: 0, count: 0 }
        } else {
            FlagType {
                flags: 1 << (item - 1),
                count: 1,
            }
        }
    }
}

impl From<FlagType<u16>> for u16 {
    fn from(item: FlagType<u16>) -> u16 {
        if item.count != 1 {
            0
        } else {
            /*
            todo: proper error handling
             */
            u16::try_from(item.flags.trailing_zeros() + 1).unwrap()
        }
    }
}

impl <V: Value, F: Flag> From <ValueType<V>> for FlagType<F> {
    fn from(other: ValueType<V>) -> Self {
        let v = usize::from(other);
        FlagType::from(v)
    }
}

// Increment and reset flagtypes
impl <F: Flag> IncReset for FlagType<F> {
    fn inc (&mut self) -> bool {
        if self.flags == F::VMAX {
            false
        } else {
            let old = self.flags;
            self.flags = old << F::ONE;
            true
        }
    }

    fn reset (&mut self) {
            self.flags = F::ZERO;
            self.count = 0;

    }
}

impl <F: Flag> Add for FlagType<F> {
    type Output = Self;
    fn add (self, other: Self) -> Self {
        let res = self.flags | other.flags;
        Self {
            flags: res,
            count: Self::count_ones(res)
        }
    }
}

impl <F: Flag> Sub for FlagType<F> {
    type Output = Self;
    fn sub (self, other: Self) -> Self {
        let res = self.flags & (other.flags ^ F::FMAX);
        Self {
            flags: res,
            count: Self::count_ones(res)
        }
    }
}



#[cfg(test)]
mod flag_tests {
    use crate::sq_element::{FlagType, ValueType};
    use super::*;

    #[test]
    fn froms () {
        let flag: FlagType<u16> = FlagType::from(4usize);
        assert_eq!(flag.flags, 0b1000);
        assert_eq!(usize::from(flag), 4);
        assert_eq!(<ValueType<u8>>::from(flag).value, 4);
        let val: ValueType<u8> = ValueType::from(6);
        assert_eq!(val.value, 6);
        let mut flag: FlagType<u32> = FlagType::from(val);
        assert_eq!(flag.flags, 0b100000);
        assert_eq!(usize::from(flag), 6);
        flag.flags = 0b1100;
        flag.count = 2;
        assert_eq!(<ValueType<u8>>::from(flag), ValueType::from(0));

    }

    #[test]
    fn inc_and_reset() {
        let mut flag: FlagType<u16> = FlagType::from(8usize);
        assert_eq!(flag.flags, 0b10000000);
        assert!(flag.inc());
        assert_eq!(flag.flags, 0b100000000);
        assert!(!flag.inc());
        assert_eq!(flag.flags, 0b100000000);
        assert_eq!(flag.count, 1);
        flag.reset();
        assert_eq!(flag.flags, 0);
        assert_eq!(flag.count, 0);

    }
}