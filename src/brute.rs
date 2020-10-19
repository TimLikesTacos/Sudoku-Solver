use crate::*;
use crate::constants::*;
use crate::puzzle::*;
use crate::grid::*;
use crate::square::*;
use crate::flag::*;

/// Brute force solving.  Returns multiple solutions if they exist
pub trait BruteForce {
    type SquareType;
    fn brute_force_solve(&self) -> Solutions<Self::SquareType>;
    //fn valid_entry(&self, index: usize) -> bool;
}

impl <G: SquareTrait> BruteForce for SimplePuzzle<G> {
    type SquareType = G;


    /// Solves the Sudoku puzzle.  Returns a vector of 1-D vectors.  Each 1-D vector represents a
    /// solution of the sudoku puzzle.  If no solution exists, the vector will be empty.
    fn brute_force_solve(&self) -> Solutions<G> {
        let move_right =  |c| {
            let mut cur = c;
            loop {
                match cur + 1 {
                    v if v >= NUM_CELLS => return None,
                    v => cur = v,
                }
                if !(self.grid[cur].fixed()){
                    break;
                }
            }
            Some(cur)
        };

        let mut puz = self.grid.clone();
        let mut position: usize = 0;
        // Stores solutions
        let mut to_return: Vec<Grid<Self::SquareType>> = Vec::new();

        // move position to non-fixed point
        if puz.grid[position].fixed() {
            position = match move_right(position) {
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

            if puz.valid_entry(position) {
                // if valid, check solved
                if position == NUM_CELLS - 1 {
                    //dbg!(usize::from(position), back_marker);
                    // Copy cell numbers into a new vector to be added to the solutions.
                    let solution = puz.clone();
                    to_return.push(solution);


                    // reset all after backmarker
                    while position > back_marker {
                        // reset starting position, but not the backmarker
                        puz.grid[position].reset_value();
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => match to_return.len() {
                                0 => return Solutions::None,
                                1 => return Solutions::One(to_return[0].clone()),
                                _ => return Solutions::Multi(to_return),
                            },
                        }
                    }
                    assert_eq!(position, back_marker);
                    //increment the position

                    while !puz.grid[position].inc() {
                        puz.grid[position].reset();
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => break 'solving,
                        };

                        back_marker = position;
                    }
                } else {
                    // if valid but not solved,
                    // move to next non-fixed position
                    match move_right(self, position) {
                        Some(v) => position = v,
                        // if last cell is fixed, this will check if the puzzle is valid.
                        None => {
                            position = NUM_CELLS - 1;
                            continue 'solving;
                        }
                    };

                    // Increment the position, if possible
                    puz.grid[position].inc();
                }
            } else {
                // if not valid
                // if not at max
                // increment position
                while !puz.grid[position].inc() {
                    // else reset position
                    // move position to next previous non-fixed
                    puz.grid[position].reset();
                    position = match position.checked_sub(1) {
                        Some(v) => v,
                        None => break 'solving,
                    };
                }
            }
        }
        match to_return.len() {
            0 => return Solutions::None,
            1 => return Solutions::One(to_return[0].clone()),
            _ => return Solutions::Multi(to_return),
        }
    }
}

#[cfg(test)]
pub mod brute_unit {
    use super::*;

    fn get_example() -> Vec<Vec<u8>> {
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
    fn sudoku_test() {
        let example = get_example();

        let expected: Vec<u8> = (vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ])
        .as_input()
        .unwrap();

        let res = Puzzle::new()
            .set_initial(example.as_input().unwrap())
            .brute_force_solve();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], expected);
    }

    #[test]
    fn two_solutions() {
        let example: Vec<Vec<u8>> = vec![
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

        let expected1: Vec<u8> = (vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 2, 7],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 7, 2],
        ])
        .as_input()
        .unwrap();

        let expected2: Vec<u8> = (vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 7, 2],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 2, 7],
        ])
        .as_input()
        .unwrap();

        let res = Puzzle::new_from_vec(example.as_input().unwrap())
            .brute_force_solve();
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
        let example: Vec<Vec<u8>> = vec![
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

        let mut puz: Puzzle<FlagSquare<Flag<u16>, u16>, SimpleSquare<u16>> = Puzzle::new_from_vec(example.as_input().unwrap());
        let res = puz.brute_force_solve();


        assert!(res.len() == 2);

        let example: Vec<Vec<u8>> = vec![
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

        let mut puz: Puzzle<FlagSquare<Flag<u16>, u16>, SimpleSquare<u16>> = Puzzle::new_from_vec(example.as_input().unwrap());

        let res = puz.brute_force_solve();

        // used https://www.thonky.com/sudoku/solution-count to verify solution count
        assert!(res.len() == 192);
    }
}
