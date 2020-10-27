
use crate::sq_element::*;
use crate::sq_element::flag_limits::*;
use crate::sq_element::value::NormalInt;
use crate::sq_element::flag::Flag;
use crate::square::flag_update::FlagUpdate;


#[derive(Copy, Clone, Debug, Default)]
pub struct FlagSquare<E: SqElement, F: FlElement> {
    pub(crate) value: E,
    pub(crate) fixed: bool,
    pub(crate) flags: F,
    pub(crate) count :u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SimpleSquare<E: SqElement> {
    pub(crate)value: E,
    pub(crate)fixed: bool,
}

pub trait  Square: PartialEq + Clone
    where Self::Element: SqElement,
       Self::Value:  Sized + PartialEq
{
    type Element;
    type Value;
    fn set(&mut self, v: Self::Value);
    fn getv(&self) -> Self::Value;
    fn exportv(&self) -> u8;
    fn has_flags() -> bool;
    fn fixed(&self) -> bool;
    fn new(v: u8, fix: bool) -> Self;
    fn inc(&mut self) -> bool;
    fn reset_value(&mut self);
    fn zero() -> Self::Value;
    fn one() -> Self::Value;
}

impl <Vt: SqElement + Into<Ft> + From<Ft>, Ft: FlElement + From<Vt>> Square for FlagSquare<Vt, Ft>
    //where Vt::Item: NormalInt
{
    type Element = Vt;
    type Value = Vt::Item;

    fn set (&mut self, v: Self::Value) {
        self.value.set(v)
    }
    fn fixed(&self) -> bool { self.fixed }
    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn exportv(&self) -> u8 {
        self.value.clone().into()
        //u8::from(self.value.clone())
    }
    fn has_flags() -> bool { true }
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
        let mut f:Ft = self.value.clone().into();
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

impl<V: NormalInt> Square  for SimpleSquare<IntType<V>>
where IntType<V>: SqElement<Item = V>
{
    type Element =IntType<V>;
    type Value = V;

    fn getv(&self) -> Self::Value {
        self.value.get()
    }
    fn exportv(&self) -> u8 {
        u8::from(self.value)
    }
    fn fixed(&self) -> bool { self.fixed }

    fn has_flags() -> bool { false }
    fn new(v: u8, fix: bool) -> Self {
        SimpleSquare {
            value: IntType::from(v),
            fixed: fix
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

    fn set(&mut self, v: Self::Value) {
        self.value.set(v)
    }

    fn zero() -> Self::Value {
        IntType::zero().get()
    }

    fn one() -> Self::Value {
        IntType::one().get()
    }
}

impl<F: Flag> Square for SimpleSquare<FlagType<F>>
where FlagType<F>: SqElement<Item = F>
{
    type Element = FlagType<F>;
    type Value = F;

    fn set(&mut self, v: Self::Value) {
        self.value.set(v)
    }

    fn getv(&self) -> Self::Value {
        self.value.get()
    }

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
            value: FlagType::from(v),
            fixed: fix
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
        FlagType::zero().get()
    }

    fn one() -> Self::Value {
        FlagType::one().get()
    }
}

impl <OS: Square, V: SqElement> PartialEq<OS> for SimpleSquare<V>
    where SimpleSquare<V>: Square + From<OS>
{
    /** todo: derive froms using reference so do not have to clone / move **/
    fn eq(&self, other: &OS) -> bool {
        self.value == Self::from(other.clone()).value
    }
}

impl <OS: Square, V: SqElement, F: FlElement> PartialEq<OS> for FlagSquare<V, F>
    where FlagSquare<V, F>: Square + From<OS>
{
    fn eq(&self, other: &OS) -> bool {
        self.value == Self::from(other.clone()).value
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;


    #[test]
    fn new_test() {
        let a: SimpleSquare<IntType<u16>> = SimpleSquare {
            value: <IntType<u16>>::from(4),
            fixed: true
        };
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), true);
        let a: SimpleSquare<IntType<u16>> = SimpleSquare::new(4u8, false);
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), false);
        let a: SimpleSquare<FlagType<u16>> = SimpleSquare::new(4u8, true);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), true);

        let a: FlagSquare<IntType<u16>, FlagType<u16>> = FlagSquare::new(4u8, true);
        assert_eq!(a.getv(), 4u16);
        assert_eq!(a.fixed(), true);
        let a: FlagSquare<FlagType<u32>, FlagType<u32>> = FlagSquare::new(4u8, false);
        assert_eq!(a.getv(), 0b1000);
        assert_eq!(a.fixed(), false);
    }

    #[test]
    fn inc_reset_test() {
        let mut s: FlagSquare<IntType<u8>, FlagType<u16>> = FlagSquare {
            value: IntType { value: 0 },
            fixed: false,
            flags: FlagType { flags: 0b100010010 },
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

        let mut s: FlagSquare<IntType<u8>, FlagType<u16>> = FlagSquare {
            value: IntType { value: 0 },
            fixed: false,
            flags: FlagType { flags: 0b010010010 },
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
        let mut s: FlagSquare<FlagType<u16>, FlagType<u16>> = FlagSquare {
            value: FlagType { flags: 0 },
            fixed: false,
            flags: FlagType { flags: 0b010010010 },
            count: 3,
        };
        assert!(s.inc());
        assert_eq!(s.getv(), 0b10);
        assert!(s.inc());
        assert_eq!(s.getv(),0b10000);
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


