use crate::grid::{Grid, MAX_NUM, NUM_CELLS};
use crate::solve::solution_report::SolveTech;
use crate::sq_element::sq_element::{FlElement, SqElement};
use crate::square::flag_update::FlagUpdate;
use crate::square::{FlagSquare, Square};
use crate::support::{get_cell, index_from_box, index_from_col, index_from_row, start_of_box};
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

pub trait BasicHumanMethods {
    /// Finds cells that have only one possible value, fills it in, and removes pencil marks for
    /// related cells.  A single candidate is where only one value is possible in that cell.
    fn single_candidate(&mut self) -> SolveTech;

    /// Fills in single possibilities, returns vector of what positions were filled.
    /// A single possibility is where only one cell in a row/column/box has the possibility of a value.
    fn single_possibility(&mut self) -> SolveTech;
    fn single_possibility_slower(&mut self) -> SolveTech;

    /// A naked pair is where two cells have the same potential values, and no others.
    /// For example, two cells have a possibility of either `2`, or `4`. This means that `2` belongs to one of these
    /// cells, and `4` to the other.  These possiblities can be eliminated in other cells in the associated row, column, or box
    /// These can be in the form of pairs, triples, or more.  It will only be evaluated from pairs to quadruples (for a 9 cell game), since
    /// checking for quintuples is the same as quadruples.  Generically, it will be `n/2` with `n` being the MAX_NUM value.
    /// This method does not fill in cells, only eliminates possibilities which can support other methods like
    /// single_candidate or single_possibility.
    fn naked_tuple(&mut self) -> SolveTech;

    /// A hidden pair is similar to a naked pair, however it contains other possibilities.
    /// For example, a cell has the possibility of either `2`, `4`, or '7'. Another cell has a possiblity of '2', '4', '6'.  The other cells in the row
    /// do not have the possiblity of '2' or '4'.  Therefore, it can be similified that the first two cells are only '2' or '4', and the remaining
    /// possibilities in these cells can be removed, along with '2' and '4' in the remaining cells.
    /// This method does not fill in cells, only eliminates possibilities which can support other methods like
    /// single_candidate or single_possibility.
    fn hidden_tuple(&mut self) -> SolveTech;
    // fn locked_candidates_pointing(&mut self, fill: bool) -> Vec<(usize, Element)> ;
    // fn locked_candidates_claiming(&mut self, fill: bool) -> Vec<(usize, Element)>;
}

impl<
        V: SqElement + From<F>,
        F: FlElement
            + From<V>
            + BitAnd<Output = F>
            + BitOr<Output = F>
            + Sub<Output = F>
            + BitXor<Output = F>,
    > BasicHumanMethods for Grid<FlagSquare<V, F>>
