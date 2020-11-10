use crate::grid::MAX_NUM;
use crate::sq_element::sq_element::FlElement;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

/// Struct used to count occurances of a value.  A value is stored in 'flag', the the
/// indicies are stored in 'indicies', as a FlagElement type.  NOTE:: when inserting indicies,
/// they must be 'starting at 1' based, since 0 would be indiscernible from empty.
#[derive(Clone, Debug)]
pub(crate) struct Ctr<FT: FlElement> {
    pub(crate) flag: FT,
    pub(crate) indicies: FT,
    pub(crate) ind_count: u8,
}
impl<FT: FlElement> Ctr<FT> {
    fn new(val: FT) -> Ctr<FT> {
        Ctr {
            flag: val,
            indicies: FT::zero(),
            ind_count: 0,
        }
    }
    pub fn merge(&self, others: &[Ctr<FT>]) -> Ctr<FT> {
        let (flag, c_ind) = others.iter().fold((self.flag, self.indicies), |(a, b), x| {
            (a | x.flag, b | x.indicies)
        });
        Ctr {
            flag: flag,
            indicies: c_ind,
            ind_count: FT::count_ones(&c_ind),
        }
    }
}

impl<FT: FlElement> Sub for Ctr<FT> {
    type Output = Ctr<FT>;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.indicies - rhs.indicies;
        Ctr {
            flag: self.flag,
            indicies: diff,
            ind_count: FT::count_ones(&diff),
        }
    }
}
/** NOTE the OR operator on flag.  This is because this operation is combining two Ctrs
* So the flags or ORed to show two values, but the flags are AND to see if BOTH values are in the
* square*/
impl<FT: FlElement> BitAnd for Ctr<FT> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let new_ind = self.indicies & rhs.indicies;
        Ctr {
            flag: self.flag | rhs.flag,
            indicies: new_ind,
            ind_count: FT::count_ones(&new_ind),
        }
    }
}

impl<FT: FlElement> BitXor for Ctr<FT> {
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

impl<FT: FlElement> BitOr for Ctr<FT> {
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

impl<FT: FlElement> PartialEq for Ctr<FT> {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.indicies == other.indicies
    }
}

/// Struct used to count values and occurances.  Contains a vector of size MAX_NUM (9 for
/// a typical 9x9 sudoku puzzle. The vector stores Ctr types.  NOTE:: Ctr stores indicies, these
/// must be begin at 1 based, since a index of 0 is indiscernible from empty.
#[derive(Clone, Debug)]
pub(crate) struct TupleCtr<FT: FlElement> {
    pub(crate) array: Vec<Ctr<FT>>,
}
impl<FT: FlElement> TupleCtr<FT> {
    /// Sets up an new tuple couter with an array of MAX_NUM length.  Each element
    /// has a single flag value. 0b0001, 0b0010, 0b0100, ect.
    pub fn new() -> TupleCtr<FT> {
        let mut res = TupleCtr { array: Vec::new() };
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
        assert_ne!(
            index,
            FT::zero(),
            "Inserting indicies should start at 1, not 0.  0 is \
        indiscernible from empty"
        );
        let zero = FT::zero();
        for i in 0..self.array.len() {
            if self.array[i].flag & flag > zero {
                self.array[i].indicies = self.array[i].indicies | index;
                self.array[i].ind_count += 1;
            }
        }
    }

    /// O(n!/r!(n-r)!) where n is the MAX_NUM grid size (9 for 9x9) and r is the tuple size.
    /// NOTE that this is a very expensive calc for large tuples.
    /// In a 9x9 puzzle, the largest tuple you should look for is where n==4, therefore
    /// it is not that large, plus skipping values with zero (already solved squares) will
    /// increase efficiency.
    pub fn combo(&self, tuple_size: u8) -> Vec<Ctr<FT>> {
        let mut results: Vec<Ctr<FT>> = Vec::new();
        self.combo_rec(
            &mut results,
            Ctr::new(FT::zero()),
            Ctr::new(FT::zero()),
            0,
            0,
            tuple_size,
        );
        results
    }
    // Recursive function to get tuples
    fn combo_rec(
        &self,
        tups: &mut Vec<Ctr<FT>>,
        lhs: Ctr<FT>,
        rhs: Ctr<FT>,
        current_ind: usize,
        current_tuple: u8,
        tuple_size: u8,
    ) {
        // Potential tuple of appropiate size calcualted.  Determine if it is a tuple.
        if current_tuple == tuple_size {
            let mut r_rhs = rhs;
            // add the remaining in the array to the rhs.
            for i in current_ind..self.array.len() {
                r_rhs = r_rhs | self.array[i].clone();
            }
            //dbg!(&lhs, &r_rhs);
            let diff = lhs - r_rhs;
            // dbg!(&diff);
            // diff will be equal to lhs_r if all the elements in lhs are not in rhs.
            //dbg!(&diff, &lhs.indicies, &r_rhs.indicies);
            if diff.ind_count == tuple_size {
                tups.push(diff);
            }
            return;
        }
        if current_ind >= self.array.len() {
            return;
        }
        // Make lhs the bitwise AND of what was passed in and the value at the cursor.
        let added_to_left = self.array[current_ind].clone() | lhs.clone();
        let added_to_right = self.array[current_ind].clone() | rhs.clone();

        //Increment current_ind
        let mut ind = current_ind;

        loop {
            ind += 1;
            if ind >= self.array.len() {
                // one more call to determine last tuple, but return after that.
                break;
            }
            // no reason to deal with empty fields
            if self.array[ind].ind_count != 0 {
                break;
            }
        }

        // continue with building tuple
        // Add to left
        self.combo_rec(
            tups,
            added_to_left,
            rhs.clone(),
            ind,
            current_tuple + 1,
            tuple_size,
        );

        // A tuple has been met, so start with the original values and increment the index;
        self.combo_rec(
            tups,
            lhs.clone(),
            added_to_right,
            ind,
            current_tuple,
            tuple_size,
        );
    }

