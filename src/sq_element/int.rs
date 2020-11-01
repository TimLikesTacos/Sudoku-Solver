use crate::sq_element::flag::{FlagElement, Flag};
use crate::sq_element::*;
use std::convert::{TryInto, TryFrom};
use std::fmt::{Debug, Display, Formatter};
use crate::sq_element::sq_element::{SqElement, OneZero};
use crate::sq_element::flag_limits::{IntLimits, ZeroAndOne};
use std::ops::Add;
use std::fmt;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct IntValue {
    pub(crate) value: u8,
}

impl Display for IntValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl OneZero for IntValue {
    type Value = u8;
    fn zero() -> Self {
        Self { value: u8::ZERO }
    }
    fn one() -> Self {
        Self { value: u8::ONE }
    }
}

// Incrememnt and reset the value
impl SqElement for IntValue {
    type Item = u8;

    fn inc(&mut self) -> bool {
        if self.value == u8::VMAX {
            false
        } else {
            let old = self.value;
            self.value = old + u8::ONE;
            true
        }
    }
    fn reset(&mut self) {
        self.value = u8::ZERO
    }

    fn get(&self) -> Self::Item {
        self.value
    }

    fn set<F: SqElement>(&mut self, value: F)
        where Self: From<F>
    {
        self.value = Self::from(value).value;
    }

}

// Conver to usize.  Useful for output.
impl From<IntValue> for usize {
    fn from(other: IntValue) -> Self {
        other.value.into()
    }
}

impl From<IntValue> for u8 {
    fn from(other: IntValue) -> Self {
        other.value
    }
}

impl From<usize> for IntValue {
    fn from(other: usize) -> Self {
        IntValue {
            value: u8::try_from(other).unwrap_or_default(),
        }
    }
}

impl From<i32> for IntValue {
    fn from(other: i32) -> Self {
        IntValue {
            value: u8::try_from(other).unwrap_or_default(),
        }
    }
}

impl From<u8> for IntValue {
    fn from(other: u8) -> Self {
        IntValue {
            value: other,
        }
    }
}

impl From<&u8> for IntValue {
    fn from(other: &u8) -> Self {
        IntValue {
            value: *other,
        }
    }
}

// Convert from flag to value.
impl<F: FlagElement> From<Flag<F>> for IntValue {
    fn from(other: Flag<F>) -> Self {
        if other.flag == F::ZERO {
            IntValue { value: u8::ZERO }
        } else {
            let mut tally = u8::ZERO;
            let mut val = other.flag;
            let mut multi_ones = false;
            while val != F::ZERO {
                // Can't convert a flag with multiple flags into a number
                if multi_ones && val & F::ONE == F::ONE {
                    return IntValue { value: u8::ZERO };
                }
                // Once true, always stays true.
                multi_ones = if val & F::ONE == F::ONE {
                    true
                } else {
                    multi_ones
                };
                val = val >> F::ONE;
                tally = tally + u8::ONE;

                assert!(
                    tally <= u8::VMAX,
                    format!(
                        "Assumption in From<Flag<F>> to ValueType<V> incorrect: {}",
                        tally
                    )
                );
            }
            IntValue { value: tally }
        }
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn inc_reset() {
        let mut v: IntValue = IntValue::from(8usize);
        assert_eq!(v.value, 8);
        assert!(v.inc());
        assert_eq!(v.value, 9);
        assert!(!v.inc());
        assert_eq!(v.value, 9);
        v.reset();
        assert_eq!(v.value, 0);
    }
}
