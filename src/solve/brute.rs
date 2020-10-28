use crate::grid::constants::*;
use crate::grid::valid::ValidEntry;
use crate::grid::*;
use crate::puzzle::*;
use crate::solve::solution_report::{Solution, SolutionReport, SolveTech};
use crate::square::*;

/// Brute force solving.  Returns multiple solutions if they exist
pub trait BruteForce<S: Square> {
    fn brute_force_solve(&self) -> SolutionReport<S>;
    //fn valid_entry(&self, index: usize) -> bool;
}

impl<S: Square> BruteForce<S> for Puzzle<S> {
    /// Solves the Sudoku puzzle.  Returns a vector of 1-D vectors.  Each 1-D vector represents a
    /// solution of the sudoku puzzle.  If no solution exists, the vector will be empty.
    fn brute_force_solve(&self) -> SolutionReport<S> {
        let move_right = |p: &Grid<S>, c: usize| {
            let mut cur = c;
            loop {
                match cur + 1 {
                    v if v >= NUM_CELLS => return None,
                    v => cur = v,
                }
                if !(p[cur].fixed()) {
                    break;
                }
            }
            Some(cur)
        };

        let mut guesses: usize = 0;
        let mut to_return: Vec<Grid<S>> = Vec::new();

        let mut puz: Grid<S> = self.board.clone();
        let mut position: usize = 0;
        // Stores solutions

        // move position to non-fixed point
        if puz[position].fixed() {
            position = match move_right(&puz, position) {
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
                    let solution: Grid<S> = puz.clone();
                    to_return.push(solution);

                    // reset all after backmarker
                    while position > back_marker {
                        // reset starting position, but not the backmarker
                        puz[position].reset_value();
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => {
                                let tech = vec![SolveTech::Guesses(guesses)];
                                match to_return.len() {
                                    0 => return SolutionReport::new(Solution::None, tech),
                                    1 => {
                                        return SolutionReport::new(
                                            Solution::One(to_return[0].clone()),
                                            tech,
                                        )
                                    }
                                    _ => {
                                        return SolutionReport::new(
                                            Solution::Multi(to_return),
                                            tech,
                                        )
                                    }
                                }
                            }
                        }
                    }

                    assert_eq!(position, back_marker);
                    //increment the position

                    while !puz[position].inc() {
                        puz[position].reset_value();
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => break 'solving,
                        };

                        back_marker = position;
                    }
                    guesses += 1;
                } else {
                    // if valid but not solved,
                    // move to next non-fixed position
                    match move_right(&puz, position) {
                        Some(v) => position = v,
                        // if last cell is fixed, this will check if the puzzle is valid.
                        None => {
                            position = NUM_CELLS - 1;
                            continue 'solving;
                        }
                    };

                    // Increment the position, if possible
                    if puz[position].inc() {
                        guesses += 1;
                    }
                }
            } else {
                // if not valid
                // if not at max
                // increment position
                while !puz[position].inc() {
                    // else reset position
                    // move position to next previous non-fixed
                    puz[position].reset_value();
                    position = match position.checked_sub(1) {
                        Some(v) => v,
                        None => break 'solving,
                    };
                }
                guesses += 1;
            }
        }
        let tech = vec![SolveTech::Guesses(guesses)];
        match to_return.len() {
            0 => return SolutionReport::new(Solution::None, tech),
            1 => return SolutionReport::new(Solution::One(to_return[0].clone()), tech),
            _ => return SolutionReport::new(Solution::Multi(to_return), tech),
        }
    }
}

#[cfg(test)]
pub mod brute_unit {
    use super::*;
    use crate::conv_input_output::PuzInput;
    use crate::puzzle::*;
    use crate::sq_element::{FlagType, IntType};

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

