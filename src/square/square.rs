use crate::sq_element::flag::*;
use crate::sq_element::int::{IntValue};
use crate::sq_element::sq_element::*;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Copy, Clone, Debug, Default)]
pub struct FlagSquare<E: SqElement, F: FlElement> {
    pub(crate) value: E,
    pub(crate) fixed: bool,
    pub(crate) flags: F,
    pub(crate) count: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SimpleSquare<E: SqElement> {
    pub(crate) value: E,
    pub(crate) fixed: bool,
}

pub trait Square: PartialEq + Clone + Display
where
    Self::Value: Sized + PartialEq,
{
    type Type;
    type Value;
    fn set <V: SqElement>(&mut self, v: V)
        where Self::Type: From<V>;
    fn exportv(&self) -> u8;
    fn has_flags() -> bool;
    fn fixed(&self) -> bool;
    fn new(v: u8, fix: bool) -> Self;
    fn inc(&mut self) -> bool;
    fn reset_value(&mut self);
    fn get_element(&self) -> Self::Type;
    fn getv(&self) -> Self::Value;
    fn zero() -> Self::Value;
    fn one() -> Self::Value;
}

impl<Vt: SqElement + Into<Ft> + From<Ft>, Ft: FlElement + From<Vt>> Square for FlagSquare<Vt, Ft>
//where Vt::Item: NormalInt
{
    type Type = Vt;
    type Value = Vt::Item;

    fn set<V: SqElement>(&mut self, v: V)
        where Self::Type: From<V>
    {
        self.value.set(v)
    }
    fn fixed(&self) -> bool {
        self.fixed
    }
    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn get_element(&self) -> Self::Type {self.value}
    fn exportv(&self) -> u8 {
        self.value.clone().into()
        //u8::from(self.value.clone())
    }
    fn has_flags() -> bool {
        true
    }
    /// Does not set flags
    fn new(v: u8, fix: bool) -> Self {
        FlagSquare {
            value: Vt::from(v),
            fixed: fix,
            flags: Ft::default(),
            count: 0,
        }
    }

    fn inc(&mut self) -> bool {
        if self.fixed {
            return false;
        }
        // convert int to flag
        let mut f: Ft = self.value.clone().into();
        let old_copy = f.clone();
        // increment once
        let mut not_maxed = f.inc();
        // increment until matches possible in flag
        while !self.flags.is_flagged(f) && not_maxed {
            not_maxed = f.inc();
        }

        if not_maxed {
            if old_copy != Ft::zero() {
                //put back old value into possiblities
                self.flags += old_copy;
                self.count += 1;
            }
            //remove from possiblities
            self.flags -= f;
            // Remove current value from possibilities
            self.count -= 1;
            self.value = Vt::from(f);
        }
        not_maxed
    }

    fn reset_value(&mut self) {
        if self.fixed() {
            return;
        }
        // but back into possibilities
        if self.value != Vt::zero() {
            self.count += 1;
            self.flags += Ft::from(self.value);
        }
        // Set to zero
        self.value.reset()
    }

    fn zero() -> Self::Value {
        Vt::zero().get()
    }

    fn one() -> Self::Value {
        Vt::one().get()
    }
}

impl Square for SimpleSquare<IntValue>

{
    type Type = IntValue;
    type Value = u8;

    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn get_element(&self) -> Self::Type {self.value}
    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }
    fn fixed(&self) -> bool {
        self.fixed
    }

    fn has_flags() -> bool {
        false
    }
    fn new(v: u8, fix: bool) -> Self {
        SimpleSquare {
            value: IntValue::from(v),
            fixed: fix,
        }
    }

    fn inc(&mut self) -> bool {
        if self.fixed {
            false
        } else {
            self.value.inc()
        }
    }

    fn reset_value(&mut self) {
        if !self.fixed {
            self.value.reset()
        }
    }

    fn set<V2:SqElement>(&mut self, v: V2)
        where Self::Type: From<V2>

    {
        self.value.set(v)
    }

    fn zero() -> Self::Value {
        IntValue::zero().get()
    }

    fn one() -> Self::Value {
        IntValue::one().get()
    }
}

