use crate::sq_element::*;
use crate::sq_element::flag::Flag;
use std::fmt::Debug;

// Organize all required traits into one for a value
pub trait Value: Default + Debug + Copy + Clone + IntLimits + Add<Output = Self> + Into<usize> + PartialEq + PartialOrd + TryFrom<usize>{}
impl Value for u8 {}
impl Value for u16 {}

// Incrememnt and reset the value
impl <V: Value> IncReset for ValueType<V> {
    fn inc(&mut self) -> bool {
        if self.value == V::VMAX {
            false
        } else {
            let old = self.value;
            self.value = old + V::VONE;
            true
        }
    }
    fn reset (&mut self) {
        self.value = V::ZERO;

    }
}

// Conver to usize.  Useful for output.
impl <V: Value> From <ValueType<V>> for usize {
    fn from (other: ValueType<V>) -> Self {
        other.value.into()
    }
}

impl <V: Value> From <usize> for ValueType<V> {
    fn from (other: usize) -> Self {
        ValueType{
            value: V::try_from(other).unwrap_or_default()
        }
    }
}

// Convert from flag to value.
impl <V: Value, F: Flag> From <FlagType<F>> for ValueType<V> {
    fn from(other: FlagType<F>) -> Self {
        if other.flags == F::ZERO || other.count != 1 {
            ValueType {
                value: V::ZERO
            }
        } else {

            let mut tally = V::ZERO;
            let mut val = other.flags;
            while val != F::ZERO {
                val = val >> F::ONE;
                tally = tally + V::VONE;
                assert!(tally <= V::VMAX, "Assumption in From<Flag<F>> to ValueType<V> incorrect");
            }
            ValueType {
                value: V::from (tally)
            }
        }
    }
}

#[cfg(test)]
mod value_tests {
    use crate::sq_element::ValueType;
    use super::*;

    #[test]
    fn inc_reset () {
        let mut v: ValueType<u8> = ValueType::from(8usize);
        assert_eq!(v.value, 8);
        assert!(v.inc());
        assert_eq!(v.value, 9);
        assert!(!v.inc());
        assert_eq!(v.value, 9);
        v.reset();
        assert_eq!(v.value, 0);


    }

}