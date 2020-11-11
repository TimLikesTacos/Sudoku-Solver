use crate::sq_element::flag_limits::FlagLimits;
use crate::sq_element::int::IntValue;
use crate::sq_element::sq_element::{FlElement, OneZero, SqElement};
use crate::sq_element::*;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Shl, Shr, Sub, SubAssign};

pub trait FlagElement:
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
    + Eq
    + Ord
{
}

impl FlagElement for u16 {}
impl FlagElement for u32 {}

/// A Flag is an integer that is used as binary flags.  000001 is 1, 000010 is 2, 000011 is 3,
/// and so on.  The FlagElement trait is a condensed summary of traits needed for implementation.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd)]
pub struct Flag<F: FlagElement> {
    pub(crate) flag: F,
}

impl<V: FlagElement> Display for Flag<V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.flag)
    }
}

impl<F: FlagElement> OneZero for Flag<F> {
    type Value = F;
    #[inline]
    fn zero() -> Self {
        Self { flag: F::ZERO }
    }
    #[inline]
    fn one() -> Self {
        Self { flag: F::ONE }
    }
}

impl<F: FlagElement> From<Flag<F>> for u8 {
    fn from(item: Flag<F>) -> u8 {
        let mut count: u8 = 0;
        let mut f = item.flag;
        while f != F::ZERO {
            f = f >> F::ONE;
            count += 1;
        }
        count
    }
}

impl<F: FlagElement> From<usize> for Flag<F> {
    fn from(item: usize) -> Flag<F> {
        /*
        todo: clean this up correctly, ensure values within bands
         */
        assert!(item as u64 <= F::FMAX.into());
        if item == 0 {
            Flag { flag: F::ZERO }
        } else {
            Flag {
                flag: F::try_from(1 << (item - 1)).unwrap_or_default(),
            }
        }
    }
}
// Convert back to usize
impl<F: FlagElement> From<Flag<F>> for usize {
    fn from(item: Flag<F>) -> usize {
        let mut count: usize = 0;
        let mut f = item.flag;
        while f != F::ZERO {
            f = f >> F::ONE;
            count += 1;
        }
        count
    }
}

impl<T: FlagElement + From<u8>> From<u8> for Flag<T> {
    fn from(item: u8) -> Flag<T> {
        if item == 0 {
            Flag { flag: T::ZERO }
        } else {
            Flag {
                flag: T::ONE << (T::from(item) - T::ONE),
            }
        }
    }
}

impl From<i32> for Flag<u16> {
    fn from(item: i32) -> Flag<u16> {
        if item == 0 {
            Flag { flag: 0 }
        } else {
            Flag {
                flag: 1 << (item - 1),
            }
        }
    }
}

impl From<Flag<u16>> for u16 {
    fn from(item: Flag<u16>) -> u16 {
        /*
        todo: proper error handling
         */
        u16::try_from(item.flag.trailing_zeros() + 1).unwrap()
    }
}

impl<F: FlagElement> From<IntValue> for Flag<F> {
    fn from(other: IntValue) -> Self {
        let v = usize::from(other);
        Flag::from(v)
    }
}

// Increment and reset flagtypes
impl<F: FlagElement> SqElement for Flag<F> {
    type Item = F;

    #[inline]
    fn inc(&mut self) -> bool {
        if self.flag == F::VMAX {
            false
        } else if self.flag == F::ZERO {
            self.flag = F::ONE;
            true
        } else {
            let old = self.flag;
            self.flag = old << F::ONE;
            true
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.flag = F::ZERO;
    }

    #[inline]
    fn get(&self) -> Self::Item {
        self.flag
    }

    #[inline]
    fn set<V: SqElement>(&mut self, value: V)
    where
        Self: From<V>,
    {
        self.flag = Self::from(value).flag;
    }
}

impl<F: FlagElement> Add for Flag<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let res = self.flag | other.flag;
        Self { flag: res }
    }
}

impl<F: FlagElement> AddAssign for Flag<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.flag = self.flag + rhs.flag
    }
}

