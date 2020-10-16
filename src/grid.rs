use crate::box_iter::*;
use crate::constants::*;
use crate::conv_input_output::*;
use crate::flag::*;
use crate::square::*;
use crate::support::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<S> {
    grid: Vec<S>,
}

/// Trait for items that will be stored in the grid
// pub trait GridItem: Clone + Default {}

// impl <V: Clone + Default> GridItem for SimpleSquare<V> {}
// impl <V: Clone + Default,F: Clone + Default> GridItem for FlagSquare<V,F> {}

// impl <S1: SquareTrait + From<S2>, S2: SquareFlagTrait + Copy> PartialEq <Grid<S1>> for Grid<S2> {
//     fn eq(&self, other: &Grid<S1>) -> bool {
//         if self.grid.len() != other.grid.len() {
//             return false;
//         }
//         self.grid.iter().zip(other.grid.iter()).any(|(a, b)| S1::from(*a).getv() == b.getv())
//     }
// }

impl<'a, Flag: 'a + SquareFlagTrait, Simple: SquareTrait + From<&'a Flag>> From<&'a Grid<Flag>>
    for Grid<Simple>
{
    fn from(other: &'a Grid<Flag>) -> Self {
        let transfer = other.grid.iter().fold(Vec::new(), |mut acc, x| {
            acc.push(Simple::from(x));
            acc
        });
        Self {
            grid: transfer.clone(),
        }
    }
}

impl<S> Grid<S>
where
    S: SquareTrait + Default + Clone,
{
    pub fn new() -> Grid<S> {
        Grid {
            grid: vec![S::default(); NUM_CELLS],
        }
    }

    fn row_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    fn col_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let col = index_to_col(index);
        self.grid.iter().skip(col).step_by(MAX_NUM)
    }

    fn box_iter(&self, index: usize) -> BoxIter<S> {
        let box_num = index_to_box(index);

        BoxIter {
            it: self.grid.iter().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    fn col_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut S> {
        let col = index_to_col(index);
        self.grid.iter_mut().skip(col).step_by(MAX_NUM)
    }

    fn box_iter_mut(&mut self, index: usize) -> BoxIterMut<S> {
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
    fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ S> {
        self.box_iter(index)
            .chain(self.row_iter(index).chain(self.col_iter(index)))
    }

    // ///* Determines if each element in the grid is the same as the respective element in the other grid.
    // /// Note that this function takes two templates, S, and S2.  This requires that S implements `From<S>`.
    // pub fn equal_values (&self, other: &Grid<S>) -> bool {
    //     if self.grid.len() != other.grid.len() {
    //         return false;
    //     }
    //     self.grid.iter().zip(other.grid.iter()).any(|(a, b)| *a == *b)
    // }

    // /// Checks if the cell is valid by comparing it to other cells in row, column, and associated box.
    // pub fn valid_entry(&self, index: usize) -> bool {
    //     // The three iterators used to check for validity are "dumb", as in they check the cell in question.
    //     // This will result in 1 match per iterator, therefore the expected Vec length is 3 if it is a valid puzzle.
    //     self.single_iterator(index)
    //         .filter(|x| x.num == self.grid[index].num())
    //         .collect::<Vec<&T>>()
    //         .len()
    //         == 3
    // }
    //
    // /// Goes over every cell in the puzzle and checks that each cell has a value and that value is valid.
    // pub fn is_solved(&self) -> bool {
    //     (0..NUM_CELLS)
    //         .into_iter()
    //         .all(|i| self.valid_entry(i) && self.grid[i].num() != 0)
    // }
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
mod grid_tests {
    use super::*;

    // fn get_example() -> Vec<Vec<u8>> {
    //         vec![
    //         vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    //         vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    //         vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    //         vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    //         vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    //         vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    //         vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    //         vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    //         vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    //     ]
    // }
    //
    // #[test]
    // fn row_iter_test() {
    //     let example:Vec<Vec<u8>> = get_example();
    //
    //     let example_copy = example.clone();
    //
    //     let mut res = Grid::new(example.as_input().unwrap());
    //
    //     let iter = res.row_iter(0);
    //     for (exp, res) in example_copy[0].iter().zip(iter) {
    //         assert_eq!(res.getv(), *exp);
    //     }
    //
    //     let mut iter = res.row_iter(72);
    //
    //     iter.nth(6);
    //
    //     assert_eq!(iter.next().unwrap().getv(), 7);
    //     assert_eq!(iter.next().unwrap().getv(), 9);
    //     assert!(iter.next().is_none());
    //     let iter1 = res.row_iter(0);
    //     let iter2 = res.row_iter(6);
    //     for (c1, c2) in iter1.zip(iter2){
    //         assert_eq!(c1, c2);
    //     }
    //
    // }
    // #[test]
    // fn col_iter_test() {
    //     let example = get_example();
    //
    //     let mut res: Grid<SimpleSquare<u32>> = Grid::set_initial(example.as_input().unwrap());
    //     let iter = res.col_iter(0);
    //     let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
    //     for (exp, res) in expected.iter().zip(iter) {
    //         assert_eq!(res.getv(), *exp);
    //     }
    //
    //     let mut iter = res.col_iter(17);
    //
    //     iter.nth(6);
    //
    //     assert_eq!(iter.next().unwrap().getv(), 5);
    //     assert_eq!(iter.next().unwrap().getv(), 9);
    //     assert!(iter.next().is_none());
    // }
    //
    // #[test]
    // fn box_iter_test() {
    //     let example = get_example();
    //
    //     let mut res: Grid<SimpleSquare<usize>> = Grid::set_initial(example.as_input().unwrap());
    //
    //     let iter = res.box_iter(60);
    //     let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
    //     for (exp, res) in expected.iter().zip(iter) {
    //         assert_eq!(res.getv(), *exp);
    //     }
    //     let iter = res.box_iter(0);
    //     let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
    //     for (exp, res) in expected.iter().zip(iter) {
    //         assert_eq!(res.getv(), *exp);
    //     }
    //
    //     let mut iter = res.col_iter(17);
    //
    //     iter.nth(6);
    //
    //     assert_eq!(iter.next().unwrap().getv(), 5);
    //     assert_eq!(iter.next().unwrap().getv(), 9);
    //     assert!(iter.next().is_none());
    // }
    //
    // #[test]
    // fn mut_iter_test() {
    //
    //     let mut puz: Grid<FlagSquare<u16, Flag<u16>>> = Grid::set_initial(get_example().as_input().unwrap());
    //     let mut iter = puz.col_iter_mut(2);
    //     iter.next();
    //     let cell = iter.next().unwrap();
    //     assert_eq!(cell.getv(), 0);
    //     assert_eq!(cell.getp().count(), 3);
    //     cell.getp_mut().remove_num(2);
    //     assert_eq!(cell.getp().count(), 2);
    //     iter.next();
    //     assert_eq!(cell.getp().count(), 2);
    //     assert_eq!(cell.getv(), 0);
    // }
}
