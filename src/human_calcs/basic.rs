use crate::solve::solution_report::SolveTech;
use crate::square::{FlagSquare, Square};
use crate::grid::{NUM_CELLS, Grid, MAX_NUM};
use crate::sq_element::sq_element::{SqElement, FlElement};
use crate::support::{get_cell, index_from_box, index_from_row, index_from_col, start_of_box};
use crate::square::flag_update::FlagUpdate;

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

impl <V: SqElement + From<F>, F: FlElement + From<V>>BasicHumanMethods for Grid<FlagSquare<V, F>>
    where FlagSquare<V,F>: FlagUpdate<FlagElement = F>
{
    // O(n) where n is the number of cells
    fn single_candidate(&mut self) -> SolveTech {
        let mut amount: usize = 0;

        /* Solving one single candidate may make other single candidates 'earlier' in the grid.
         * Loop until no new singles are found. */
        loop {

            // get list of single candidate cells
            let singles: Vec<(usize, F)> = self.grid_iter().enumerate().filter(|(_, s)| s.count == 1)
                .map(|(i, s)| (i, s.flags)).collect();

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

    fn naked_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }

    fn hidden_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }

    fn single_possibility(&mut self) -> SolveTech {
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
    use crate::puzzle::{Puzzle, PuzzleTrait};
    use crate::conv_input_output::PuzInput;
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
        let inv =
            vec![
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
        let mut puz = <Puzzle<FlagSquare<Flag<u16>, Flag<u16>>>>::new(inv.as_input().unwrap()).board;

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

        if let SolveTech::SingleCandidates(a) = puz.single_candidate(){
            assert_eq!(a, 50);
        } else {assert!(false);}

        // This puzzle is solved by 100% single candidates.
        for (act, exp) in puz.grid_iter().zip(expected.iter().flatten()) {
            assert_eq!(act.exportv(), *exp);
        }
    }
}