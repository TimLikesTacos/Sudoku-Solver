use crate::sq_element::flag::Flag;
use crate::sq_element::*;
use std::convert::TryInto;
use std::fmt::{Debug, Display};

// Organize all required traits into one for a value
pub trait NormalInt:
    Default
    + Display
    + Debug
    + From<u8>
    + Copy
    + Clone
    + IntLimits
    + Add<Output = Self>
    + Into<usize>
    + TryInto<u8>
    + PartialEq
    + PartialOrd
    + TryFrom<i32>
    + TryFrom<usize>
{
}
impl NormalInt for u8 {}
impl NormalInt for u16 {}

// Incrememnt and reset the value
impl<V: NormalInt> SqElement for IntType<V> {
    type Item = V;

    fn inc(&mut self) -> bool {
        if self.value == V::VMAX {
            false
        } else {
            let old = self.value;
            self.value = old + V::ONE;
            true
        }
    }
    fn reset(&mut self) {
        self.value = V::ZERO
    }

    fn get(&self) -> Self::Item {
        self.value
    }

    fn set(&mut self, value: Self::Item) {
        self.value = value
    }

    // fn zero() -> Self {
    //     Self {
    //         value: V::ZERO
    //     }
    // }
    //
    // fn one() -> Self {
    //     Self {
    //         value: V::ONE
    //     }
    // }
}

// Conver to usize.  Useful for output.
impl<V: NormalInt> From<IntType<V>> for usize {
    fn from(other: IntType<V>) -> Self {
        other.value.into()
    }
}

impl<V: NormalInt> From<IntType<V>> for u8 {
    fn from(other: IntType<V>) -> Self {
        other.value.try_into().unwrap_or_default()
    }
}

impl<V: NormalInt> From<usize> for IntType<V> {
    fn from(other: usize) -> Self {
        IntType {
            value: V::try_from(other).unwrap_or_default(),
        }
    }
}

impl<V: NormalInt> From<i32> for IntType<V> {
    fn from(other: i32) -> Self {
        IntType {
            value: V::try_from(other).unwrap_or_default(),
        }
    }
}

impl<V: NormalInt> From<u8> for IntType<V> {
    fn from(other: u8) -> Self {
        IntType {
            value: V::from(other),
        }
    }
}

impl<V: NormalInt> From<&u8> for IntType<V> {
    fn from(other: &u8) -> Self {
        IntType {
            value: V::from(*other),
        }
    }
}

// Convert from flag to value.
impl<V: NormalInt, F: Flag> From<FlagType<F>> for IntType<V> {
    fn from(other: FlagType<F>) -> Self {
        if other.flags == F::ZERO {
            IntType { value: V::ZERO }
        } else {
            let mut tally = V::ZERO;
            let mut val = other.flags;
            let mut multi_ones = false;
            while val != F::ZERO {
                // Can't convert a flag with multiple flags into a number
                if multi_ones && val & F::ONE == F::ONE {
                    return IntType { value: V::ZERO };
                }
                // Once true, always stays true.
                multi_ones = if val & F::ONE == F::ONE {
                    true
                } else {
                    multi_ones
                };
                val = val >> F::ONE;
                tally = tally + V::ONE;
                if tally == V::from(10) {
                    dbg!(&other.flags);
                }
                assert!(
                    tally <= V::VMAX,
                    format!(
                        "Assumption in From<Flag<F>> to ValueType<V> incorrect: {}",
                        tally
                    )
                );
            }
            IntType { value: tally }
        }
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;
    use crate::sq_element::IntType;

    #[test]
    fn inc_reset() {
        let mut v: IntType<u8> = IntType::from(8usize);
        assert_eq!(v.value, 8);
        assert!(v.inc());
        assert_eq!(v.value, 9);
        assert!(!v.inc());
        assert_eq!(v.value, 9);
        v.reset();
        assert_eq!(v.value, 0);
    }
}
