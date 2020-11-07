use crate::sq_element::flag::*;
use crate::sq_element::int::IntValue;
use crate::sq_element::sq_element::FlElement;
use crate::square::square::*;

impl From<u8> for SimpleSquare<IntValue> {
    fn from(v: u8) -> Self {
        SimpleSquare {
            value: IntValue::from(v),
            fixed: if v > 0 { true } else { false },
        }
    }
}

impl From<&u8> for SimpleSquare<IntValue> {
    fn from(v: &u8) -> Self {
        SimpleSquare {
            value: IntValue::from(v),
            fixed: if *v > 0 { true } else { false },
        }
    }
}

impl<Ft: FlElement + From<u8>> From<u8> for FlagSquare<IntValue, Ft> {
    fn from(v: u8) -> Self {
        FlagSquare {
            value: IntValue::from(v),
            fixed: if v > 0 { true } else { false },
            flags: Ft::from(0u8),
            count: 0,
        }
    }
}

impl<Ft: FlElement> From<&u8> for FlagSquare<IntValue, Ft> {
    fn from(v: &u8) -> Self {
        FlagSquare {
            value: IntValue::from(v),
            fixed: if *v > 0 { true } else { false },
            flags: Ft::zero(),
            count: 0,
        }
    }
}

impl<V: FlagElement, Ft: FlElement> From<&u8> for FlagSquare<Flag<V>, Ft> {
    fn from(v: &u8) -> Self {
        FlagSquare {
            value: Flag::from(*v),
            fixed: if *v > 0 { true } else { false },
            flags: Ft::zero(),
            count: 0,
        }
    }
}
impl<F: FlagElement> From<u8> for SimpleSquare<Flag<F>> {
    fn from(v: u8) -> Self {
        SimpleSquare {
            value: Flag::from(v),
            fixed: if v > 0 { true } else { false },
        }
    }
}

impl<F: FlagElement, Ft: FlElement + From<u8>> From<u8> for FlagSquare<Flag<F>, Ft> {
    fn from(v: u8) -> Self {
        FlagSquare {
            value: Flag::from(v),
            fixed: if v > 0 { true } else { false },
            flags: Ft::from(0u8),
            count: 0,
        }
    }
}

/*
*todo: Converts value of one type to usize, then to value type of another.  This is due to not being able
*to implement from<intType<V1>> for IntType<V2> since the basic form of From<T> to <T> is ambiguous.
* Can try creating new trait that converts values to values.
 */
impl<Ft: FlElement> From<SimpleSquare<IntValue>> for FlagSquare<IntValue, Ft> {
    fn from(other: SimpleSquare<IntValue>) -> Self {
        FlagSquare {
            value: <IntValue>::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl<Ft: FlElement, V2: FlagElement> From<SimpleSquare<IntValue>> for FlagSquare<Flag<V2>, Ft> {
    fn from(other: SimpleSquare<IntValue>) -> Self {
        FlagSquare {
            value: Flag::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl<Ft: FlElement, F: FlagElement, F2: FlagElement> From<SimpleSquare<Flag<F2>>>
    for FlagSquare<Flag<F>, Ft>
{
    fn from(other: SimpleSquare<Flag<F2>>) -> Self {
        FlagSquare {
            value: <Flag<F>>::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl<Ft: FlElement, F: FlagElement> From<SimpleSquare<Flag<F>>> for FlagSquare<IntValue, Ft> {
    fn from(other: SimpleSquare<Flag<F>>) -> Self {
        FlagSquare {
            value: IntValue::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl<Ft: FlElement> From<FlagSquare<IntValue, Ft>> for SimpleSquare<IntValue> {
    fn from(other: FlagSquare<IntValue, Ft>) -> Self {
        SimpleSquare {
            value: IntValue::from(usize::from(other.value)),
            fixed: other.fixed,
        }
    }
}

impl<Ft: FlElement, F: FlagElement, F2: FlagElement> From<FlagSquare<Flag<F>, Ft>>
    for SimpleSquare<Flag<F2>>
{
    fn from(other: FlagSquare<Flag<F>, Ft>) -> Self {
        SimpleSquare {
            value: Flag::from(usize::from(other.value)),
            fixed: other.fixed,
        }
    }
}

impl<Ft: FlElement, F: FlagElement> From<FlagSquare<Flag<F>, Ft>> for SimpleSquare<IntValue> {
    fn from(other: FlagSquare<Flag<F>, Ft>) -> Self {
        SimpleSquare {
            value: IntValue::from(usize::from(other.value)),
            fixed: other.fixed,
        }
    }
}

impl<F: FlagElement> From<SimpleSquare<Flag<F>>> for SimpleSquare<IntValue> {
    fn from(other: SimpleSquare<Flag<F>>) -> Self {
        SimpleSquare {
            value: IntValue::from(usize::from(other.value)),
            fixed: other.fixed,
        }
    }
}

impl<Ft: FlagElement, F2: FlagElement, V2: FlagElement> From<FlagSquare<IntValue, Flag<Ft>>>
    for FlagSquare<Flag<V2>, Flag<F2>>
{
    fn from(other: FlagSquare<IntValue, Flag<Ft>>) -> Self {
        FlagSquare {
            value: Flag::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Flag::from(usize::from(other.flags)),
            count: other.count,
        }
    }
}

impl<Ft: FlagElement, F2: FlagElement, V2: FlagElement> From<FlagSquare<Flag<V2>, Flag<Ft>>>
    for FlagSquare<IntValue, Flag<F2>>
{
    fn from(other: FlagSquare<Flag<V2>, Flag<Ft>>) -> Self {
        FlagSquare {
            value: IntValue::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Flag::from(usize::from(other.flags)),
            count: other.count,
        }
    }
}

#[cfg(test)]
mod square_from_tests {
    use super::*;
    use crate::sq_element::{Flag, IntValue};
    use crate::square::SimpleSquare;

    #[test]
    fn from_tests() {
        let a: SimpleSquare<IntValue> = SimpleSquare {
            value: IntValue::from(4),
            fixed: true,
        };
        let b: FlagSquare<Flag<u16>, Flag<u16>> = FlagSquare::from(a);
        assert_eq!(b.getv(), 0b1000);
        assert_eq!(b.exportv(), 4);
        assert_eq!(b.fixed(), true);

        let c: FlagSquare<IntValue, Flag<u16>> = FlagSquare::from(b);
        assert_eq!(c.getv(), 4);
        assert_eq!(c.exportv(), 4);
        assert_eq!(c.fixed(), true);
    }
}
