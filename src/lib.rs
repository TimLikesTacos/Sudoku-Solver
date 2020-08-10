use std::collections::HashSet;

static BOX_DIMEN: usize = 3;
static MAX_NUM: usize = BOX_DIMEN * BOX_DIMEN;
static NUM_CELLS: usize = MAX_NUM * MAX_NUM;

// The Cell struct contains the number, boolean if it is fixed, and functions to incremement
#[derive( Clone, PartialEq, Debug)]
struct Cell {
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

    // Increment cell.  Returns boolean.  True is incremented, false if already at max value or fixed value.
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

    fn reset(&mut self) {
        if !self.fixed {
            self.num = 0;
        }
    }

    // used in initial setting of puzzle and fixed numbers
    fn set_cell_initial(&mut self, val: usize) {
        self.num = val;
        self.fixed = true;
    }

    fn set (&mut self, val: usize) {
        self.num = val;
    }

    fn is_possible (&self, val: usize) -> bool {
        self.penciled.contains(val)
    }

    fn mark_possible (&mut self, val: usize) {
        self.penciled.insert(val);
    }

    fn remove_possible (&mut self, val: usize) {
        self.penciled.remove(&val);
    }
}

struct CellIter<'a> {
    i_penciled: &'a HashSet<usize>,
    index: usize,
}

impl <'a> IntoIterator for &'a Cell{
    type Item = usize;
    type IntoIter = CellIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CellIter {
            i_penciled: &self.penciled,
            index: 0,
        }
    }
}

impl <'a> Iterator for CellIter<'a> {
    type Item = usize;
    fn next (&mut self) -> Option<usize> {
        let res = match self.i_penciled.get(self.index){
            None => None,
            Some(v) => Some(*v),
        };
        self.index += 1;
        res
    }
}
// Contains a row dominant 1-D vector for all the cells in the puzzle
#[derive(Clone, Debug, Default)]
pub struct Puzzle {
    cells: Vec<Cell>,
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {
            cells: vec![Cell::default(); NUM_CELLS],
        }
    }

    // convert 2-D coordinate to 1-D
    fn get_cell(row: usize, col: usize) -> usize {
        row * MAX_NUM + col
    }

    // covert 1-D coordinate to 2-D
    fn get_row(index: usize) -> usize {
        index / MAX_NUM
    }
    fn get_col(index: usize) -> usize {
        index % MAX_NUM
    }

    // Assumes that the puzzle has already been initially set
    pub fn set_penciled (&mut self) -> &mut Puzzle {

        let r_iter = self.cells.iter_mut();

    }

    /// Sets a new puzzle using 2-D vector parameter
    pub fn set_initial(&mut self, initial: Vec<Vec<usize>>) -> &mut Puzzle {
        for (row, row_vec) in initial.iter().enumerate() {
            for (col, cell) in row_vec.iter().enumerate() {
                if *cell == 0 {
                    continue;
                } else {
                    self.cells[Puzzle::get_cell(row, col)].set_cell_initial(*cell);
                }
            }
        }
        self
    }

    // Used to check for valid entry along the row
    fn check_row(&self, cell: usize) -> bool {
        let right: usize = self.cells[cell].num();
        let (row, col): (usize, usize) = (Puzzle::get_row(cell), Puzzle::get_col(cell));

        for c in 0..MAX_NUM {
            if c != col {
                let left = self.cells[Puzzle::get_cell(row, c)].num();

                if left == right {
                    return false;
                }
            }
        }
        true
    }

    // Used to check for valid entry along the column
    fn check_col(&self, cell: usize) -> bool {
        let right: usize = self.cells[cell].num();
        let (row, col): (usize, usize) = (Puzzle::get_row(cell), Puzzle::get_col(cell));

        for r in 0..MAX_NUM {
            if r != row {
                let left = self.cells[Puzzle::get_cell(r, col)].num();
                if left == right {
                    return false;
                }
            }
        }
        true
    }
    // Used to check for valid entry in the associated box.
    fn check_box(&self, cell: usize) -> bool {
        let right: usize = self.cells[cell].num();
        let (row, col): (usize, usize) = (Puzzle::get_row(cell), Puzzle::get_col(cell));

        //get box coordinate
        let box_num: (usize, usize) = (row / BOX_DIMEN, col / BOX_DIMEN);

        // go through each cell in the box
        for r in 0..BOX_DIMEN {
            for c in 0..BOX_DIMEN {
                let other_coord: (usize, usize) =
                    (box_num.0 * BOX_DIMEN + r, box_num.1 * BOX_DIMEN + c);
                // Prevents self-checking
                if (row, col) != other_coord {
                    let left = self.cells[Puzzle::get_cell(other_coord.0, other_coord.1)].num();
                    if left == right {
                        return false;
                    }
                }
            }
        }
        true
    }

    // Simple collection of validity checks
    fn valid(&self, cell: usize) -> bool {
        self.check_col(cell)
            && self.check_box(cell)
            && self.check_row(cell)
            && self.cells[cell].num() != 0
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
                        solution[Puzzle::get_row(i)][Puzzle::get_col(i)] = cell.num();
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

                    //increment the position
                    self.cells[position].inc();
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
    #[test]
    fn sudoku_test() {
        let example: Vec<Vec<usize>> = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

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