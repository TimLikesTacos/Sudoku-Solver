

static BOX_DIMEN: usize = 3;
static MAX_NUM: usize = BOX_DIMEN * BOX_DIMEN;
static NUM_CELLS: usize = MAX_NUM * MAX_NUM;


// The Cell struct contains the number, boolean if it is fixed, and functions to incremement
#[derive( Copy, Clone, PartialEq, Debug)]
struct Cell {
    num: usize,
    fixed: bool,
}

impl Default for Cell {
    fn default ()-> Self {
        Cell {
            num: 0,
            fixed: false,
        }
    }

}

impl Cell {

    // Cell getter functions
    fn fixed(&self) -> bool {self.fixed}
    fn num(&self) -> usize {self.num}

    // Increment cell.  Returns boolean.  True is incremented, false if already at max value or fixed value.
    fn inc (&mut self) -> bool {
        if self.fixed {
            return false;
        }
        if self.num < MAX_NUM {
            self.num = self.num + 1;
            true
        } else {false}
    }

    fn reset (&mut self) {
        if !self.fixed {
            self.num = 0;
        }
    }

    // used in initial setting of puzzle and fixed numbers
    fn set (&mut self, val: usize) {
        self.num = val;
        self.fixed = true;
    }

}

// Contains a row dominant 1-D vector for all the cells in the puzzle
#[derive(Clone, Debug)]
pub struct Puzzle {
    cells: Vec<Cell>,
}

impl Puzzle {
    fn new () -> Puzzle {
        Puzzle {
            cells: vec![Cell::default(); NUM_CELLS]
        }
    }

    // convert 2-D coordinate to 1-D
    fn get_cell (row: usize, col: usize) -> usize {
        row * MAX_NUM + col
    }

    fn get_row (index: usize) -> usize { index / MAX_NUM }
    fn get_col (index: usize) -> usize {index % MAX_NUM }

    // This is configured to solve a Leet-Code problem, which used a 2-D vector of chars for input,
    // that uses the '.' char to designate non-fixed cells.
    fn set_initial (&mut self, initial: Vec<Vec<usize>>) ->&mut Puzzle {
        for (row, row_vec) in initial.iter().enumerate() {
            for (col, cell) in row_vec.iter().enumerate() {
                if *cell == 0 {
                    continue;
                } else {
                    self.cells[Puzzle::get_cell(row, col)].set(*cell);
                }

            }
        }
        self

    }

    fn check_row (&self, cell: usize) -> bool {
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

    fn check_col (&self, cell: usize) -> bool {
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

    fn check_box (&self, cell: usize) -> bool {
        let right: usize = self.cells[cell].num();
        let (row, col): (usize, usize) = (Puzzle::get_row(cell), Puzzle::get_col(cell));

        //get box coordinate
        let box_num: (usize, usize) = (row / BOX_DIMEN, col / BOX_DIMEN);
        let coord: (usize, usize) = (row % BOX_DIMEN, col % BOX_DIMEN);

        // go through each cell in the box
        for r in 0..BOX_DIMEN {
            for c in 0..BOX_DIMEN {
                let other_coord:(usize, usize) = (box_num.0 * BOX_DIMEN + r, box_num.1 * BOX_DIMEN + c);
                // Prevents self-checking
                if coord != other_coord {
                    let left = self.cells[Puzzle::get_cell(other_coord.0, other_coord.1)].num();
                    if left == right {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn valid (&self, cell: usize) -> bool {
        self.check_col(cell) && self.check_box(cell) && self.check_row(cell) && self.cells[cell].num() != 0
    }

    pub fn solve (&mut self) -> Vec<Vec<usize>> {

        let mut position: usize = 0;
        loop {
            // Exit condition for invalid puzzle
            if position == 0 && (self.cells[position].fixed() || self.cells[position].num() == MAX_NUM){
                if !self.valid(position) {
                    panic!("Unsolvable");
                }
            }

            // solved puzzle
            if position == NUM_CELLS - 1 && self.valid(position) {
                let mut to_return: Vec<Vec<usize>> = vec![vec![0; MAX_NUM]; MAX_NUM];
                for (i, cell) in self.cells.iter().enumerate() {
                    to_return[Puzzle::get_row(i)][Puzzle::get_col(i)] = cell.num();
                }
                return to_return;
            }

            if position < MAX_NUM {
                self.cells[position].inc();
            }
            dbg!(position, self.cells[position].num());

            if self.valid(position) {
                position += 1;
            } else {
                while (self.cells[position].num() == MAX_NUM || self.cells[position].fixed()){
                    self.cells[position].reset();
                    position = match position.checked_sub(1){
                        Some (v) => v,
                        None => panic!("Unsolvable")
                    }
                }



            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sudoku_test() {
        let example:Vec<Vec<usize>> = vec![vec![5, 3, 0, 0, 7, 0, 0, 0, 0], vec![6, 0, 0, 1, 9, 5, 0, 0, 0],vec![0, 9, 8, 0, 0, 0, 0, 6, 0],vec![8, 0, 0, 0, 6, 0, 0, 0, 3],vec![4, 0, 0, 8, 0, 3, 0, 0, 1], vec![7, 0, 0, 0, 2, 0, 0, 0, 6], vec![0, 6, 0, 0, 0, 0, 2, 8, 0], vec![0, 0, 0, 4, 1, 9, 0, 0, 5], vec![0, 0, 0, 0, 8, 0, 0, 7, 9]];



        let expected:Vec<Vec<usize>> = vec![vec![5, 3, 4, 6, 7, 8, 9, 1, 2], vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
                                            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],vec![8, 5, 9, 7, 6, 1, 4 ,2, 3],
                                            vec![4, 2, 6, 8, 5, 3, 7, 9, 1], vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
                                            vec![9, 6, 1, 5, 3, 7, 2, 8, 4], vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
                                            vec![3, 4, 2, 2, 8, 6, 1, 7, 9]];


        let mut puz:Puzzle = Puzzle::new();
        let res = puz.set_initial(example).solve();

        assert_eq!(res, expected);
    }
}
