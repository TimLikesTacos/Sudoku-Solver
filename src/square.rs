use crate::flag::*;
use std::sync::mpsc::TryRecvError;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FlagSquare<V, F:FlagTrait> {
    value: V,
    fixed: bool,
    pencil: F,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SimpleSquare<V> {
    value: V,
}




pub trait SquareTrait
{
    type Value: PartialEq;
    fn setv (&mut self, v: Self::Value);
    fn getv (&self) -> Self::Value;
}

impl <V: Clone + PartialEq, F: FlagTrait> SquareTrait for FlagSquare<V, F> {
    type Value = V;

    fn setv (&mut self, v: Self::Value) {
        self.value = v;
    }
    fn getv (&self) -> Self::Value {self.value.clone()}
}

impl <V: Clone + PartialEq> SquareTrait for SimpleSquare<V> {

    type Value = V;

    fn setv (&mut self, v: Self::Value) {
        self.value = v;
    }
    fn getv (&self) -> Self::Value {self.value.clone()}
}


pub trait  SquareFlagTrait: SquareTrait {

    type FlagType;

    fn fixed (&self) -> bool;
    fn setp (&mut self, p: Self::FlagType);
    fn getp (&self) -> &Self::FlagType;
    fn getp_mut (&mut self) -> &mut Self::FlagType;
    fn new (v: Self::Value, fix: bool) -> Self;
    fn initial_setp (&mut self, slice: &[Self::FlagType]);
}


impl <V: Clone + PartialEq, F> SquareFlagTrait for FlagSquare <V, F>
where F: FlagTrait + Default
{

    type FlagType = F;

    fn fixed (&self) -> bool {self.fixed}
    fn getp (&self) -> &Self::FlagType {&self.pencil}
    fn getp_mut (&mut self) -> &mut Self::FlagType {&mut self.pencil}

    fn setp (&mut self, p: Self::FlagType) {
        self.pencil = p;
    }

    fn new (v: V, fix: bool) -> FlagSquare<V, F>{
        FlagSquare {
            value: v,
            fixed: fix,
            pencil: F::default(),
        }
    }

    fn initial_setp (&mut self, slice: &[F]) {
        let values = F::merge(slice);
        self.pencil = F::set_initial(values);
    }

}

///* `From` is implemented for FlagSquare to SimpleSquare, but not the other way as data would
/// be lost for the flag values.  Keep this in mind when using other functions that have `from()`
/// for different square functions *///
impl <V: Clone, F: FlagTrait> From<&FlagSquare<V, F>> for SimpleSquare<V> {
    fn from (other: &FlagSquare<V, F>) -> SimpleSquare<V> {
        SimpleSquare {
            value: other.value.clone(),
        }
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;
    use crate::flag::FlagTrait;

    #[test]
    fn new_test () {
        let s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(5).get_flags(), true);
        let t: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(7).get_flags(), false);
        assert_eq!(s.pencil.count(), 0);
        assert_eq!(usize::from(s.value), 2usize.pow(5 - 1));
        assert_eq!(s.fixed, true);
        assert_eq!(t.pencil.count(), 0);
        assert_eq!(t.fixed, false);
        assert_eq!(usize::from(t.value), 2usize.pow(7 - 1));
    }

    #[test]
    fn setv_test () {
        let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0).get_flags(), false);
        s.setv(Flag::from(4).get_flags());
        assert_eq!(usize::from(s.getv()), 2usize.pow(4 - 1));
    }

    #[test]
    fn setp_test() {
        let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0).get_flags(), false);
        s.setp(Flag::from(0b1110));
        assert_eq!(*s.getp(), Flag::from(0b1110));
    }

    #[test]
    fn mutp_test() {
        let mut s: FlagSquare<u16, Flag<u16>> = FlagSquare::new(Flag::from(0).get_flags(), false);
        s.setp(Flag::new(0b1110));
        let mut p = s.getp_mut();
        *p = p.remove_flag(Flag::new(0b10000010));
        assert_eq!(*s.getp(), Flag::new(0b1100));

    }
}