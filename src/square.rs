use crate::flag::*;


pub struct Square<F: FlagTrait> {
    value: F,
    fixed: bool,
    pencil: F,
}

pub trait  SquareTrait {
    type Value;
    type FlagType;


    fn fixed (&self) -> bool;
    fn setv (&mut self, v: Self::Value);
    fn setp (&mut self, p: Self::FlagType);
    fn getv (&self) -> Self::Value;
    fn getp (&self) -> &Self::FlagType;
    fn getp_mut (&mut self) -> &mut Self::FlagType;
    fn new (v: Self::Value, fix: bool) -> Self;
    fn initial_setp (&mut self, slice: &[Self::FlagType]);
}

impl <F: FlagTrait + Default + Clone> SquareTrait for Square<F> {

    type Value = F;
    type FlagType = F;

    fn fixed (&self) -> bool {self.fixed}
    fn setv (&mut self, v: Self::Value) {
        self.value = v;
    }

    fn setp (&mut self, p: Self::FlagType) {
        self.pencil = p;
    }

    fn getv (&self) -> Self::Value {self.value.clone()}
    fn getp (&self) -> &Self::FlagType {&self.pencil}
    fn getp_mut (&mut self) -> &mut Self::FlagType {&mut self.pencil}

    fn new (v: Self::Value, fix: bool) -> Square<F>{
        Square {
            value: v,
            fixed: fix,
            pencil: F::default(),
        }
    }

    fn initial_setp (&mut self, slice: &[Self::FlagType]) {
        let values = F::merge(&slice);
        // values contain what are NOT possibilities, therefore need to negate
        self.pencil = F::set_initial(values);
    }

}

#[cfg(test)]
mod square_tests {
    use super::*;
    use crate::flag::FlagTrait;

    #[test]
    fn new_test () {
        let s: Square<Flag<u16>> = Square::new(Flag::from(5), true);
        let t: Square<Flag<u16>> = Square::new(Flag::from(7), false);
        assert_eq!(s.pencil.count(), 0);
        assert_eq!(usize::from(s.value), 5);
        assert_eq!(s.fixed, true);
        assert_eq!(t.pencil.count(), 0);
        assert_eq!(t.fixed, false);
        assert_eq!(usize::from(t.value), 7);
    }

    #[test]
    fn setv_test () {
        let mut s: Square<Flag<u16>> = Square::new(Flag::from(0), false);
        s.setv(Flag::from(4));
        assert_eq!(usize::from(s.getv()), 4);
    }

    #[test]
    fn setp_test() {
        let mut s: Square<Flag<u16>> = Square::new(Flag::from(0), false);
        s.setp(Flag::from(0b1110));
        assert_eq!(*s.getp(), Flag::from(0b1110));
    }

    #[test]
    fn mutp_test() {
        let mut s: Square<Flag<u16>> = Square::new(Flag::from(0), false);
        s.setp(Flag::new(0b1110));
        let mut p = s.getp_mut();
        *p = p.remove_flag(Flag::new(0b10000010));
        assert_eq!(*s.getp(), Flag::new(0b1100));

    }
}