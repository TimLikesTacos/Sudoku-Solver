mod human_calcs;
mod support;

use crate::support::*;
use std::collections::HashSet;
use std::iter::Skip;

use std::slice::{Iter, IterMut};

static BOX_DIMEN: usize = 3;
static MAX_NUM: usize = BOX_DIMEN * BOX_DIMEN;
static NUM_CELLS: usize = MAX_NUM * MAX_NUM;

/// Enum to determine if pencil marks are made by user (for game usage)
/// or by the system (for solving algorithms)
// #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// pub enum Pencil {
//     User(usize),
//     Calcs(usize),
// }

// The Cell struct contains the number, boolean if it is fixed, and functions to incremement
#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    num: usize,
    fixed: bool,
    penciled: HashSet<usize>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            num: 0,
            fixed: false,
            penciled: HashSet::new(),
        }
    }
}

impl Cell {
    // Cell getter functions
    fn fixed(&self) -> bool {
        self.fixed
    }
    fn num(&self) -> usize {
        self.num
    }

    /// Increment cell.  Returns boolean.  True is incremented, false if already at max value or fixed value.
    fn inc(&mut self) -> bool {
        if self.fixed {
            return false;
        }
        if self.num < MAX_NUM {
            self.num += 1;
            true
        } else {
            false
        }
    }

    /// Resets non-fixed cells to zero.
    fn reset(&mut self) {
        if !self.fixed {
            self.num = 0;
        }
    }

    /// used in initial setting of puzzle and fixed numbers
    pub fn set_cell_initial(&mut self, val: usize) {
        self.num = val;
        self.fixed = true;
    }

    fn set(&mut self, val: usize) {
        self.num = val;
    }

    fn is_possible(&self, val: usize) -> bool {
        self.penciled.contains(&val)
    }

    fn mark_possible(&mut self, val: usize) -> bool {
        self.penciled.insert(val)
    }

    fn remove_possible(&mut self, val: usize) -> bool {
        self.penciled.remove(&val)
    }

    pub fn poss_iter(&'_ self) -> impl Iterator<Item = &usize> {
        self.penciled.iter()
    }

    pub fn get_penciled(&mut self) -> &mut HashSet<usize> {
        &mut self.penciled
    }
}

/// Contains a row dominant 1-D vector for all the cells in the puzzle
#[derive(Clone, Debug, Default)]
pub struct Puzzle {
    pub cells: Vec<Cell>,
}

pub struct BoxIter<'a> {
    it: Skip<Iter<'a, Cell>>,
    index: usize,
}

pub struct BoxIterMut<'a> {
    it: Skip<IterMut<'a, Cell>>,
    index: usize,
}

impl<'a> Iterator for BoxIter<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<&'a Cell> {
        self.index += 1;
        // This check prevents skipping to next row in the upcoming match statement
        if self.index == 1 {
            return self.it.next();
        }

        match self.index {
            v if v > MAX_NUM => None,
            v if v % BOX_DIMEN == 1 => self.it.nth(MAX_NUM - BOX_DIMEN),
            _ => self.it.next(),
        }
    }
}

impl<'a> Iterator for BoxIterMut<'a> {
    type Item = &'a mut Cell;
    fn next(&mut self) -> Option<&'a mut Cell> {
        self.index += 1;
        // This check prevents skipping to next row in the upcoming match statement
        if self.index == 1 {
            return self.it.next();
        }