    pub fn ctr_iter(&self) -> impl Iterator<Item = &Ctr<FT>> {
        self.array.iter()
    }
}

impl<F: FlElement> IntoIterator for TupleCtr<F> {
    type Item = Ctr<F>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

#[cfg(test)]
mod ctr_tests {
    use crate::human_calcs::tuple_ctr::TupleCtr;
    use crate::sq_element::sq_element::{FlElement, OneZero, SqElement};
    use crate::sq_element::Flag;

    #[test]
    fn ctr_test() {
        let mut c: TupleCtr<Flag<u16>> = TupleCtr::new();

        for (i, v) in c.array.iter().enumerate() {
            let val = 1 << i;
            assert_eq!(v.flag.get(), val);
            assert_eq!(v.indicies, Flag::zero())
        }

        c.insert(
            Flag::from(5),
            Flag::from(1).merge(&[Flag::from(3), Flag::from(4)]),
        );
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

    #[test]
    fn combo_test() {
        let one = <Flag<u16>>::from(1);
        let two = <Flag<u16>>::from(2);
        let three = <Flag<u16>>::from(3);
        let four = <Flag<u16>>::from(4);
        let five = <Flag<u16>>::from(5);
        let six = <Flag<u16>>::from(6);
        let seven = <Flag<u16>>::from(7);
        let eight = <Flag<u16>>::from(8);
        let nine = <Flag<u16>>::from(9);

        let array: [Flag<u16>; 9] = [one, two, three, four, five, six, seven, eight, nine];
        let mut the_counter = TupleCtr::new();
        for (i, x) in array.iter().enumerate() {
            the_counter.insert(Flag::from(i + 1), *x);
        }
        //dbg!(&the_counter);
        let res = the_counter.combo(1);

        assert_eq!(res.len(), 9);
        let res = the_counter.combo(2);
        assert_eq!(res.len(), 36);
        let res = the_counter.combo(3);
        assert_eq!(res.len(), 84);
        for (i, x) in array.iter().enumerate() {
            let c = if i + 2 > 9 { 9 } else { i + 2 };
            the_counter.insert(Flag::from(c), *x);
        }
        let res = the_counter.combo(2);
        //dbg!(&the_counter, &res);
        // One-Two tuple on the first two squares.
        assert_eq!(res.len(), 1);
        // reset value 2 to include only index 2.
        the_counter.array[1].indicies = Flag::from(2);
        the_counter.array[1].ind_count = 1;
        // after below, index 1 and 2 have values for 1 and 2 and should be the only tuple.
        the_counter.insert(one, two);
        assert_eq!(the_counter.array[0].indicies, the_counter.array[1].indicies);
        let res = the_counter.combo(2);
        // The value of '3' in square 3 is a 1-uple, so it is also present in the count
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].indicies, Flag::from(1).merge(&[Flag::from(2)]));
        let res = the_counter.combo(3);
        // 1,2 in 1,2. 3,4 in 4 (3 in unique), 4,5 in 4, 5,6 in 5
        assert_eq!(res.len(), 2);

        // make triple.  Should be no tuples, but one triple
        the_counter.insert(one, three);
        the_counter.insert(three, one);

        assert_eq!(the_counter.array[0].indicies, one.merge(&[two, three]));
        assert_eq!(the_counter.array[1].indicies, one.merge(&[two]));
        assert_eq!(the_counter.array[2].indicies, one.merge(&[three, four]));
        assert_eq!(the_counter.array[3].indicies, four.merge(&[five]));
        assert_eq!(the_counter.array[8].indicies, nine);

        let res = the_counter.combo(2);
        assert_eq!(res.len(), 0);
        let res = the_counter.combo(3);
        assert_eq!(res.len(), 1);

        let mut the_counter = TupleCtr::new();
        for (i, x) in array.iter().enumerate() {
            the_counter.insert(Flag::from(i + 1), *x);
            let c = if i + 2 > 9 { 9 } else { i + 2 };
            the_counter.insert(Flag::from(c), *x);
        }
        let res = the_counter.combo(1);
        assert_eq!(res.len(), 1);
        let res = the_counter.combo(2);
        assert_eq!(res.len(), 1);
        the_counter.insert(one, nine);
        let res = the_counter.combo(1);

        assert_eq!(res.len(), 0);
        let res = the_counter.combo(2);
        assert_eq!(res.len(), 0);
        the_counter.insert(nine, one);
        the_counter.array[1].indicies = one.merge(&[two]);
        // lost a 3, add a 3
        the_counter.insert(three, five);
        let res = the_counter.combo(1);
        assert_eq!(res.len(), 0);
        let res = the_counter.combo(2);
        dbg!(&the_counter, &res);
        assert_eq!(res.len(), 0);
        let res = the_counter.combo(3);
        assert_eq!(res.len(), 1);
    }
}