        let res: Puzzle<SimpleSquare<IntType<u8>>> = Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 1);
        let sol = res.solution.get_solution();
        assert!(match sol {
            Solution::One(v) => {
                assert!(v
                    .grid_iter()
                    .zip(expected.iter())
                    .all(|(v, e)| v.exportv() == *e));
                true
            }
            _ => false,
        });

        let res: Puzzle<SimpleSquare<FlagType<u16>>> = Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 1);
        let sol = res.solution.get_solution();
        assert!(match sol {
            Solution::One(v) => {
                assert!(v
                    .grid_iter()
                    .zip(expected.iter())
                    .all(|(v, e)| v.exportv() == *e));
                true
            }
            _ => false,
        });

        let res: Puzzle<FlagSquare<IntType<u8>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 1);
        let sol = res.solution.get_solution();
        assert!(match sol {
            Solution::One(v) => {
                assert!(v
                    .grid_iter()
                    .zip(expected.iter())
                    .all(|(v, e)| v.exportv() == *e));
                true
            }
            _ => false,
        });

        let res: Puzzle<FlagSquare<FlagType<u16>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 1);
        let sol = res.solution.get_solution();
        assert!(match sol {
            Solution::One(v) => {
                assert!(v
                    .grid_iter()
                    .zip(expected.iter())
                    .all(|(v, e)| v.exportv() == *e));
                true
            }
            _ => false,
        });

        let res: Puzzle<FlagSquare<IntType<u16>, FlagType<u32>>> =
            Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 1);
        let sol = res.solution.get_solution();
        assert!(match sol {
            Solution::One(v) => {
                assert!(v
                    .grid_iter()
                    .zip(expected.iter())
                    .all(|(v, e)| v.exportv() == *e));
                true
            }
            _ => false,
        });
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

        let res: Puzzle<SimpleSquare<IntType<u8>>> = Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 2);
        match res.solution.get_solution() {
            Solution::Multi(vec) => {
                if vec[0]
                    .grid_iter()
                    .zip(expected1.iter())
                    .all(|(a, e)| a.getv() == *e)
                {
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.getv() == *e));
                } else {
                    assert!(vec[0]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.getv() == *e));
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected1.iter())
                        .all(|(a, e)| a.getv() == *e));
                }
            }
            _ => assert!(false),
        }
        let guesses1: Vec<usize> = res
            .solution
            .data
            .iter()
            .filter(|i| match i {
                SolveTech::Guesses(_) => true,
                _ => false,
            })
            .map(|e| match e {
                SolveTech::Guesses(v) => *v,
                _ => 0,
            })
            .collect();
        assert_eq!(guesses1.len(), 1);
        let guesses1 = guesses1[0];

        let guess_n_check = if let SolveTech::Guesses(n) =
            res.solution.get_tech(SolveTech::Guesses).next().unwrap()
        {
            n
        } else {
            &0
        };
        assert_eq!(guesses1, *guess_n_check);

        let res: Puzzle<FlagSquare<IntType<u8>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 2);
        match res.solution.get_solution() {
            Solution::Multi(vec) => {
                if vec[0]
                    .grid_iter()
                    .zip(expected1.iter())
                    .all(|(a, e)| a.getv() == *e)
                {
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.getv() == *e));
                } else {
                    assert!(vec[0]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.getv() == *e));
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected1.iter())
                        .all(|(a, e)| a.getv() == *e));
                }
            }
            _ => assert!(false),
        }

        let res: Puzzle<FlagSquare<FlagType<u16>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 2);
        match res.solution.get_solution() {
            Solution::Multi(vec) => {
                if vec[0]
                    .grid_iter()
                    .zip(expected1.iter())
                    .all(|(a, e)| a.exportv() == *e)
                {
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.exportv() == *e));
                } else {
                    assert!(vec[0]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.exportv() == *e));
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected1.iter())
                        .all(|(a, e)| a.exportv() == *e));
                }
            }
            _ => assert!(false),
        }

        let res: Puzzle<SimpleSquare<FlagType<u16>>> = Puzzle::new(example.as_input().unwrap());
        assert_eq!(res.solution.num_solutions(), 2);
        match res.solution.get_solution() {
            Solution::Multi(vec) => {
                if vec[0]
                    .grid_iter()
                    .zip(expected1.iter())
                    .all(|(a, e)| a.exportv() == *e)
                {
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.exportv() == *e));
                } else {
                    assert!(vec[0]
                        .grid_iter()
                        .zip(expected2.iter())
                        .all(|(a, e)| a.exportv() == *e));
                    assert!(vec[1]
                        .grid_iter()
                        .zip(expected1.iter())
                        .all(|(a, e)| a.exportv() == *e));
                }
            }
            _ => assert!(false),
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

        let res: Puzzle<FlagSquare<IntType<u16>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());

        assert!(res.solution.num_solutions() == 2);

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

        let res: Puzzle<FlagSquare<IntType<u16>, FlagType<u16>>> =
            Puzzle::new(example.as_input().unwrap());

        // used https://www.thonky.com/sudoku/solution-count to verify solution count
        assert!(res.solution.num_solutions() == 192);
    }
}
