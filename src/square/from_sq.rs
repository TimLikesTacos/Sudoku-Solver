use crate::square::square::*;
use crate::sq_element::{SqElement, FlElement, IntType, FlagType};
use crate::sq_element::value::Value;
use crate::sq_element::flag::Flag;


impl <V: Value> From<u8> for SimpleSquare<IntType<V>> {
    fn from(v: u8) -> Self {
        SimpleSquare {
            value: IntType::from(v),
            fixed: if v > 0 {true} else {false},
        }
    }
}

impl <V: Value> From<&u8> for SimpleSquare<IntType<V>> {
    fn from(v: &u8) -> Self {
        SimpleSquare {
            value: IntType::from(v),
            fixed: if *v > 0 {true} else {false},
        }
    }
}

impl <V: Value, Ft: FlElement + From<u8>> From<u8> for FlagSquare<IntType<V>, Ft> {
    fn from(v: u8) -> Self {
        FlagSquare {
            value: IntType::from(v),
            fixed: if v > 0 {true} else {false},
            flags: Ft::from(0u8),
            count: 0,
        }
    }
}

impl <F: Flag> From<u8> for SimpleSquare<FlagType<F>> {
    fn from(v: u8) -> Self {
        SimpleSquare {
            value: FlagType::from(v),
            fixed: if v > 0 {true} else {false},
        }
    }
}

impl <F: Flag, Ft: FlElement + From<u8>> From<u8> for FlagSquare<FlagType<F>, Ft> {
    fn from(v: u8) -> Self {
        FlagSquare {
            value: FlagType::from(v),
            fixed: if v > 0 {true} else {false},
            flags: Ft::from(0),
            count: 0,
        }
    }
}


/*
*todo: Converts value of one type to usize, then to value type of another.  This is due to not being able
*to implement from<intType<V1>> for IntType<V2> since the basic form of From<T> to <T> is ambiguous.
* Can try creating new trait that converts values to values.
 */
impl <Ft: FlElement, V1:Value, V2: Value> From<SimpleSquare<IntType<V1>>> for FlagSquare<IntType<V2>, Ft> {
    fn from(other: SimpleSquare<IntType<V1>>) -> Self {
        FlagSquare {
            value: <IntType<V2>>::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl <Ft: FlElement, V1:Value, V2: Flag> From<SimpleSquare<IntType<V1>>> for FlagSquare<FlagType<V2>, Ft> {
    fn from(other: SimpleSquare<IntType<V1>>) -> Self {
        FlagSquare {
            value: FlagType::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl <Ft: FlElement, F: Flag, F2: Flag> From<SimpleSquare<FlagType<F2>>> for FlagSquare<FlagType<F>, Ft> {
    fn from(other: SimpleSquare<FlagType<F2>>) -> Self {
        FlagSquare {
            value: <FlagType<F>>::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl <Ft: FlElement, F: Flag, V: Value> From<SimpleSquare<FlagType<F>>> for FlagSquare<IntType<V>, Ft> {
    fn from(other: SimpleSquare<FlagType<F>>) -> Self {
        FlagSquare {
            value: IntType::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: Ft::default(),
            count: 0,
        }
    }
}

impl <Ft: FlElement, V:Value> From<FlagSquare<IntType<V>, Ft>> for SimpleSquare<IntType<V>> {
    fn from(other: FlagSquare<IntType<V>, Ft>) -> Self {
        SimpleSquare {
            value: IntType::from(usize::from(other.value)),
            fixed: other.fixed,

        }
    }
}

impl <Ft: FlElement, F:Flag, F2:Flag> From<FlagSquare<FlagType<F>, Ft>> for SimpleSquare<FlagType<F2>> {
    fn from(other: FlagSquare<FlagType<F>, Ft>) -> Self {
        SimpleSquare {
            value: FlagType::from(usize::from(other.value)),
            fixed: other.fixed,

        }
    }
}

impl <Ft: Flag, F2:Flag, V: Value, V2: Flag> From<FlagSquare<IntType<V>, FlagType<Ft>>> for FlagSquare<FlagType<V2>, FlagType<F2>> {
    fn from(other: FlagSquare<IntType<V>, FlagType<Ft>>) -> Self {
        FlagSquare {
            value: FlagType::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: FlagType::from(usize::from(other.flags)),
            count: other.count,
        }
    }
}

impl <Ft: Flag, F2:Flag, V: Value, V2: Flag> From<FlagSquare<FlagType<V2>, FlagType<Ft>>> for FlagSquare<IntType<V>, FlagType<F2>> {
    fn from(other: FlagSquare<FlagType<V2>, FlagType<Ft>>) -> Self {
        FlagSquare {
            value: IntType::from(usize::from(other.value)),
            fixed: other.fixed,
            flags: FlagType::from(usize::from(other.flags)),
            count: other.count,
        }
    }
}

#[cfg(test)]
mod square_from_tests {
    use crate::square::SimpleSquare;
    use super::*;
    use crate::sq_element::{IntType, FlagType};

    #[test]
    fn from_tests () {
        let a: SimpleSquare<IntType<u8>> = SimpleSquare {
            value: IntType::from(4),
            fixed: true
        };
        let b: FlagSquare<FlagType<u16>, FlagType<u16>> = FlagSquare::from(a);
        assert_eq!(b.getv(), 0b1000);
        assert_eq!(b.exportv(), 4);
        assert_eq!(b.fixed(), true);

        let c: FlagSquare<IntType<u16>, FlagType<u16>> = FlagSquare::from(b);
        assert_eq!(c.getv(), 4);
        assert_eq!(c.exportv(), 4);
        assert_eq!(c.fixed(), true);



    }
}