use crate::sq_element::*;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, BitAnd, BitOr, BitXor, Shl, Shr, Sub, SubAssign, Add};
use crate::sq_element::sq_element::{IntType, SqElement};
use crate::sq_element::flag_limits::FlagLimits;
use crate::sq_element::int::NormalInt;

pub trait Flag:
    Default
    + Copy
    + Debug
    + Display
    + Clone
    + FlagLimits
    + Add<Output = Self>
    + Sub<Output = Self>
    + PartialEq
    + PartialOrd
    + Into<u64>
    + From<u8>
    + TryFrom<usize>
    + BitAnd<Output = Self>
    + Shr<Output = Self>
    + Shl<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
{
}

impl Flag for u16 {}
impl Flag for u32 {}


#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FlagType<F: Flag> {
    pub(crate) flags: F,
}

impl<F: Flag> From<FlagType<F>> for u8 {
    fn from(item: FlagType<F>) -> u8 {
        let mut count: u8 = 0;
        let mut f = item.flags;
        while f != F::ZERO {
            f = f >> F::ONE;
            count += 1;
        }
        count
    }
}

impl<F: Flag> From<usize> for FlagType<F> {
    fn from(item: usize) -> FlagType<F> {
        /*
        todo: clean this up correctly, ensure values within bands
         */
        assert!(item as u64 <= F::FMAX.into());
        if item == 0 {
            FlagType { flags: F::ZERO }
        } else {
            FlagType {
                flags: F::try_from(1 << (item - 1)).unwrap_or_default(),
            }
        }
    }
}
// Convert back to usize
impl<F: Flag> From<FlagType<F>> for usize {
    fn from(item: FlagType<F>) -> usize {
        let mut count: usize = 0;
        let mut f = item.flags;
        while f != F::ZERO {
            f = f >> F::ONE;
            count += 1;
        }
        count
    }
}

impl<T: Flag + From<u8>> From<u8> for FlagType<T> {
    fn from(item: u8) -> FlagType<T> {
        if item == 0 {
            FlagType { flags: T::ZERO }
        } else {
            FlagType {
                flags: T::ONE << (T::from(item) - T::ONE),
            }
        }
    }
}

impl From<i32> for FlagType<u16> {
    fn from(item: i32) -> FlagType<u16> {
        if item == 0 {
            FlagType { flags: 0 }
        } else {
            FlagType {
                flags: 1 << (item - 1),
            }
        }
    }
}

impl From<FlagType<u16>> for u16 {
    fn from(item: FlagType<u16>) -> u16 {
        /*
        todo: proper error handling
         */
        u16::try_from(item.flags.trailing_zeros() + 1).unwrap()
    }
}

impl<V: NormalInt, F: Flag> From<IntType<V>> for FlagType<F> {
    fn from(other: IntType<V>) -> Self {
        let v = usize::from(other);
        FlagType::from(v)
    }
}

// Increment and reset flagtypes
impl<F: Flag> SqElement for FlagType<F> {
    type Item = F;

    fn inc(&mut self) -> bool {
        if self.flags == F::VMAX {
            false
        } else if self.flags == F::ZERO {
            self.flags = F::ONE;
            true
        } else {
            let old = self.flags;
            self.flags = old << F::ONE;
            true
        }
    }

    fn reset(&mut self) {
        self.flags = F::ZERO;
    }

    fn get(&self) -> Self::Item {
        self.flags
    }

    fn set<V: SqElement>(&mut self, value: V)
        where Self: From<V>
    {
       self.flags = Self::from(value).flags;
    }

}

impl<F: Flag> Add for FlagType<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let res = self.flags | other.flags;
        Self { flags: res }
    }
}

impl<F: Flag> AddAssign for FlagType<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.flags = self.flags + rhs.flags
    }
}

impl<F: Flag> Sub for FlagType<F> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let res = self.flags & (other.flags ^ F::FMAX);
        Self { flags: res }
    }
}

impl<F: Flag> SubAssign for FlagType<F> {
    fn sub_assign(&mut self, rhs: Self) {
        let res = self.flags & (rhs.flags ^ F::FMAX);
        self.flags = res;
    }
}

impl<F: Flag> BitAnd for FlagType<F> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            flags: self.flags & rhs.flags,
        }
    }
}

#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn other_froms () {
        let mut a: FlagType<u16> = FlagType::from(0usize);
        a.set(<IntType<u8>>::from(3usize));
        assert_eq!(a.flags, 0b100);
        let b: IntType<u16> = IntType::from(a);
        assert_eq!(b.get(), 3);
        let mut b: IntType<u16> = IntType::from(0);
        b.set(<FlagType<u16>>::from(4usize));
        assert_eq!(b.get(), 4);

    }
    #[test]
    fn froms() {
        let flag: FlagType<u16> = FlagType::from(4usize);
        assert_eq!(flag.flags, 0b1000);
        assert_eq!(usize::from(flag), 4);
        assert_eq!(<IntType<u8>>::from(flag).value, 4);
        let val: IntType<u8> = IntType::from(6);
        assert_eq!(val.value, 6);
        let mut flag: FlagType<u32> = FlagType::from(val);
        assert_eq!(flag.flags, 0b100000);
        assert_eq!(usize::from(flag), 6);
        flag.flags = 0b1100;

        assert_eq!(<IntType<u8>>::from(flag), IntType::from(0));
    }

    #[test]
    fn inc_and_reset() {
        let mut flag: FlagType<u16> = FlagType::from(8usize);
        assert_eq!(flag.flags, 0b10000000);
        assert!(flag.inc());
        assert_eq!(flag.flags, 0b100000000);
        assert!(!flag.inc());
        assert_eq!(flag.flags, 0b100000000);

        flag.reset();
        assert_eq!(flag.flags, 0);
    }
}