where
    FlagSquare<V, F>: FlagUpdate<FlagElement = F>,
    F: Debug,
{
    // O(n) where n is the number of cells
    fn single_candidate(&mut self) -> SolveTech {
        let mut amount: usize = 0;

        /* Solving one single candidate may make other single candidates 'earlier' in the grid.
         * Loop until no new singles are found. */
        loop {
            // get list of single candidate cells
            let singles: Vec<(usize, F)> = self
                .grid_iter()
                .enumerate()
                .filter(|(_, s)| s.count == 1)
                .map(|(i, s)| (i, s.flags))
                .collect();

            // Exit loop if no new singles are found.
            if singles.len() == 0 {
                break;
            }
            amount += singles.len();

            // set and update affected flags
            for (i, v) in singles {
                self.set_value_update_flags(i, v);
            }
        }

        SolveTech::SingleCandidates(amount)
    }
    fn single_possibility(&mut self) -> SolveTech {
        /**
         * Ones: Bitwise OR, starting from all 0.  If it has been used at least once, it be 1,
         * Multis: Selfassign bitwise or with (Ones(n-1) bitwiseAND current)
         * Ones:  0000
         * Multi: 0000
         * S1  :  0100
         * Ones:  0100
         * Multi: 0000
         * S2:    1010
         * Ones:  1110
         * Multi: 0000
         * S3:    1100
         * Ones:  1110
         * Multi: 1100
         *
         * Single possiblities will be 1's in Ones, but not in Multi
         **/
        let mut cands: usize = 0;
        loop {
            let initial = cands;
            for i in 0..MAX_NUM {
                let row_start = index_from_row(i, 0);
                let col_start = index_from_col(i, 0);
                let box_start = index_from_box(i, 0);

                /// This function goes through each row, col, and box and collects tuples
                /// in the form of relative index (to the start of the row/col/box) amd value.
                /// Grid will be updated in another function.
                fn get_singles<
                    'a,
                    'b,
                    VT: 'b + SqElement + From<FT>,
                    FT: 'b
                        + FlElement
                        + From<VT>
                        + BitXor<Output = FT>
                        + BitAnd<Output = FT>
                        + BitOr<Output = FT>,
                    I: Iterator<Item = &'b FlagSquare<VT, FT>>,
                >(
                    grid: &'a mut Grid<FlagSquare<VT, FT>>,
                    iter: fn(&'a Grid<FlagSquare<VT, FT>>, usize) -> I,
                    //impl Iterator<Item=&'a FlagSquare<VT, FT>>,
                    index_from: fn(usize, usize) -> usize,
                    step: usize,
                ) -> Vec<(usize, FT)>
                where
                    FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
                {
                    /* `ones` contains all flags used at least once
                     *  `multi` contains all flags used more than once. How many times does
                     *  not matter in this situation. */
                    let (multi, ones) = iter(grid, index_from(step, 0)).fold(
                        (FT::zero(), FT::zero()),
                        |(mut mul, mut ones), s| {
                            mul = mul | ones & s.flags.clone();
                            ones = ones | s.flags.clone();
                            (mul, ones)
                        },
                    );

                    // Singles are the difference in ones and multi
                    let singles = ones ^ multi;
                    // Find the singles if any
                    let mut pairs: Vec<(usize, FT)> = Vec::new();

                    // No need to waste time iterating through the puzzle again if there are no singles.
                    if singles > FT::zero() {
                        /* Iterate throught the rol/col/box to find what is a single possibility.
                         * Since `singles` has flags that are only used once (even if there are multiple flags in `singles`
                         * when the squares flag is bitwise AND with `singles`, there will only be
                         * zero or 1 flag remaining, since there cannot be a square that has two single
                         * possibilities
                         */
                        pairs = iter(grid, index_from(step, 0))
                            .enumerate()
                            .map(|(x, s)| (x, s.flags & singles))
                            .filter(|(_, v)| v > &FT::zero())
                            .collect();
                    }

                    pairs
                }

                /// This function is separate from the above to prevent immutable borrow at
                /// the same time as mutable borrow.
                /// This function takes the pairs and updates the grid
                fn update_grid<VT: SqElement + From<FT>, FT: FlElement + From<VT>>(
                    grid: &mut Grid<FlagSquare<VT, FT>>,
                    pairs: &Vec<(usize, FT)>,
                    index_from: fn(usize, usize) -> usize,
                    step: usize,
                ) -> usize
                where
                    FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
                {
                    for p in pairs {
                        assert_eq!(FT::count_ones(&p.1), 1);
                        &grid.set_value_update_flags(index_from(step, p.0), p.1);
                    }
                    pairs.len()
                }

                /* Here is where the solving takes place */
                // Row
                let pairs = get_singles(self, Self::row_iter, index_from_row, i);
                cands += update_grid(self, &pairs, index_from_row, i);
                //Col
                let pairs = get_singles(self, Self::col_iter, index_from_col, i);
                cands += update_grid(self, &pairs, index_from_col, i);
                // Box
                let pairs = get_singles(self, Self::box_iter, index_from_box, i);
                cands += update_grid(self, &pairs, index_from_box, i);
            }
            if cands == initial {
                break;
            }
        }
        SolveTech::SinglePossibilities(cands)
    }
    fn naked_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }

    fn hidden_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }

    fn single_possibility_slower(&mut self) -> SolveTech {
        unimplemented!()
    }
}
//     let mut adds: Vec<(usize, Element)> = Vec::new();
//
//     let ind_in_row = |row, col| get_cell(row, col);
//     let ind_in_col = |col, row| get_cell(row, col);
//     let ind_in_box = |el_box, ind| index_from_box(el_box, ind);
//
//     let mut end_adds = adds.len();
//     loop {
//         let count = adds.len();
//         for iteration in 0..MAX_NUM {
//             adds.append(&mut find_single(self.row_iter(index_from_row(iteration, 0)), iteration, ind_in_row));
//             adds.append(&mut find_single(self.col_iter(index_from_col(iteration, 0)), iteration, ind_in_col));
//             adds.append(&mut find_single(self.box_iter(start_of_box(iteration)), iteration, ind_in_box));
//
//             for (ind, val) in adds.iter().skip(end_adds) {
//                 if fill {
//                     self.cells[*ind].set(*val);
//                     self.cells[*ind].fixed = true;
//                     update_pencil_after_add(self, *ind);
//                 }
//             }
//             end_adds = adds.len();
//         }
//         if count == adds.len() {
//             break;
//         }
//     }
//
//
//     fn find_single<'a, F, I>(iter: I, iteration: usize, func: F) -> Vec<(usize, Element)>
//         where F: Fn(usize, usize) -> usize, I: Iterator<Item=&'a Cell>
//     {
//         let the_cells = iter.collect::<Vec<&Cell>>();
//         let mut section_add: Vec<(usize, Element)> = Vec::new();
//         for i in 0..MAX_NUM {
//             if the_cells[i].fixed() {
//                 continue;
//             }
//             let index = func(iteration, i);
//
//             let other_sets = the_cells.iter()
//                 .enumerate()
//                 .filter(|(step, _)| *step != i)
//                 .fold(BTreeSet::new(), |set, (_, s)| set.union(s.penciled()).cloned().collect());
//             let values = the_cells[i].penciled().difference(&other_sets).cloned().collect::<Vec<Element>>();
//             // If values is greater than 1 element, it would mean the puzzle is invalid due to two unique numbers in one spot.
//             assert!(values.len() <= 1, "values: {:?} \nother_sets: {:?}\ncell: {:?}\nIndex: {}\nIteration: {}", values, other_sets, the_cells[index].penciled(), index, iteration);
//             if values.len() > 0 {
//                 section_add.push((index, values[0]));
//             }
//         }
//         section_add
//     }
//
//     adds
// }

