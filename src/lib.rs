// mod brute;
// mod conv_input_output;
// mod errors;
// // mod human_calcs;
// mod box_iter;
 //mod constants;
// mod conv_input_output;
mod sq_element;
// mod grid;
// mod puzzle;
 //mod support;
// mod brute;
mod square;

//mod square;

// mod puzzle;
//mod conv_input_output;
//
// pub use crate::brute::BruteForce;
// pub use crate::brute::*;
// pub use crate::conv_input_output::*;
// pub use crate::human_calcs::*;
// pub use crate::support::*;
// use std::collections::{BTreeSet, HashSet};
// use std::iter::Skip;
// pub use crate::constants::*;
//
// use std::slice::{Iter, IterMut};
// use crate::box_iter::BoxIter;
//
// /// The primitive type that is used for the cell number and pencil mark numbers.
// type Element = u8;
//
//
//
// // The Cell struct contains the number, boolean if it is fixed, and functions to incremement
// #[derive(Clone, PartialEq, Debug)]
// pub struct Cell {
//     num: Element,
//     fixed: bool,
//     penciled: BTreeSet<Element>,
// }
// /*
// todo: Implement user pencil marks in cell struct.  In an actual game situation, the user may
//  want to make pencil entries.
// */
//
// impl Default for Cell {
//     fn default() -> Self {
//         Cell {
//             num: 0,
//             fixed: false,
//             penciled: BTreeSet::new(),
//         }
//     }
// }
//
// impl Cell {
//     // Cell getter functions
//     pub fn fixed(&self) -> bool {
//         self.fixed
//     }
//
//     pub fn num(&self) -> Element {
//         self.num
//     }
//
//     pub fn penciled(&self) -> &BTreeSet<Element> {
//         &self.penciled
//     }
//
//     pub fn penciled_mut(&mut self) -> &mut BTreeSet<Element> {
//         &mut self.penciled
//     }
//
//     /// Increment cell.  Returns boolean.  True is incremented, false if already at max value or fixed value.
//     /// Uses pencil marks to get the next value
//     fn inc(&mut self) -> bool {
//         if self.fixed {
//             return false;
//         }
//         match self.penciled.iter().skip_while(|&v| *v <= self.num).next() {
//             Some(v) => {
//                 self.num = *v;
//                 true
//             }
//             None => false,
//         }
//     }
//
//     /// Resets non-fixed cells to zero.
//     fn reset(&mut self) {
//         if !self.fixed {
//             self.num = 0;
//         }
//     }
//
//     /// used in initial setting of puzzle and fixed numbers
//     pub fn set_cell_initial(&mut self, val: Element) {
//         self.num = val;
//         self.fixed = true;
//     }
//
//     fn set(&mut self, val: Element) {
//         self.num = val;
//     }
//
//     pub fn is_possible(&self, val: Element) -> bool {
//         self.penciled.contains(&val)
//     }
//
//     pub fn mark_possible(&mut self, val: Element) -> bool {
//         self.penciled.insert(val)
//     }
//
//     pub fn remove_possible(&mut self, val: Element) -> bool {
//         self.penciled.remove(&val)
//     }
//
//     pub fn poss_iter(&'_ self) -> impl Iterator<Item = &Element> {
//         self.penciled.iter()
//     }
// }
//
//
// /// Contains a row dominant 1-D vector for all the cells in the puzzle
// #[derive(Clone, Debug, Default, PartialEq)]
// pub struct Puzzle {
//     pub cells: Vec<Cell>,
// }
//
//
//
// // impl Puzzle {
// //     pub fn new() -> Puzzle {
// //         Puzzle {
// //             cells: vec![Cell::default(); NUM_CELLS],
// //         }
// //     }
// //
// //     fn row_iter(&self, index: usize) -> impl Iterator<Item = &Cell> {
// //         let row = index_to_row(index);
// //         self.cells[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
// //     }
// //
// //     fn col_iter(&self, index: usize) -> impl Iterator<Item = &Cell> {
// //         let col = index_to_col(index);
// //         self.cells.iter().skip(col).step_by(MAX_NUM)
// //     }
// //
// //     fn box_iter(&self, index: usize) -> BoxIter {
// //         let box_num = index_to_box(index);
// //
// //         BoxIter {
// //             it: self.cells.iter().skip(start_of_box(box_num)),
// //             index: 0,
// //         }
// //     }
// //
// //     fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut Cell> {
// //         let row = index_to_row(index);
// //         self.cells[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
// //     }
// //
// //     fn col_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut Cell> {
// //         let col = index_to_col(index);
// //         self.cells.iter_mut().skip(col).step_by(MAX_NUM)
// //     }
// //
// //     fn box_iter_mut(&mut self, index: usize) -> BoxIterMut {
// //         let box_num = index_to_box(index);
// //
// //         BoxIterMut {
// //             it: self.cells.iter_mut().skip(start_of_box(box_num)),
// //             index: 0,
// //         }
// //     }
// //
// //     /// An iterator that iterates over the row, column, and box that the cell with the parameter `index`.
// //     /// NOTE: The cell of parameter `index` is iterated over 3 times (once for each iterator).
// //     /// There is no mutable version of this as it would require 3 mutable borrows at the same time.
// //     /// todo: create a struct with a slice of the cells and create a mutable iterator to get around borrow issues.
// //     fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ Cell> {
// //         self.box_iter(index)
// //             .chain(self.row_iter(index).chain(self.col_iter(index)))
// //     }
// //
// //     /// Sets a new puzzle using 2-D vector parameter
// //     pub fn set_initial(&mut self, initial: Vec<Element>) -> &mut Self {
// //         initial
// //             .iter()
// //             .enumerate()
// //             .filter(|(_, &c)| c != 0)
// //             .map(|(i, c)| self.cells[i].set_cell_initial(*c))
// //             .all(|_| true);
// //
// //         self.set_penciled()
// //     }
// //
// //     // Assumes that the puzzle has already been initially set
// //     fn set_penciled(&mut self) -> &mut Self {
// //         // For all cells that are not fixed...
// //         for i in 0..NUM_CELLS {
// //             if self.cells[i].fixed() {
// //                 continue;
// //             }
// //             let set = self
// //                 .single_iterator(i)
// //                 .map(|x| x.num())
// //                 .collect::<HashSet<Element>>();
// //             for n in 1..=MAX_NUM {
// //                 if !set.contains(&(n as Element)) {
// //                     self.cells[i].mark_possible(n as Element);
// //                 }
// //             }
// //         }
// //         self
// //     }
// //
// //     /// Checks if the cell is valid by comparing it to other cells in row, column, and associated box.
// //     pub fn valid_entry(&self, index: usize) -> bool {
// //         // The three iterators used to check for validity are "dumb", as in they check the cell in question.
// //         // This will result in 1 match per iterator, therefore the expected Vec length is 3 if it is a valid puzzle.
// //         self.single_iterator(index)
// //             .filter(|x| x.num == self.cells[index].num())
// //             .collect::<Vec<&Cell>>()
// //             .len()
// //             == 3
// //     }
// //
// //     /// Goes over every cell in the puzzle and checks that each cell has a value and that value is valid.
// //     pub fn is_solved(&self) -> bool {
// //         (0..NUM_CELLS)
// //             .into_iter()
// //             .all(|i| self.valid_entry(i) && self.cells[i].num() != 0)
// //     }
// // }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn get_example() -> Vec<Vec<Element>> {
//         vec![
//             vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
//             vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
//             vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
//             vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
//             vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
//             vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
//             vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
//             vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
//             vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
//         ]
//     }
//
//     #[test]
//     fn valid_puz_test() {
//         let example1: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
//         ];
//         let mut test1 = Puzzle::new();
//         test1.set_initial(example1.as_input().unwrap());
//         assert!(test1.is_solved());
//
//         // Same as example 1 but with a zero cell
//         let example2: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 0, 9],
//         ];
//         let mut test2 = Puzzle::new();
//         test2.set_initial(example2.as_input().unwrap());
//         assert!(!test2.is_solved());
//
//         // Same as example 1 but with a double value
//         let example3: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 3, 9],
//         ];
//         let mut test3 = Puzzle::new();
//         test3.set_initial(example3.as_input().unwrap());
//         assert!(!test3.is_solved());
//     }
//
//     #[test]
//     fn mut_iter_test() {
//         let mut puz = Puzzle::new();
//         puz.set_initial(get_example().as_input().unwrap());
//         let mut iter = puz.col_iter_mut(2);
//         iter.next();
//         let cell = iter.next().unwrap();
//         assert_eq!(cell.num(), 0);
//         assert_eq!(cell.penciled.len(), 3);
//         cell.penciled.remove(&2);
//         assert_eq!(cell.penciled.len(), 2);
//         iter.next();
//         assert_eq!(cell.penciled.len(), 2);
//         assert_eq!(cell.num(), 0);
//     }
//     #[test]
//     fn get_box_test() {
//         assert_eq!(index_to_box(10), 0);
//         assert_eq!(index_to_box(26), 2);
//         assert_eq!(index_to_box(30), 4);
//         assert_eq!(index_to_box(80), 8);
//     }
//     #[test]
//     fn row_iter_test() {
//         let example = get_example();
//
//         let example_copy = example.clone();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//         let iter = res.row_iter(0);
//         for (exp, res) in example_copy[0].iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.row_iter(72);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 7);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//         let iter1 = res.row_iter(0);
//         let iter2 = res.row_iter(6);
//         for (c1, c2) in iter1.zip(iter2){
//             assert_eq!(c1, c2);
//         }
//
//     }
//     #[test]
//     fn col_iter_test() {
//         let example = get_example();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//         let iter = res.col_iter(0);
//         let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 5);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn box_iter_test() {
//         let example = get_example();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//
//         let iter = res.box_iter(60);
//         let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//         let iter = res.box_iter(0);
//         let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 5);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn set_pencil_test() {
//         let example = get_example();
//         let mut puz = Puzzle::new();
//         puz.set_initial(example.as_input().unwrap());
//
//         assert_eq!(
//             puz.cells[1]
//                 .poss_iter()
//                 .collect::<HashSet<&Element>>()
//                 .len(),
//             0
//         );
//         let mut cell2 = puz.cells[2].poss_iter().collect::<HashSet<&Element>>();
//
//         assert_eq!(cell2.len(), 3);
//         let expected: [Element; 3] = [1, 2, 4];
//         for exp in expected.iter() {
//             assert!(cell2.remove(&exp));
//         }
//         assert!(cell2.is_empty());
//
//         let mut cell78 = puz.cells[78].poss_iter().collect::<HashSet<&Element>>();
//         assert_eq!(cell78.len(), 4);
//         let expected: [Element; 4] = [1, 3, 4, 6];
//         for exp in expected.iter() {
//             assert!(cell78.remove(&exp));
//         }
//         assert!(cell78.is_empty());
//     }
// }