impl<F: FlagElement> Sub for Flag<F> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let res = self.flag & (other.flag ^ F::FMAX);
        Self { flag: res }
    }
}

impl<F: FlagElement> SubAssign for Flag<F> {
    fn sub_assign(&mut self, rhs: Self) {
        let res = self.flag & (rhs.flag ^ F::FMAX);
        self.flag = res;
    }
}

impl<F: FlagElement> BitAnd for Flag<F> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Flag<F> {
        Flag {
            flag: self.flag & rhs.flag,
        }
    }
}

impl<F: FlagElement> BitAnd for &Flag<F> {
    type Output = Flag<F>;

    #[inline]
    fn bitand(self, rhs: Self) -> Flag<F> {
        Flag {
            flag: self.flag & rhs.flag,
        }
    }
}

impl<F: FlagElement> BitOr for Flag<F> {
    type Output = Flag<F>;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Flag {
            flag: self.flag | rhs.flag,
        }
    }
}

impl<F: FlagElement> BitXor for Flag<F> {
    type Output = Flag<F>;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Flag {
            flag: self.flag ^ rhs.flag,
        }
    }
}

impl<F: FlagElement> Shl for Flag<F> {
    type Output = Flag<F>;

    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        Flag {
            flag: self.flag << rhs.flag,
        }
    }
}

impl<F: FlagElement> Shr for Flag<F> {
    type Output = Flag<F>;

    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        Flag {
            flag: self.flag >> rhs.flag,
        }
    }
}
impl<F: FlagElement> Ord for Flag<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flag.cmp(&other.flag)
    }
}

// impl <F:FlagElement> Eq for Flag<F> {
//
// }
impl<F: FlagElement> FlElement for Flag<F> {
    type FlagItem = F;
    #[inline]
    fn count_ones(flags: &Self) -> u8 {
        let mut f = flags.flag;
        let mut count = 0;
        while f > F::ZERO {
            f = f & (f - F::ONE);
            count += 1;
        }
        count
    }

    fn merge(&self, slice: &[Self]) -> Self {
        Self {
            flag: slice.iter().fold(self.flag, |acc, x| acc | x.flag),
        }
    }

    fn set_from_value(&mut self, v_slice: &[u8]) {
        self.flag = v_slice
            .iter()
            .fold(F::ZERO, |acc, x| acc | Self::from(*x).flag);
    }

    #[inline]
    fn is_flagged(&self, other: &Self) -> bool {
        if self.flag & other.flag > F::ZERO {
            true
        } else {
            false
        }
    }
    #[inline]
    fn max() -> Self {
        Self { flag: F::FMAX }
    }
}

#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn other_froms() {
        let mut a: Flag<u16> = Flag::from(0usize);
        a.set(<IntValue>::from(3usize));
        assert_eq!(a.flag, 0b100);
        let b: IntValue = IntValue::from(a);
        assert_eq!(b.get(), 3);
        let mut b: IntValue = IntValue::from(0);
        b.set(<Flag<u16>>::from(4usize));
        assert_eq!(b.get(), 4);
    }
    #[test]
    fn froms() {
        let flag: Flag<u16> = Flag::from(4usize);
        assert_eq!(flag.flag, 0b1000);
        assert_eq!(usize::from(flag), 4);
        assert_eq!(<IntValue>::from(flag).value, 4);
        let val: IntValue = IntValue::from(6);
        assert_eq!(val.value, 6);
        let mut flag: Flag<u32> = Flag::from(val);
        assert_eq!(flag.flag, 0b100000);
        assert_eq!(usize::from(flag), 6);
        flag.flag = 0b1100;

        assert_eq!(<IntValue>::from(flag), IntValue::from(0));
    }

    #[test]
    fn inc_and_reset() {
        let mut flag: Flag<u16> = Flag::from(8usize);
        assert_eq!(flag.flag, 0b10000000);
        assert!(flag.inc());
        assert_eq!(flag.flag, 0b100000000);
        assert!(!flag.inc());
        assert_eq!(flag.flag, 0b100000000);

        flag.reset();
        assert_eq!(flag.flag, 0);
    }
}
