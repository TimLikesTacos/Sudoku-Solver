use crate::sq_element::flag::*;
use crate::sq_element::sq_element::*;
use crate::square::*;
use crate::sq_element::int::IntValue;

pub trait FlagUpdate {
    type FlagElement;
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>);
    fn remove_flag (&mut self, other: Self::FlagElement);
    // fn add_flag(&mut self, e: impl SqElement);
}

// impl<S: SqElement> FlagUpdate for SimpleSquare<S> {
//     fn set_initial<'a>(&'a mut self, _: impl Iterator<Item = &'a Self>) {}
//     //fn remove_flag<F: FlagElement>(&mut self, other: Flag<F>) {}
//     //fn add_flag(&mut self, e: IntType<S::Element>) {}
// }

impl<F1: FlagElement, F2: FlagElement> FlagUpdate for FlagSquare<Flag<F1>, Flag<F2>>
where
    Flag<F1>: From<Flag<F2>>,
    Flag<F2>: FlElement<FlagItem = F2, Item = F2> + From<Flag<F1>>,
{
    type FlagElement = Flag<F2>;
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>) {
        self.flags = it.fold(Flag::max(), |acc, x| {
            acc - <Flag<F2>>::from(x.value)
        });
        self.count = Flag::count_ones(&self.flags);
    }

    fn remove_flag(&mut self, other: Flag<F2>)
    {
        if self.flags.is_flagged(&other) {
            let similar: Flag<F2> = self.flags & other;
            let num = Flag::count_ones(&similar);
            self.flags -= other;
            self.count -= num;
        }
    }

    // fn add_flag(&mut self, e: impl SqElement) {
    //     unimplemented!()
    // }
}

impl<F2: FlagElement> FlagUpdate for FlagSquare<IntValue, Flag<F2>>
where
    Flag<F2>: FlElement<FlagItem = F2, Item = F2> + From<IntValue>,
{
    type FlagElement = Flag<F2>;
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>) {
        self.flags = it.fold(Flag::max(), |acc, x| {
            acc - <Flag<F2>>::from(x.value)
        });
        self.count = Flag::count_ones(&self.flags);
    }

    fn remove_flag(&mut self, other: Flag<F2>) {

        if self.flags.is_flagged(&other) {
            let num = Flag::count_ones(&(&self.flags & &other));
            self.flags -= other;
            self.count -= num;
        }
    }

    // fn add_flag(&mut self, e: impl SqElement) {
    //     unimplemented!()
    // }
}

#[cfg(test)]
mod flag_update_tests {
    use super::*;

    #[test]
    fn ft_ft_test() {
        let v: Vec<u8> = vec![1, 3, 5];
        let vals: Vec<FlagSquare<Flag<u16>, Flag<u16>>> =
            v.iter().map(|&e| FlagSquare::new(e, true)).collect();
        let mut fs: FlagSquare<Flag<u16>, Flag<u16>> = FlagSquare::new(0, false);
        assert_eq!(fs.getv(), 0);
        assert_eq!(fs.flags.get(), 0);
        fs.set_initial(vals.iter());
        assert_eq!(fs.getv(), 0);
        assert_eq!(fs.flags.get(), 0b111101010);
        assert!(fs.inc());
        assert_eq!(fs.getv(), 0b10);
        assert_eq!(fs.flags.get(), 0b111101000);
        assert!(fs.inc());
        assert_eq!(fs.getv(), 0b1000);
        assert_eq!(fs.flags.get(), 0b111100010);
        assert!(fs.inc());
        assert!(fs.inc());
        assert!(fs.inc());
        assert!(fs.inc());
        assert_eq!(fs.getv(), 0b100000000);
        assert_eq!(fs.flags.get(), 0b011101010);
        assert!(!fs.inc());
    }

    #[test]
    fn int_ft_test() {
        let v: Vec<u8> = vec![1, 3, 5];
        let vals: Vec<FlagSquare<IntValue, Flag<u16>>> =
            v.iter().map(|&e| FlagSquare::new(e, true)).collect();
        let mut fs: FlagSquare<IntValue, Flag<u16>> = FlagSquare::new(0, false);
        assert_eq!(fs.getv(), 0);
        assert_eq!(fs.flags.get(), 0);
        fs.set_initial(vals.iter());
        assert_eq!(fs.getv(), 0);
        assert_eq!(fs.flags.get(), 0b111101010);
        assert!(fs.inc());
        assert_eq!(fs.getv(), 2);
        assert_eq!(fs.flags.get(), 0b111101000);
        assert!(fs.inc());
        assert_eq!(fs.getv(), 4);
        assert_eq!(fs.flags.get(), 0b111100010);
        assert!(fs.inc());
        assert!(fs.inc());
        assert!(fs.inc());
        assert!(fs.inc());
        assert_eq!(fs.getv(), 9);
        assert_eq!(fs.flags.get(), 0b011101010);
        assert!(!fs.inc());
    }
}
