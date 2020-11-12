use crate::grid::{Grid, MAX_NUM};
use crate::human_calcs::tuple_ctr::*;
use crate::solve::solution_report::SolveTech;
use crate::sq_element::sq_element::{FlElement, SqElement};
use crate::square::flag_update::FlagUpdate;
use crate::square::{FlagSquare, Square};
use crate::support::{index_from_box, index_from_col, index_from_row};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr, BitXor, Shr, Sub};

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
    fn naked_tuple(&mut self) -> Vec<SolveTech>;

    /// A hidden pair is similar to a naked pair, however it contains other possibilities.
    /// For example, a cell has the possibility of either `2`, `4`, or '7'. Another cell has a possiblity of '2', '4', '6'.  The other cells in the row
    /// do not have the possiblity of '2' or '4'.  Therefore, it can be similified that the first two cells are only '2' or '4', and the remaining
    /// possibilities in these cells can be removed, along with '2' and '4' in the remaining cells.
    /// This method does not fill in cells, only eliminates possibilities which can support other methods like
    /// single_candidate or single_possibility.
    fn hidden_tuple(&mut self) -> Vec<SolveTech>;
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
            + BitXor<Output = F>
            + Ord
            + Shr<Output = F>,
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
        /*
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

    /// Naked tuples are squares in a rol/col/box that have the same identical possibilities.
    /// For example, square#1 can be either a 3 or 4.  Another square in that row can only be
    /// a 3 or 4.  This means that all other squares in that row CANNOT be 3 or 4.  It doesn't
    /// solve the square, but it eliminates possibilities in other squares that then can be solved
    /// easier with other techniques.
    fn naked_tuple<'a>(&'a mut self) -> Vec<SolveTech> {
        fn get_tuples<
            'a,
            VT: SqElement + From<FT>,
            FT: FlElement + From<VT> + Ord,
            I: Iterator<Item = &'a FlagSquare<VT, FT>>,
        >(
            grid: &'a mut Grid<FlagSquare<VT, FT>>,
            iter: fn(&'a Grid<FlagSquare<VT, FT>>, usize) -> I,
            index_from: fn(usize, usize) -> usize,
            step: usize,
        ) -> BTreeMap<FT, usize>
        where
            FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
        {
            // This map is used to track the amount of times a combo comes up.
            // The square's flags are the key, the amount of times is the value.
            let mut map: BTreeMap<FT, usize> = BTreeMap::new();
            for s in iter(grid, index_from(step, 0)) {
                // map.insert returns None if nothing existed before, else returns previous value.
                // This will take the old value, if present, and increment it
                if let Some(v) = map.insert(s.flags, 1) {
                    map.insert(s.flags, v + 1);
                }
            }
            map
        }

        fn update_grid<
            'a,
            VT: SqElement + From<FT>,
            FT: FlElement + From<VT>,
            I: Iterator<Item = &'a mut FlagSquare<VT, FT>>,
        >(
            grid: &mut Grid<FlagSquare<VT, FT>>,
            tuples: &BTreeMap<FT, usize>,
            iter: fn(&'a mut Grid<FlagSquare<VT, FT>>, usize) -> I,
            index_from: fn(usize, usize) -> usize,
            step: usize,
        ) -> Vec<SolveTech>
        where
            FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
        {
            let mut results = Vec::new();
            // //keep tuples that have 2 or more matches.
            // let good_tuples: BTreeMap<FT, usize> = tuples.iter().filter(|t| t.1 > &1).collect();
            // // Early out
            // if good_tuples.len() == 0 {
            //     return results;
            // }
            // for mut s in iter(grid, index_from(step, 0)){
            //     if !good_tuples.contains(s.flags) {
            //         for t in good_tuples {
            //             s.flags = s.flags - t.0;
            //         }
            //     }
            // }
            // // Handle the results
            // let t_count: Vec<usize> = good_tuples.iter().map(|t| t.1).collect();
            // for v in t_count {
            //     results.push(SolveTech::NakedTuple(v))
            // }
            results
        }

        let mut tuples: Vec<SolveTech> = Vec::new();
        for i in 0..MAX_NUM {
            let mut tuple_map: BTreeMap<F, usize> =
                get_tuples(self, Self::row_iter, index_from_row, i);
            tuples.append(&mut update_grid(
                self,
                &tuple_map,
                Self::row_iter_mut,
                index_from_row,
                i,
            ));
        }
        tuples
    }

    fn hidden_tuple<'a>(&'a mut self) -> Vec<SolveTech> {
        fn get_tuples<
            'a,
            'b,
            VT: 'b + SqElement + From<FT>,
            FT: 'b + FlElement + From<VT> + Ord + Shr<Output = FT>,
            I: Iterator<Item = &'b FlagSquare<VT, FT>>,
        >(
            grid: &'a Grid<FlagSquare<VT, FT>>,
            iter: fn(&'a Grid<FlagSquare<VT, FT>>, usize) -> I,
            index_from: fn(usize, usize) -> usize,
            step: usize,
        ) -> Vec<Ctr<FT>>
        where
            FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
        {

            // Collect the counts for occurances of each flagged value
            let mut v_indicies: Vec<(usize, usize, FT)> = Vec::new();
            let mut counter: TupleCtr<FT> = TupleCtr::new();
            for (i, &s) in iter(grid, index_from(step, 0)).enumerate() {
                counter.insert(FT::from(i + 1), s.flags);
            }

            let mut results: Vec<Ctr<FT>> = Vec::new();
            for uple in 1..(MAX_NUM / 2) {
                 results.append(&mut counter.combo(u8::try_from(uple).unwrap()));
                // // Unravel results:
                // for r in results {
                //     assert_eq!(FT::count_ones(&r.flag), FT::count_ones(&r.indicies));
                //     let mut ind = r.indicies;
                //     let mut count = 0;
                //     let one = FT::one();
                //     while ind > FT::zero() {
                //         if ind & one == one {
                //             v_indicies.push((uple, count, r.flag));
                //         }
                //         ind = ind >> one;
                //         count += 1;
                //     }
                // }
            }
            results
        }

        fn update_grid<'a, 'b, VT: 'b + SqElement + From<FT>, FT: 'b + FlElement + From<VT> + Ord, I: Iterator<Item = &'b mut FlagSquare<VT, FT>>,>(
            grid: &'a mut Grid<FlagSquare<VT, FT>>,
            indicies: &Vec<Ctr<FT>>,
            iter: fn(&'a mut Grid<FlagSquare<VT, FT>>, usize) -> I,
            index_from: fn(usize, usize) -> usize,
            step: usize,
        ) -> Vec<SolveTech>
        where
        FlagSquare<VT, FT>: FlagUpdate<FlagElement = FT>,
        {
            let zero = FT::zero();
            let mut tups: Vec<SolveTech> = Vec::new();
            let mut indicies = indicies;
            for (i, s) in iter(grid, index_from(step, 0)).enumerate().filter(|(_,s)|s.count > 0) {
                let ind = FT::from(i+1);
                for c in indicies {
                    /* If a square's index is in the indicies value for a Ctr, then it means
                    * that it belongs in the tuple.  All other potential values for that square
                    * can be eliminated.  If the index is NOT in the Ctr, then all values of the tuple
                    * can be eliminated.
                     */
                    if ind & c.indicies > zero {
                        s.flags = s.flags & c.flag.clone();
                        tups.push(SolveTech::HiddenTuples((c.ind_count as usize, index_from(step, i))));
                    } else {
                        s.flags = s.flags - c.flag;
                    }
                }
            }
            tups
        }

        let mut ret: Vec<SolveTech> = Vec::new();
        for i in 0..MAX_NUM {
            ret.append(&mut update_grid(
                self,
                &get_tuples(self, Self::row_iter, index_from_row, i),
                Self::row_iter_mut,
                index_from_row,
                i,
            ));
            ret.append(&mut update_grid(
                self,
                &get_tuples(self, Self::col_iter, index_from_col, i),
                Self::col_iter_mut,
                index_from_col,
                i,
            ));
            ret.append(&mut update_grid(
                self,
                &get_tuples(self, Self::box_iter, index_from_box, i),
                Self::box_iter_mut,
                index_from_box,
                i,
            ));
        }

        ret
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
    use crate::conv_input_output::PuzInput;
    use crate::puzzle::{Puzzle, PuzzleTrait};
    use crate::solve::brute::BruteForce;
    use crate::solve::solution_report::Solution;
    use crate::sq_element::{Flag, IntValue};
    use crate::sq_element::sq_element::OneZero;

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

    #[test]
    fn hidden_tuples_test() {
        let mut puz: Puzzle <FlagSquare<Flag<u16>, Flag<u16>>> = Puzzle::new(
            vec![
                vec![7, 0, 0, 8, 4, 9, 0, 3, 0],
                vec![9, 2, 8, 1, 3, 5, 0, 0, 6],
                vec![4, 0, 0, 2, 6, 7, 0, 8, 9],
                vec![6, 4, 2, 7, 8, 3, 9, 5, 1],
                vec![3, 9, 7, 4, 5, 1, 6, 2, 8],
                vec![8, 1, 5, 6, 9, 2, 3, 0, 0],
                vec![2, 0, 4, 5, 1, 6, 0, 9, 3],
                vec![1, 0, 0, 0, 0, 8, 0, 6, 0],
                vec![5, 0, 0, 0, 0, 4, 0, 1, 0],
            ]
            .as_input()
            .unwrap());

        let i1 = index_from_row(7, 2);
        let i2 = index_from_row(7, 3);
        let i3 = index_from_row(1, 6);
        let i4 = index_from_row(1, 7);

        let f1 = puz.board[i1].flags;
        let f2 = puz.board[i2].flags;
        let f3 = puz.board[i3].flags;
        let f4 = puz.board[i4].flags;

        assert_eq!(f1, <Flag<u16>>::from(3).merge(&[<Flag<u16>>::from(9)]));
        assert_eq!(f2, <Flag<u16>>::from(3).merge(&[<Flag<u16>>::from(9)]));
        // check that not identical
        // assert_ne!(f1, f2);
        // assert_eq!(f3, f4);

        let res = puz.board.hidden_tuple();

        let f1t = puz.board[i1].flags;
        let f2t = puz.board[i2].flags;
        let f3t = puz.board[i3].flags;
        let f4t = puz.board[i4].flags;

        // Identical as they are tuples and extra should have been removed
        dbg!(&res);
        assert_eq!(f1t, f2t);
        assert_eq!(f3t, f4t);

        let zero = Flag::zero();
        // Check nothing has been added
        assert_eq!(f1t - f1, zero);
        assert_eq!(f2t - f2, zero);
        assert_eq!(f3t - f3, zero);
        assert_eq!(f4t - f4, zero);


        assert!(res.contains(&SolveTech::HiddenTuples((2, i1))));
        assert!(res.contains(&SolveTech::HiddenTuples((2, i2))));
        assert!(res.contains(&SolveTech::HiddenTuples((2, i3))));
        assert!(res.contains(&SolveTech::HiddenTuples((2, i4))));

        // //ensure puzzle is solvable after pencil mark eliminations. This puzzle is solvable with
        // // single candidate which uses the pencil marks
        // let count = puz.single_candidate(true);
        // assert_eq!(count.len(), 25);
        //
        // // Quick validity check over all filled in blocks.
        // let res = puz.brute_force_solve();
        // assert_eq!(res.len(), 1);
        //
        // /*
        // Naked triple check
        //  */
        // let example = vec![
        //     vec![0, 0, 0, 2, 9, 4, 3, 8, 0],
        //     vec![0, 0, 0, 1, 7, 8, 6, 4, 0],
        //     vec![4, 8, 0, 3, 5, 6, 1, 0, 0],
        //     vec![0, 0, 4, 8, 3, 7, 5, 0, 1],
        //     vec![0, 0, 0, 4, 1, 5, 7, 0, 0],
        //     vec![5, 0, 0, 6, 2, 9, 8, 3, 4],
        //     vec![9, 5, 3, 7, 8, 2, 4, 1, 6],
        //     vec![1, 2, 6, 5, 4, 3, 9, 7, 8],
        //     vec![0, 4, 0, 9, 6, 1, 2, 5, 3],
        // ];
        //
        // let mut puz = Puzzle::new();
        // puz.set_initial(example.as_input().unwrap());
        //
        // let res = puz.naked_tuple();
        // assert!(res.contains(&get_cell(1, 1)));
        // assert!(res.contains(&get_cell(3, 1)));
        // assert!(res.contains(&get_cell(4, 1)));
        // assert!(!puz.cells[1].penciled.contains(&6));
        //
        // assert!(res.contains(&get_cell(3, 7)));
        // assert!(res.contains(&get_cell(4, 7)));
        // assert!(res.contains(&get_cell(4, 8)));
        //
        //
        // // Quick validity check over all filled in blocks.
        // let res = puz.brute_force_solve();
        // assert_eq!(res.len(), 1);
        //
        // let example = vec![
        //     vec![3, 9, 0, 0, 0, 0, 7, 0, 0],
        //     vec![0, 0, 0, 0, 0, 0, 6, 5, 0],
        //     vec![5, 0, 7, 0, 0, 0, 3, 4, 9],
        //     vec![0, 4, 9, 3, 8, 0, 5, 0, 6],
        //     vec![6, 0, 1, 0, 5, 4, 9, 8, 3],
        //     vec![8, 5, 3, 0, 0, 0, 4, 0, 0],
        //     vec![9, 0, 0, 8, 0, 0, 1, 3, 4],
        //     vec![0, 0, 2, 9, 4, 0, 8, 6, 5],
        //     vec![4, 0, 0, 0, 0, 0, 2, 9, 7],
        // ];
        //
        // let mut puz = Puzzle::new();
        // puz.set_initial(example.as_input().unwrap());
        // // There are in fact no single candidates.  Using pairs will free some up and be able to
        // // be solved later.
        // let uns = puz.single_candidate(true);
        // assert_eq!(uns.len(), 0);
        //
        // let res = puz.naked_tuple();
        //
        // assert!(res.contains(&get_cell(0, 4)));
        // assert!(res.contains(&get_cell(2, 3)));
        // assert!(res.contains(&get_cell(2, 4)));
        //
        // assert!(puz.cells[get_cell(2, 3)].penciled.contains(&1));
        // assert!(!puz.cells[get_cell(1, 4)].penciled.contains(&1));
        //
        // // Puzzle is solvable with single_candidate now that some pencil marks were eliminated by tuples.
        // let count = puz.single_candidate(true);
        // assert!(count.len() == 39); // 39 empty cells
        //
        // // Quick validity check over all filled in blocks.
        // let res = puz.brute_force_solve();
        // assert_eq!(res.len(), 1);
        //
        // // The following test for hidden triples
        // let str = "5..62..37..489........5....93........2....6.57.......3.....9............68.57...2";
        // let mut puz = Puzzle::new();
        // puz.set_initial(str.as_input().unwrap());
        // let res = puz.naked_tuple();
        // let threefive = get_cell (3,5);
        // assert!(res.contains(&threefive));
        // assert!(puz.cells[threefive].penciled().contains(&5));
        // assert!(puz.cells[threefive].penciled().contains(&2));
        // assert!(puz.cells[threefive].penciled().contains(&6));
        // assert!(!puz.cells[threefive].penciled().contains(&1));


    }
}
