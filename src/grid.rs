use crate::square::*;
use crate::constants::*;
use crate::box_iter::*;
use crate::support::*;
use crate::flag::*;

pub struct Grid <S, F>
where S: SquareTrait<FlagType = F, Value = u8> + Default, F: FlagTrait + Default
{
    grid: Vec<S>
}

impl <T: SquareTrait<FlagType = F, Value = u8> + Clone + Default, F: FlagTrait + Default> Grid <T, F> {
    pub fn new() -> Grid<T, F> {
        Grid {
            grid: vec![T::default(); NUM_CELLS],
        }
    }

    fn row_iter(&self, index: usize) -> impl Iterator<Item = &T> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    fn col_iter(&self, index: usize) -> impl Iterator<Item = &T> {
        let col = index_to_col(index);
        self.grid.iter().skip(col).step_by(MAX_NUM)
    }

    fn box_iter(&self, index: usize) -> BoxIter<T> {
        let box_num = index_to_box(index);

        BoxIter {
            it: self.grid.iter().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut T> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    fn col_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut T> {
        let col = index_to_col(index);
        self.grid.iter_mut().skip(col).step_by(MAX_NUM)
    }

    fn box_iter_mut(&mut self, index: usize) -> BoxIterMut<T> {
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
    fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ T> {
        self.box_iter(index)
            .chain(self.row_iter(index).chain(self.col_iter(index)))
    }

    /// Sets a new puzzle using 2-D vector parameter
    pub fn set_initial(&mut self, initial: Vec<usize>) -> () {
        initial
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, c)| self.grid[i] = T::new(*c as u8, true))
            .all(|_| true);

       // self.set_pencil()
    }

    // // Assumes that the puzzle has already been initially set
    // fn set_pencil(&mut self) -> &mut Self {
    //     // For all cells that are not fixed...
    //     for i in 0..NUM_CELLS {
    //         if self.grid[i].fixed() {
    //             continue;
    //         }
    //         let set = self
    //             .single_iterator(i);
    //            // .fold(T::new(0, false).getp(), |acc, x|, )
    //         for n in 1..=MAX_NUM {
    //             if !set.contains(&(n as Element)) {
    //                 self.grid[i].mark_possible(n as U);
    //             }
    //         }
    //     }
    //     self
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