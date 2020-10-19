use crate::box_iter::*;
use crate::constants::*;
use crate::conv_input_output::*;
use crate::flag::*;
use crate::square::*;
use crate::support::*;
use std::ops::{Index, IndexMut};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<S> {
    pub(crate) grid: Vec<S>,
}


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
    S: SquareTrait + Default,
{
    pub fn new() -> Grid<S> {
        Grid {
            grid: vec![S::default(); NUM_CELLS],
        }
    }

    pub fn grid_iter (&self) -> impl Iterator <Item= &S>{
        self.grid.iter()
    }

    pub fn grid_iter_mut(&mut self) -> impl Iterator <Item = &mut S> {
        self.grid.iter_mut()
    }
    pub fn row_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    pub fn col_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let col = index_to_col(index);
        self.grid.iter().skip(col).step_by(MAX_NUM)
    }

    pub fn box_iter(&self, index: usize) -> BoxIter<S> {
        let box_num = index_to_box(index);

        BoxIter {
            it: self.grid.iter().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    pub fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    pub fn col_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut S> {
        let col = index_to_col(index);
        self.grid.iter_mut().skip(col).step_by(MAX_NUM)
    }

    pub fn box_iter_mut(&mut self, index: usize) -> BoxIterMut<S> {
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

    pub fn valid_entry(&self, index: usize) -> bool{
        let mut v:Vec<S::Value> = Vec::new();
        self.single_iterator(index)
            .filter(|&s| s.getv() != S::Value::from(0))
            .all ( |s| {
                if v.contains(&s.getv()){false}
                else {v.push(s.getv().clone()); true}
            })

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