        match self.index {
            v if v > MAX_NUM => None,
            v if v % BOX_DIMEN == 1 => self.it.nth(MAX_NUM - BOX_DIMEN),
            _ => self.it.next(),
        }
    }
}
impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {
            cells: vec![Cell::default(); NUM_CELLS],
        }
    }

    fn row_iter(&self, index: usize) -> Iter<Cell> {
        let row = get_row(index);
        self.cells[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    fn col_iter(&self, index: usize) -> std::iter::StepBy<Skip<Iter<Cell>>> {
        let col = get_col(index);
        self.cells.iter().skip(col).step_by(MAX_NUM)
    }

    fn box_iter(&self, index: usize) -> BoxIter {
        let box_num = get_box(index);
        let start_row = (box_num / BOX_DIMEN) * BOX_DIMEN;
        let start_col = (box_num % BOX_DIMEN) * BOX_DIMEN;

        BoxIter {
            it: self.cells.iter().skip(start_row * MAX_NUM + start_col),
            index: 0,
        }
    }

    fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut Cell> {
        let row = get_row(index);
        self.cells[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    fn col_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut Cell> {
        let col = get_col(index);
        self.cells.iter_mut().skip(col).step_by(MAX_NUM)
    }

    fn box_iter_mut(&mut self, index: usize) -> BoxIterMut {
        let box_num = get_box(index);
        let start_row = (box_num / BOX_DIMEN) * BOX_DIMEN;
        let start_col = (box_num % BOX_DIMEN) * BOX_DIMEN;

        BoxIterMut {
            it: self.cells.iter_mut().skip(start_row * MAX_NUM + start_col),
            index: 0,
        }
    }

    // Assumes that the puzzle has already been initially set
    fn set_penciled(&mut self) -> &mut Self {
        // For all cells that are not fixed...
        for i in 0..NUM_CELLS {
            if self.cells[i].fixed() {
                continue;
            }
            let set = self
                .single_iterator(i)
                .map(|x| x.num())
                .collect::<HashSet<usize>>();
            for n in 1..=MAX_NUM {
                if !set.contains(&n) {
                    self.cells[i].mark_possible(n);
                }
            }
        }
        self
    }

    /// Sets a new puzzle using 2-D vector parameter
    pub fn set_initial(&mut self, initial: Vec<Vec<usize>>) -> &mut Self {
        for (row, row_vec) in initial.iter().enumerate() {
            for (col, cell) in row_vec.iter().enumerate() {
                if *cell == 0 {
                    continue;
                } else {
                    self.cells[get_cell(row, col)].set_cell_initial(*cell);
                }
            }
        }
        self.set_penciled()
    }

    fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ Cell> {
        self.box_iter(index)
            .chain(self.row_iter(index).chain(self.col_iter(index)))
    }

    // fn single_iterator_mut(&mut self, index: usize) -> impl Iterator<Item = &mut Cell> {
    //
    //     self.box_iter_mut(index)
    //         .chain(self.row_iter_mut(index).chain(self.col_iter_mut(index)))
    //
    //  }

    /// Checks if the cell is valid by comparing it to other cells in row, column, and associated box.
    pub fn valid(&self, index: usize) -> bool {
        // The three iterators used to check for validity are "dumb", as in they check the cell in question.
        // This will result in 1 match per iterator, therefore the expected Vec length is 3 if it is a valid puzzle.
        self.single_iterator(index)
            .filter(|x| x.num == self.cells[index].num())
            .collect::<Vec<&Cell>>()
            .len()
            == 3
    }

    /// Solves the Sudoku puzzle.  Returns a vector of 2-D vectors.  Each 2-D vector represents a
    /// solution of the sudoku puzzle.  If no solution exists, the vector will be empty.
    pub fn brute_force_solve(&mut self) -> Vec<Vec<Vec<usize>>> {
        fn move_cursor_left(puz: &mut Puzzle, cursor: usize) -> Option<usize> {
            let mut cur = cursor;
            loop {
                puz.cells[cur].reset();
                cur = match cur.checked_sub(1) {
                    Some(v) => v,
                    // At beginning of puzzle
                    None => return None,
                };
                if !(puz.cells[cur].fixed() || puz.cells[cur].num() == MAX_NUM) {
                    break;
                }
            }
            Some(cur)
        }

        fn move_cursor_right(puz: &Puzzle, cursor: usize) -> Option<usize> {
            let mut cur = cursor;
            loop {
                match cur + 1 {
                    v if v >= NUM_CELLS => return None,
                    v => cur = v,
                }
                if !(puz.cells[cur].fixed()) {
                    break;
                }
            }
            Some(cur)
        }

        let mut position: usize = 0;
        let mut to_return: Vec<Vec<Vec<usize>>> = Vec::new();

        // move position to non-fixed point
        if self.cells[position].fixed() {
            position = match move_cursor_right(self, position) {
                Some(v) => v,
                None => MAX_NUM - 1,
            };
        }

        // set backmarker to the last cell
        let mut back_marker: usize = NUM_CELLS - 1;

        // This loop increments, checks, determine if solved, and adjust the backmarker to check
        // for additional solutions.
        'solving: loop {
            // check valid

            if self.valid(position) {
                // if valid, check solved
                if position == NUM_CELLS - 1 {
                    //dbg!(usize::from(position), back_marker);
                    // Covert self to 2-D vector to add to solution vector
                    let mut solution: Vec<Vec<usize>> = vec![vec![0; MAX_NUM]; MAX_NUM];
                    for (i, cell) in self.cells.iter().enumerate() {
                        solution[get_row(i)][get_col(i)] = cell.num();
                    }
                    to_return.push(solution);

                    // reset all after backmarker
                    while position > back_marker {
                        // reset starting position, but not the backmarker
                        self.cells[position].reset();
                        position = match move_cursor_left(self, position) {
                            Some(v) => v,
                            None => return to_return,
                        }
                    }
                    assert_eq!(position, back_marker);
                    //increment the position

                    if !self.cells[position].inc() {
                        position = match move_cursor_left(self, position) {
                            Some(v) => v,
                            // No other solutions avail
                            None => break 'solving,
                        };
                        back_marker = position;
                        self.cells[position].inc();
                    }
                } else {
                    // if valid but not solved,
                    // move to next non-fixed position
                    match move_cursor_right(self, position) {
                        Some(v) => position = v,
                        // if last cell is fixed, this will check if the puzzle is valid.
                        None => {
                            position = NUM_CELLS - 1;
                            continue 'solving;
                        }
                    };

                    /*
                        increment the position.  This if statement needed to finish solving for human calcs.
                        'Human_calcs' functions could have changed the value, while the cell is not listed as "fixed"
                        Without the if statement, a valid number would be incremented into a non-valid and the puzzle unsolvable.
                     */
                    if self.cells[position].num() == 0 {
                        self.cells[position].inc();
                    }
                }
            } else {
                // if not valid
                // if not at max
                if self.cells[position].num() < MAX_NUM {
                    //increment the position
                    self.cells[position].inc();
                } else {
                    // else reset position
                    // move position to next previous non-fixed
                    'move_left: loop {
                        self.cells[position].reset();
                        position = match move_cursor_left(self, position) {
                            Some(v) => v,
                            // No more valid solutions
                            None => break 'solving,
                        };

                        if !(self.cells[position].fixed() || self.cells[position].num() == MAX_NUM)
                        {
                            break 'move_left;
                        }
                    }
                    self.cells[position].inc();
                }
            }
        }
        to_return
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> Vec<Vec<usize>> {
        vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
    }

    #[test]
    fn mut_iter_test() {
        let mut puz = Puzzle::new();
        puz.set_initial(get_example());
        let mut iter = puz.col_iter_mut(2);
        iter.next();
        let cell = iter.next().unwrap();
        assert_eq!(cell.num(), 0);
        assert_eq!(cell.penciled.len(), 3);
        cell.penciled.remove(&2);
        assert_eq!(cell.penciled.len(), 2);
        iter.next();
        assert_eq!(cell.penciled.len(), 2);
        assert_eq!(cell.num(), 0);
    }
    #[test]
    fn get_box_test() {
        assert_eq!(get_box(10), 0);
        assert_eq!(get_box(26), 2);
        assert_eq!(get_box(30), 4);
        assert_eq!(get_box(80), 8);
    }
    #[test]
    fn row_iter_test() {
        let example = get_example();

        let example_copy = example.clone();

        let mut res = Puzzle::new();
        res.set_initial(example);
        let iter = res.row_iter(0);
        for (exp, res) in example_copy[0].iter().zip(iter) {
            assert_eq!(res.num(), *exp);
        }

        let mut iter = res.row_iter(72);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().num(), 7);
        assert_eq!(iter.next().unwrap().num(), 9);
        assert!(iter.next().is_none());
    }
    #[test]
    fn col_iter_test() {
        let example = get_example();

        let mut res = Puzzle::new();
        res.set_initial(example);
        let iter = res.col_iter(0);
        let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.num(), *exp);
        }

        let mut iter = res.col_iter(17);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().num(), 5);
        assert_eq!(iter.next().unwrap().num(), 9);
        assert!(iter.next().is_none());
    }

    #[test]
    fn box_iter_test() {
        let example = get_example();

        let mut res = Puzzle::new();
        res.set_initial(example);

        let iter = res.box_iter(60);
        let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.num(), *exp);
        }
        let iter = res.box_iter(0);
        let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.num(), *exp);
        }

        let mut iter = res.col_iter(17);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().num(), 5);
        assert_eq!(iter.next().unwrap().num(), 9);
        assert!(iter.next().is_none());
    }

    #[test]
    fn set_pencil_test() {
        let example = get_example();
        let mut puz = Puzzle::new();
        puz.set_initial(example);

        assert_eq!(
            puz.cells[1].poss_iter().collect::<HashSet<&usize>>().len(),
            0
        );
        let mut cell2 = puz.cells[2].poss_iter().collect::<HashSet<&usize>>();

        assert_eq!(cell2.len(), 3);
        let expected: [usize; 3] = [1, 2, 4];
        for exp in expected.iter() {
            assert!(cell2.remove(&exp));
        }
        assert!(cell2.is_empty());

        let mut cell78 = puz.cells[78].poss_iter().collect::<HashSet<&usize>>();
        assert_eq!(cell78.len(), 4);
        let expected: [usize; 4] = [1, 3, 4, 6];
        for exp in expected.iter() {
            assert!(cell78.remove(&exp));
        }
        assert!(cell78.is_empty());
    }
    #[test]
    fn sudoku_test() {
        let example = get_example();

        let expected: Vec<Vec<usize>> = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        let res = Puzzle::new().set_initial(example).brute_force_solve();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], expected);
    }

    #[test]
    fn two_solutions() {
        let example: Vec<Vec<usize>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 0, 0],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 0, 0],
        ];

        let expected1: Vec<Vec<usize>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 2, 7],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 7, 2],
        ];

        let expected2: Vec<Vec<usize>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 7, 2],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 2, 7],
        ];

        let res = Puzzle::new().set_initial(example).brute_force_solve();
        assert_eq!(res.len(), 2);
        if res[0] == expected1 {
            assert_eq!(res[0], expected1);
            assert_eq!(res[1], expected2);
        } else {
            assert_eq!(res[0], expected2);
            assert_eq!(res[1], expected1);
        }
    }

    #[test]
    fn oh_no_test() {
        let example: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut puz: Puzzle = Puzzle::new();
        let res = puz.set_initial(example).brute_force_solve();

        assert!(res.len() == 2);

        let example: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut puz: Puzzle = Puzzle::new();
        let res = puz.set_initial(example).brute_force_solve();

        // used https://www.thonky.com/sudoku/solution-count to verify solution count
        assert!(res.len() == 192);
    }
}
