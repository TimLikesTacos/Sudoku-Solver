use crate::*;

/// Brute force solving.  Returns multiple solutions if they exist
pub trait BruteForce {
    fn brute_force_solve(&mut self) -> Vec<Vec<Element>>;
}

impl BruteForce for Puzzle {
    /// Solves the Sudoku puzzle.  Returns a vector of 1-D vectors.  Each 1-D vector represents a
    /// solution of the sudoku puzzle.  If no solution exists, the vector will be empty.
    fn brute_force_solve(&mut self) -> Vec<Vec<Element>> {
        fn move_cursor_left(puz: &mut Puzzle, cursor: usize) -> Option<usize> {
            let mut cur = cursor;
            loop {
                puz.cells[cur].reset();
                cur = match cur.checked_sub(1) {
                    Some(v) => v,
                    // At beginning of puzzle
                    None => return None,
                };
                if !(puz.cells[cur].fixed() || puz.cells[cur].num() == MAX_NUM as Element) {
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
        let mut to_return: Vec<Vec<Element>> = Vec::new();

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

            if self.valid_entry(position) {
                // if valid, check solved
                if position == NUM_CELLS - 1 {
                    //dbg!(usize::from(position), back_marker);
                    // Copy cell numbers into a new vector to be added to the solutions.
                    let mut solution: Vec<Element> = self.cells.iter().map(|c| c.num()).collect();
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
                if (self.cells[position].num() as usize)< MAX_NUM {
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

                        if !(self.cells[position].fixed() || self.cells[position].num() == MAX_NUM as Element)
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