#[cfg(test)]
mod human_method_tests {
    use super::*;
    use crate::conv_input_output::{PuzInput, PuzOutput};
    use crate::puzzle::{Puzzle, PuzzleTrait};
    use crate::solve::brute::BruteForce;
    use crate::solve::solution_report::Solution;
    use crate::sq_element::{Flag, IntValue};

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

    fn get_puzzle() -> Puzzle<FlagSquare<IntValue, Flag<u16>>> {
        Puzzle::new(get_example().as_input().unwrap())
    }

    #[test]
    fn remove_flag_test() {
        let mut puz = get_puzzle().board;
        let cell1 = puz[2];
        let cell2 = puz[10];
        let cell3 = puz[17];
        let cell4 = puz[73];
        let cells = [cell1, cell2, cell3, cell4];
        for c in cells.iter() {
            assert!(c.flags.is_flagged(&Flag::from(0b10)));
        }
        puz.set_value_update_flags(10, IntValue::from(2));

        let cell1 = puz[2];
        let cell2 = puz[10];
        let cell3 = puz[17];
        let cell4 = puz[73];
        let cells = [cell1, cell2, cell3, cell4];

        for c in cells.iter() {
            assert!(!c.flags.is_flagged(&Flag::from(0b10)));
        }
    }

    #[test]
    fn single_cand() {
        let inv = vec![
            vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![0, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        let mut puz =
            <Puzzle<FlagSquare<Flag<u16>, Flag<u16>>>>::new(inv.as_input().unwrap()).board;

        let expected: Vec<Vec<u8>> = vec![
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

        if let SolveTech::SingleCandidates(a) = puz.single_candidate() {
            assert_eq!(a, 50);
        } else {
            assert!(false);
        }

        // This puzzle is solved by 100% single candidates.
        for (act, exp) in puz.grid_iter().zip(expected.iter().flatten()) {
            assert_eq!(act.exportv(), *exp);
        }
    }

    #[test]
    fn single_possibility_test() {
        let mut puz: Puzzle<FlagSquare<IntValue, Flag<u16>>> = Puzzle::new(
            vec![
                vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
                vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                vec![0, 1, 0, 0, 2, 0, 0, 0, 6],
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
            ]
            .as_input()
            .unwrap(),
        );

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

        if let SolveTech::SinglePossibilities(res) = puz.board.single_possibility() {
            //dbg!(&res);
            assert!(res > 4);
            dbg!(&res);
        } else {
            assert!(false);
        }

        // assert!(res.contains(&(get_cell(2, 6), 5)));
        // assert!(res.contains(&(get_cell(5, 6), 8)));
        // assert!(res.contains(&(get_cell(6, 2), 1)));
        // assert!(res.contains(&(get_cell(3, 5), 1)));

        /* This puzzle does not get completely solved using this method.
         * The remaining portion of the puzzle with be solve forced solved to ensure that the solving is correct
         */
        let finished = puz.brute_force_solve();
        let guesses = finished
            .tech_iter(SolveTech::Guesses)
            .map(|t| t.clone())
            .collect::<Vec<SolveTech>>();
        assert_eq!(guesses.len(), 1);
        if let SolveTech::Guesses(g) = guesses[0] {
            // dbg!(&g);
            if g > 0 {
                assert!(g > 0);
            } else {
                if let Solution::One(s) = &finished.sol {
                    // dbg!(&s);
                    assert!(s
                        .grid_iter()
                        .zip(puz.board.grid_iter())
                        .all(|(s, e)| s.exportv() == e.exportv()));
                } else {
                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
        if let Solution::One(s) = finished.sol {
            // dbg!(&s);
            assert!(s
                .grid_iter()
                .zip(expected.iter())
                .all(|(s, e)| s.exportv() == *e));
        } else {
            assert!(false);
        }
    }
}
