pub(crate) mod flag;
pub(crate) mod flag_limits;
pub(crate) mod int;
pub(crate) mod sq_element;

pub use crate::sq_element::flag::Flag;
pub use crate::sq_element::int::IntValue;
// use crate::sq_element::flag::Flag;
// use crate::sq_element::flag_limits::ZeroAndOne;
// use crate::sq_element::value::NormalInt;
// use flag_limits::{FlagLimits, IntLimits};
// use std::convert::TryFrom;
// use std::ops::{Add, AddAssign, SubAssign};
// use std::fmt::{Result, Display, Formatter};

#[cfg(test)]
mod sq_element_mod {
    #[test]
    fn tests() {}

}


