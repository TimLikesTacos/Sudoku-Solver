use crate::grid::*;
use crate::square::{Square, FlagSquare, SimpleSquare};
use crate::sq_element::{FlagType, SqElement, FlElement};
use crate::sq_element::value::Value;
use crate::sq_element::flag::Flag;
use std::ops::{Index, IndexMut};
use crate::support::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Grid<S: Square> {
    pub(crate) grid: Vec<S>,
}

impl <V1: SqElement, V2: SqElement, F: FlElement> From<Grid<SimpleSquare<V1>>> for Grid<FlagSquare<V2, F>>
    where FlagSquare<V2, F>: From<SimpleSquare<V1>>, SimpleSquare<V1>: Copy{
    fn from(other: Grid<SimpleSquare<V1>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<FlagSquare<V2, F>>::from(x)); acc}),
        }
    }
}

impl <V1: SqElement, V2: SqElement, F: FlElement> From<Grid<FlagSquare<V2, F>>> for Grid<SimpleSquare<V1>>
    where SimpleSquare<V1>: From<FlagSquare<V2, F>>, FlagSquare<V2, F>: Copy{
    fn from(other: Grid<FlagSquare<V2, F>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<SimpleSquare<V1>>::from(x)); acc}),
        }
    }
}


impl<S: SqElement + Clone> Grid<S>

{
    pub fn new<I: Iterator>(input_vec: I) -> Grid<S>
        where S: From<I::Item>, I::Item: Clone
    {
        Grid {
            grid: input_vec.map(|x| S::from(x.clone())).collect()
        }
    }
    /// Iterate over the entire 1-D row dominate grid vector
    pub fn grid_iter (&self) -> impl Iterator <Item= &S>{
        self.grid.iter()
    }

    /// Iterate mutably over the entire 1-D row dominate grid vector
    pub fn grid_iter_mut(&mut self) -> impl Iterator <Item = &mut S> {
        self.grid.iter_mut()
    }

    /// Iterate starting from the beginning of the row that contains the element called.
    pub fn row_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    /// Iterate starting from the beginning of the column that contains the element called.
    pub fn col_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let col = index_to_col(index);
        self.grid.iter().skip(col).step_by(MAX_NUM)
    }

    /// Iterate over the box that contains the element called.
    pub fn box_iter(&self, index: usize) -> BoxIter<S> {
        let box_num = index_to_box(index);

        BoxIter {
            it: self.grid.iter().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    /// Iterate mutably starting from the beginning of the row that contains the element called.
    pub fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    /// Iterate mutably starting from the beginning of the column that contains the element called.
    pub fn col_iter_mut(& mut self, index: usize) -> impl Iterator<Item = & mut S> {
        let col = index_to_col(index);
        self.grid.iter_mut().skip(col).step_by(MAX_NUM)
    }

    /// Iterate mutably over the box that contains the element called.
    pub fn box_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator <Item = &'a mut S> {
        let box_num = index_to_box(index);

        BoxIterMut {
            it: self.grid.iter_mut().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    /// An iterator that iterates over the row, column, and box that the cell with the parameter `index`.
    /// NOTE: The cell of parameter `index` is iterated over 3 times (once for each iterator).
    /// There is no mutable version of this as it would require 3 mutable borrows at the same time.
    /// todo: create a struct with a slice of the cells and create a mutable iterator to get around borrow issues.
    pub fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ S> {
        self.box_iter(index)
            .chain(self.row_iter(index).chain(self.col_iter(index)))
    }

}

impl<S> Index<usize> for Grid<S> {
    type Output = S;
    fn index<'a>(&'a self, i: usize) -> &'a S {
        &self.grid[i]
    }
}

impl<S> IndexMut<usize> for Grid<S> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut S {
        &mut self.grid[i]
    }
}

#[cfg(test)]
mod Grid_Tests {

    use super::*;
    use crate::sq_element::{IntType, FlagType};
    use crate::sq_element::*;
    use crate::sq_element::value::*;
    use crate::sq_element::flag::*;


    #[test]
    fn new_test() {
        let vec: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let grid: Grid<SimpleSquare<IntType<u8>>> = Grid::new(vec.iter());
        let mut iter = grid.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert_eq!(iter.next().unwrap().getv(), 8);
        assert_eq!(iter.next().unwrap().getv(), 7);
        assert_eq!(iter.next().unwrap().getv(), 6);
        assert_eq!(iter.next().unwrap().getv(), 5);
        assert_eq!(iter.next().unwrap().getv(), 4);
        assert_eq!(iter.next().unwrap().getv(), 3);
        assert_eq!(iter.next().unwrap().getv(), 2);
        assert_eq!(iter.next().unwrap().getv(), 1);
        assert_eq!(iter.next(), Option::None);

        let vec: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let grid: Grid<FlagSquare<FlagType<u16>, FlagType<u16>>> = Grid::new(vec.iter());
        let mut iter = grid.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 0b100000000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000000);
        assert_eq!(iter.next().unwrap().getv(), 0b100000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000);
        assert_eq!(iter.next().unwrap().getv(), 0b100);
        assert_eq!(iter.next().unwrap().getv(), 0b10);
        assert_eq!(iter.next().unwrap().getv(), 0b1);
        assert_eq!(iter.next(), Option::None);

    }
}