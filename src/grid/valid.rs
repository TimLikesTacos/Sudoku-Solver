use crate::grid::*;
use crate::square::{SimpleSquare, FlagSquare, Square};
use crate::sq_element::{SqElement, FlElement};


pub trait ValidEntry {
    /// Determines if the value at the called index is valid based on associated cells in box, row, and col.
    fn valid_entry(&self, index:usize) -> bool;
}


impl <S: SqElement + PartialEq> ValidEntry for Grid<SimpleSquare<S>>
where SimpleSquare<S>: Square + PartialEq, S: PartialEq
{
    fn valid_entry(&self, index: usize) -> bool {
        let v = self.grid[index].getv();
        if self.grid.iter().filter(|x|x.getv() == v).count() > 1 {
            false
        } else {
            true
        }

    }
}

impl<S1: SqElement, S2: FlElement> ValidEntry for Grid<FlagSquare<S1, S2>> {
    fn valid_entry(&self, index:usize) -> bool {
        true
    }
}