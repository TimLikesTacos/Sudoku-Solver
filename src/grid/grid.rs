use crate::grid::*;
use crate::square::{Square, FlagSquare, SimpleSquare};
use crate::sq_element::{SqElement, FlElement};
use std::ops::{Index, IndexMut};
use crate::support::*;


#[derive(Clone, Debug)]
pub struct Grid<S: Square> {
    pub(crate) grid: Vec<S>,
}

impl <V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy> From<Grid<SimpleSquare<V1>>> for Grid<FlagSquare<V2, F>>
    where FlagSquare<V2, F>: From<SimpleSquare<V1>>, SimpleSquare<V1>: Square + Copy{
    fn from(other: Grid<SimpleSquare<V1>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<FlagSquare<V2, F>>::from(x)); acc}),
        }
    }
}

impl <V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy> From<Grid<FlagSquare<V2, F>>> for Grid<SimpleSquare<V1>>
    where SimpleSquare<V1>: Square + From<FlagSquare<V2, F>>, FlagSquare<V2, F>: Copy{
    fn from(other: Grid<FlagSquare<V2, F>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<SimpleSquare<V1>>::from(x)); acc}),
        }
    }
}

impl <V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy> From<&Grid<SimpleSquare<V1>>> for Grid<FlagSquare<V2, F>>
    where FlagSquare<V2, F>: From<SimpleSquare<V1>>, SimpleSquare<V1>: Square + Copy{
    fn from(other: &Grid<SimpleSquare<V1>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<FlagSquare<V2, F>>::from(x)); acc}),
        }
    }
}

impl <V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy> From<&Grid<FlagSquare<V2, F>>> for Grid<SimpleSquare<V1>>
    where SimpleSquare<V1>: Square + From<FlagSquare<V2, F>>, FlagSquare<V2, F>: Copy{
    fn from(other: &Grid<FlagSquare<V2, F>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(),|mut acc, &x|
                {acc.push(<SimpleSquare<V1>>::from(x)); acc}),
        }
    }
}


impl<S: Square + Clone> Grid<S>

{
    pub fn new(input_vec: Vec<u8>) -> Grid<S>
        where S: Square
    {
        Grid {
            grid: input_vec.iter().map(|x| {
                if *x == 0 {
                    S::new(*x, false)
                } else {
                    S::new(*x, true)
                }
            }).collect()
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

impl<S1: Square , S2: Square + PartialEq<S1>> PartialEq<Grid<S1>> for Grid<S2> {
    fn eq(&self, other: &Grid<S1>) -> bool {
        self.grid_iter().zip(other.grid_iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

// impl<S1: Square, S2: Square> PartialEq<Grid<S2>> for Grid<S1> {
//     fn eq(&self, other: &Grid<S2>) -> bool {
//         unimplemented!()
//     }
// }

impl<S: Square> Index<usize> for Grid<S> {
    type Output = S;
    fn index<'a>(&'a self, i: usize) -> &'a S {
        &self.grid[i]
    }
}

impl<S: Square> IndexMut<usize> for Grid<S> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut S {
        &mut self.grid[i]
    }
}

#[cfg(test)]
mod Grid_Tests {

    use super::*;
    use crate::sq_element::{IntType, FlagType};
    use crate::sq_element::*;




    #[test]
    fn new_test() {
        let vec: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let grid: Grid<SimpleSquare<IntType<u8>>> = Grid::new(vec);
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
        let grid: Grid<FlagSquare<FlagType<u16>, FlagType<u16>>> = Grid::new(vec);
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

    #[test]
    fn from_test() {
        let vec: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let grid: Grid<FlagSquare<FlagType<u16>, FlagType<u16>>> = Grid::new(vec);
        let g2: Grid<SimpleSquare<IntType<u16>>> = Grid::from(&grid);
        let mut iter = g2.grid_iter();
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
        assert_eq!(grid, g2);

        let g3: Grid<SimpleSquare<FlagType<u32>>> = Grid::from(&grid);
        let mut iter = g3.grid_iter();
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
        assert_eq!(grid, g2);
        assert_eq!(grid, g3);
        assert_eq!(g2, g3);

    }
}