impl<F: FlagElement> Square for SimpleSquare<Flag<F>>
where
    Flag<F>: SqElement<Item = F>,
{
    type Type = Flag<F>;
    type Value = F;

    fn set<V: SqElement>(&mut self, v: V)
        where Self::Type: From<V>
    {
        self.value.set(v)
    }

    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn get_element(&self) -> Self::Type {self.value}

    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }

    fn has_flags() -> bool {
        false
    }

    fn fixed(&self) -> bool {
        self.fixed
    }

    fn new(v: u8, fix: bool) -> Self {
        SimpleSquare {
            value: Flag::from(v),
            fixed: fix,
        }
    }

    fn inc(&mut self) -> bool {
        if self.fixed {
            false
        } else {
            self.value.inc()
        }
    }

    fn reset_value(&mut self) {
        if !self.fixed {
            self.value.reset()
        }
    }

    fn zero() -> Self::Value {
        Flag::zero().get()
    }

    fn one() -> Self::Value {
        Flag::one().get()
    }
}

impl<OS: Square, V: SqElement> PartialEq<OS> for SimpleSquare<V>
where
    SimpleSquare<V>: Square + From<OS>,
{
    /** todo: derive froms using reference so do not have to clone / move **/
    fn eq(&self, other: &OS) -> bool {
        self.value == Self::from(other.clone()).value
    }
}

impl<OS: Square, V: SqElement, F: FlElement> PartialEq<OS> for FlagSquare<V, F>
where
    FlagSquare<V, F>: Square + From<OS>,
{
    fn eq(&self, other: &OS) -> bool {
        self.value == Self::from(other.clone()).value
    }
}



impl <E: SqElement> Display for SimpleSquare<E>
    where SimpleSquare<E>: Square{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.exportv())
    }
}

impl <E: SqElement, F: FlElement> Display for FlagSquare<E, F>
    where FlagSquare<E, F>: Square{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.exportv())
    }
}

impl <V: SqElement, F: FlElement> FlagSquare<V, F> {
    fn count (&self) -> u8 {self.count}
}
#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    fn new_test() {
        let a: SimpleSquare<IntValue> = SimpleSquare {
            value: <IntValue>::from(4),
            fixed: true,
        };
        assert_eq!(a.getv(), 4);
        assert_eq!(a.fixed(), true);
        let a: SimpleSquare<IntValue> = SimpleSquare::new(4u8, false);
        assert_eq!(a.getv(), 4);
        assert_eq!(a.fixed(), false);
        let a: SimpleSquare<Flag<u16>> = SimpleSquare::new(4u8, true);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), true);

        let a: FlagSquare<IntValue, Flag<u16>> = FlagSquare::new(4u8, true);
        assert_eq!(a.getv(), 4);
        assert_eq!(a.fixed(), true);
        let a: FlagSquare<Flag<u32>, Flag<u32>> = FlagSquare::new(4u8, false);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), false);
    }

    #[test]
    fn set_test() {
        let mut a: SimpleSquare<IntValue> = SimpleSquare::new(4u8, true);
        let b: SimpleSquare<IntValue> = SimpleSquare::new(2u8, false);
        a.set(b.get_element());
        assert_eq!(a.getv(), 2);
        assert_eq!(a.fixed, true);

        let mut c: FlagSquare<Flag<u16>, Flag<u16>> = FlagSquare::new(3u8, true);
        a.set(c.get_element());
        assert_eq!(a.getv(), 3);
        assert_eq!(a.fixed, true);
        c.set(b.get_element());
        assert_eq!(c.getv(), 2);
        assert_eq!(c.fixed, true);

    }

    #[test]
    fn inc_reset_test() {
        let mut s: FlagSquare<IntValue, Flag<u16>> = FlagSquare {
            value: IntValue { value: 0 },
            fixed: false,
            flags: Flag { flag: 0b100010010 },
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

        let mut s: FlagSquare<IntValue, Flag<u16>> = FlagSquare {
            value: IntValue { value: 0 },
            fixed: false,
            flags: Flag { flag: 0b010010010 },
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
        let mut s: FlagSquare<Flag<u16>, Flag<u16>> = FlagSquare {
            value: Flag { flag: 0 },
            fixed: false,
            flags: Flag { flag: 0b010010010 },
            count: 3,
        };
        assert!(s.inc());
        assert_eq!(s.getv(), 0b10);
        assert!(s.inc());
        assert_eq!(s.getv(), 0b10000);
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
