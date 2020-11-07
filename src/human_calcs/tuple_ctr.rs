use crate::sq_element::sq_element::FlElement;
use crate::grid::MAX_NUM;
use std::ops::{BitAnd, BitXor, BitOr};

#[derive(Clone, Debug)]
pub(crate) struct Ctr <FT: FlElement>{
    pub (crate)flag: FT,
    pub (crate)indicies: FT,
    pub(crate) ind_count: u8
}
impl <FT: FlElement> Ctr<FT> {
    fn new(val: FT) -> Ctr<FT> {
        Ctr {
            flag: val,
            indicies: FT::zero(),
            ind_count: 0
        }
    }
    pub fn merge (&self, others: &[Ctr<FT>]) -> Ctr<FT> {
       let (flag, c_ind) = others.iter().fold((self.flag, self.indicies), |(a, b), x| (a | x.flag, b | x.indicies));
        Ctr {
            flag: flag,
            indicies: c_ind,
            ind_count: FT::count_ones(&c_ind),
        }
    }
}

/** NOTE the OR operator on flag.  This is because this operation is combining two Ctrs
* So the flags or ORed to show two values, but the flags are AND to see if BOTH values are in the
* square*/
impl <FT :FlElement>BitAnd for Ctr<FT>{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let new_ind = self.indicies & rhs.indicies;
        Ctr {
            flag: self.flag | rhs.flag,
            indicies: new_ind,
            ind_count: FT::count_ones(&new_ind)

        }
    }
}

impl <FT :FlElement>BitXor for Ctr<FT>{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let new_ind = self.indicies ^ rhs.indicies;
        Ctr {
            flag: self.flag ^ rhs.flag,
            indicies: new_ind,
            ind_count: FT::count_ones(&new_ind),

        }
    }
}

impl <FT :FlElement>BitOr for Ctr<FT>{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let new_ind = self.indicies | rhs.indicies;
        Ctr {
            flag: self.flag | rhs.flag,
            indicies: new_ind,
            ind_count: FT::count_ones(&new_ind),
        }
    }
}

impl <FT: FlElement>PartialEq for Ctr<FT> {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.indicies == other.indicies
    }
}
#[derive(Clone, Debug)]
pub(crate) struct TupleCtr<FT: FlElement> {
    pub (crate) array: Vec<Ctr<FT>>
}
impl <FT: FlElement>TupleCtr<FT> {
    /// Sets up an new tuple couter with an array of MAX_NUM length.  Each element
    /// has a single flag value. 0b0001, 0b0010, 0b0100, ect.
    pub fn new() -> TupleCtr<FT> {
        let mut res = TupleCtr {
            array: Vec::new()
        };
        res.array.reserve(MAX_NUM);
        let one = FT::one();
        let mut f = one;
        for i in 0..MAX_NUM {
            res.array.push(Ctr::new(f));
            f = f << one;
        }
        res
    }

    pub fn insert(&mut self, index: FT, flag: FT) {
        let zero = FT::zero();
        for i in 0..self.array.len() {
            if self.array[i].flag & flag > zero {
                self.array[i].indicies = self.array[i].indicies | index;
                self.array[i].ind_count += 1;
            }
        }
    }



    pub fn ctr_iter(&self) -> impl Iterator<Item=&Ctr<FT>> {
        self.array.iter()
    }
}

impl <F: FlElement> IntoIterator for TupleCtr<F> {
    type Item = Ctr<F>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

#[cfg(test)]
mod ctr_tests {
    use crate::human_calcs::tuple_ctr::TupleCtr;
    use crate::sq_element::{Flag};
    use crate::sq_element::sq_element::{SqElement, FlElement, OneZero};

    #[test]
    fn ctr_test() {
        let mut c: TupleCtr<Flag<u16>> = TupleCtr::new();

        for (i, v) in c.array.iter().enumerate() {
            let val = 1 << i;
            assert_eq!(v.flag.get(), val);
            assert_eq!(v.indicies, Flag::zero())
        }

        c.insert(Flag::from(5), Flag::from(1).merge(&[Flag::from(3), Flag::from(4)]));
        dbg!(&c);
        for (i, v) in c.into_iter().enumerate() {
            if i == 0 || i == 2 || i == 3 {
                assert_eq!(v.indicies & Flag::from(5), Flag::from(5));
                assert_eq!(Flag::count_ones(&v.indicies), 1);
            } else {
                assert_eq!(v.indicies & Flag::from(5), Flag::from(0));
                assert_eq!(Flag::count_ones(&v.indicies), 0);
            }
        }

    }
}