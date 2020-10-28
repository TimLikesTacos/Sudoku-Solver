use crate::sq_element::flag::Flag;
use crate::sq_element::value::NormalInt;
use crate::sq_element::*;
use crate::square::*;

pub trait FlagUpdate {
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>);
    // fn add_flag(&mut self, e: impl SqElement);
}

impl<S: SqElement> FlagUpdate for SimpleSquare<S> {
    fn set_initial<'a>(&'a mut self, _: impl Iterator<Item = &'a Self>) {}
    //fn add_flag(&mut self, e: IntType<S::Element>) {}
}

impl<F1: Flag, F2: Flag> FlagUpdate for FlagSquare<FlagType<F1>, FlagType<F2>>
where
    FlagType<F1>: From<FlagType<F2>>,
    FlagType<F2>: FlElement<FlagItem = F2, Item = F2> + From<FlagType<F1>>,
{
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>) {
        self.flags = it.fold(FlagType::max(), |acc, x| {
            acc - <FlagType<F2>>::from(x.value)
        });
        self.count = FlagType::count_ones(&self.flags.get());
    }

    // fn add_flag(&mut self, e: impl SqElement) {
    //     unimplemented!()
    // }
}

impl<V: NormalInt, F2: Flag> FlagUpdate for FlagSquare<IntType<V>, FlagType<F2>>
where
    FlagType<F2>: FlElement<FlagItem = F2, Item = F2> + From<IntType<V>>,
{
    fn set_initial<'a>(&'a mut self, it: impl Iterator<Item = &'a Self>) {
        self.flags = it.fold(FlagType::max(), |acc, x| {
            acc - <FlagType<F2>>::from(x.value)
        });
        self.count = FlagType::count_ones(&self.flags.get());
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
        let vals: Vec<FlagSquare<FlagType<u16>, FlagType<u16>>> =
            v.iter().map(|&e| FlagSquare::new(e, true)).collect();
        let mut fs: FlagSquare<FlagType<u16>, FlagType<u16>> = FlagSquare::new(0, false);
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
        let vals: Vec<FlagSquare<IntType<u8>, FlagType<u16>>> =
            v.iter().map(|&e| FlagSquare::new(e, true)).collect();
        let mut fs: FlagSquare<IntType<u8>, FlagType<u16>> = FlagSquare::new(0, false);
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
