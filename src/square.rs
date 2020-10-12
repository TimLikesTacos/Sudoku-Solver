use crate::flag::{Flag, FlagTrait};


pub struct Square {
    value: u8,
    fixed: bool,
    pencil: Flag<u16>
}

pub trait  SquareTrait<T>
    where T: FlagTrait {
    type Value;

    fn setv (&mut self, v: Self::Value);
    fn setp (&mut self, p: T);
    fn getv (&self) -> Self::Value;
    fn getp (&self) -> T;
    fn getp_mut (&mut self) -> &mut T;
    fn new (v: Self::Value, fix: bool) -> Self;
}

impl SquareTrait<Flag<u16>> for Square {
    type Value = u8;

    fn setv (&mut self, v: u8) {
        self.value = v;
    }

    fn setp (&mut self, p: Flag<u16>) {
        self.pencil = p;
    }

    fn getv (&self) -> Self::Value {self.value}
    fn getp (&self) -> Flag<u16> {self.pencil}
    fn getp_mut (&mut self) -> &mut Flag<u16> {&mut self.pencil}

    fn new (v: u8, fix: bool) -> Square{
        Square {
            value: v,
            fixed: fix,
            pencil: Flag::default(),
        }
    }

}

#[cfg(test)]
mod square_tests {
    use super::*;
    use crate::flag::FlagTrait;

    #[test]
    fn new_test () {
        let s = Square::new(5, true);
        let t = Square::new(7, false);
        assert_eq!(s.pencil.count(), 0);
        assert_eq!(s.value, 5);
        assert_eq!(s.fixed, true);
        assert_eq!(t.pencil.count(), 0);
        assert_eq!(t.fixed, false);
        assert_eq!(t.value, 7);
    }

    #[test]
    fn setv_test () {
        let mut s = Square::new(0, false);
        s.setv(4);
        assert_eq!(s.getv(), 4);
    }

    #[test]
    fn setp_test() {
        let mut s = Square::new(0, false);
        s.setp(Flag::from(0b1110));
        assert_eq!(s.getp(), Flag::from(0b11110));
    }

    #[test]
    fn mutp_test() {
        let mut s = Square::new(0, false);
        let mut p = s.getp_mut();
        *p = p.remove_flag(Flag::from(10000010));
        assert_eq!(s.getp(), Flag::from(1100));

    }
}