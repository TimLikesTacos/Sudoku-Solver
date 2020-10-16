use crate::square::*;
use crate::constants::*;
use crate::box_iter::*;
use crate::support::*;
use crate::flag::*;

#[derive (Clone, Debug)]
pub struct Grid <S>
{
    grid: Vec<S>
}

/// Trait for items that will be stored in the grid
// pub trait GridItem: Clone + Default {}

// impl <V: Clone + Default> GridItem for SimpleSquare<V> {}
// impl <V: Clone + Default,F: Clone + Default> GridItem for FlagSquare<V,F> {}

impl <S1: SquareTrait + From<S2>, S2: SquareFlagTrait + Copy> PartialEq<Grid<S1>> for Grid<S2> {
    fn eq(&self, other: &Grid<S1>) -> bool {
        if self.grid.len() != other.grid.len() {
            return false;
        }
        self.grid.iter().zip(other.grid.iter()).any(|(a, b)| S1::from(*a).getv() == b.getv())
    }
}

impl  <S> Grid <S>
where S: Default + Clone + PartialEq
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

    // //Sets a new puzzle using 2-D vector parameter
    // pub fn set_initial(initial: Vec<usize>) -> Grid<S> {
    //     let mut ret: Grid<S> = Grid::new();
    //     initial
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, &c)| c != 0)
    //         .map(|(i, &c)| {ret.grid[i] = S::new(F::from(c ), true)})
    //         .all(|_| true);
    //
    //     ret.set_pencil();
    //     ret
    // }
    //
    // // Assumes that the puzzle has already been initially set
    // fn set_pencil(&mut self)  {
    //     // For all cells that are not fixed...
    //     for i in 0..NUM_CELLS {
    //         if self.grid[i].fixed() {
    //             continue;
    //         }
    //         let new_pencil:Vec<F> = self
    //             .single_iterator(i)
    //             .fold(Vec::new(), |mut acc, x| {acc.push(x.getv()); acc});
    //
    //         self.grid[i].initial_setp(&new_pencil);
    //
    //     }
    // }

    ///* Determines if each element in the grid is the same as the respective element in the other grid.
    /// Note that this function takes two templates, S, and S2.  This requires that S implements `From<S>`.
    pub fn equal_values (&self, other: &Grid<S>) -> bool {
        if self.grid.len() != other.grid.len() {
            return false;
        }
        self.grid.iter().zip(other.grid.iter()).any(|(a, b)| *a == *b)
    }

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