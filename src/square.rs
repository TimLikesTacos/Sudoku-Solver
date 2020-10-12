use crate::flag::{Flag, FlagTrait};


pub struct Square<F: FlagTrait> {
    value: u8,
    fixed: bool,
    pencil: F,
}

pub trait  SquareTrait {
    type Value;
    type FlagType;

    fn setv (&mut self, v: Self::Value);
    fn setp (&mut self, p: Self::FlagType);
    fn getv (&self) -> Self::Value;
    fn getp (&self) -> &Self::FlagType;
    fn getp_mut (&mut self) -> &mut Self::FlagType;
    fn new (v: Self::Value, fix: bool) -> Self;
}

impl <F: FlagTrait + Default> SquareTrait for Square<F> {
    type Value = u8;
    type FlagType = F;

    fn setv (&mut self, v: u8) {
        self.value = v;
    }

    fn setp (&mut self, p: Self::FlagType) {
        self.pencil = p;
    }

    fn getv (&self) -> Self::Value {self.value}
    fn getp (&self) -> &Self::FlagType {&self.pencil}
    fn getp_mut (&mut self) -> &mut Self::FlagType {&mut self.pencil}

    fn new (v: u8, fix: bool) -> Square<F>{
        Square {
            value: v,
            fixed: fix,
            pencil: F::default(),
        }
    }

}

#[cfg(test)]
mod square_tests {
    use super::*;
    use crate::flag::FlagTrait;

    #[test]
    fn new_test () {
        let s: Square<Flag<u16>> = Square::new(5, true);
        let t: Square<Flag<u16>> = Square::new(7, false);
        assert_eq!(s.pencil.count(), 0);
        assert_eq!(s.value, 5);
        assert_eq!(s.fixed, true);
        assert_eq!(t.pencil.count(), 0);
        assert_eq!(t.fixed, false);
        assert_eq!(t.value, 7);
    }

    #[test]
    fn setv_test () {
        let mut s: Square<Flag<u16>> = Square::new(0, false);
        s.setv(4);
        assert_eq!(s.getv(), 4);
    }

    #[test]
    fn setp_test() {
        let mut s: Square<Flag<u16>> = Square::new(0, false);
        s.setp(Flag::from(0b1110));
        assert_eq!(*s.getp(), Flag::from(0b1110));
    }

    #[test]
    fn mutp_test() {
        let mut s: Square<Flag<u16>> = Square::new(0, false);
        s.setp(Flag::from(0b1110));
        let mut p = s.getp_mut();
        *p = p.remove_flag(Flag::from(0b10000010));
        assert_eq!(*s.getp(), Flag::from(0b1100));

    }
}