use crate::solve::solution_report::SolveTech;
use crate::square::{FlagSquare};
use crate::grid::{NUM_CELLS, Grid};
use crate::sq_element::{FlElement, SqElement};

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

impl <V: SqElement, F: FlElement>BasicHumanMethods for Grid<FlagSquare<V, F>> {

    // O(n) where n is the number of cells
    fn single_candidate(&mut self) -> SolveTech {
        // This vector contains coordinate and value for cells that are solved for.  This can be used to debug
        // or function modified to return this vector for display.
        let mut amount: usize = 0;

        loop {
            let initial_amount = amount;


            // get list of single candidate cells
            let singles: Vec<usize> = self.grid_iter().enumerate().filter(|(_, s)| s.count == 1)
                .map(|(i,_)| i).collect();

        }
            let single_cands = (0..NUM_CELLS)
                .into_iter()
                .filter(|i| self.single_iterator(*i).count() == 1)
                .collect::<Vec<usize>>();

            for i in single_cands {

                let val: Element = *self.cells[i].penciled().iter().next().expect(format!("index: {}", i).as_str());
                // Mark the cell that has one candidate
                if fill {
                    self.cells[i].set(val);
                    self.cells[i].fixed = true;
                    update_pencil_after_add(self, i);
                }
                // Update others that this value is now no longer a candidate
                amount.push((i, val));
            }
            // Loop to see if finding single candidates clears the way for new single candidates
            // breaks if no single candidates were found.
            if initial_amount == amount.len() {
                break;
            }
        }
        amount
    }

    fn single_possibility(&mut self, fill: bool) -> Vec<(usize, _)> {
        unimplemented!()
    }

    fn single_possibility_slower(&mut self, fill: bool) -> Vec<(usize, Element)> {

        let mut adds: Vec<(usize, Element)> = Vec::new();

        let ind_in_row = |row, col| get_cell(row, col);
        let ind_in_col = |col, row| get_cell(row, col);
        let ind_in_box = |el_box, ind| index_from_box(el_box, ind);

        let mut end_adds = adds.len();
        loop {
            let count = adds.len();
            for iteration in 0..MAX_NUM {

                 adds.append(&mut find_single (self.row_iter(index_from_row(iteration, 0)), iteration, ind_in_row));
                 adds.append(&mut find_single (self.col_iter(index_from_col(iteration, 0)), iteration, ind_in_col));
                 adds.append(&mut find_single (self.box_iter(start_of_box(iteration)), iteration, ind_in_box));

                for (ind, val) in adds.iter().skip(end_adds) {
                    if fill {
                        self.cells[*ind].set(*val);
                        self.cells[*ind].fixed = true;
                        update_pencil_after_add(self, *ind);
                    }
                }
                end_adds = adds.len();

            }
            if count == adds.len(){
                break;
            }
        }


        fn find_single <'a, F, I>(iter: I, iteration: usize, func: F) -> Vec<(usize, Element)>
        where F: Fn(usize, usize) -> usize, I: Iterator<Item = &'a Cell>
        {

            let the_cells = iter.collect::<Vec<&Cell>>();
            let mut section_add: Vec<(usize, Element)> = Vec::new();
            for i in 0..MAX_NUM {
                if the_cells[i].fixed() {
                    continue;
                }
                let index = func(iteration, i);

                let other_sets = the_cells.iter()
                    .enumerate()
                    .filter (|(step, _)| *step != i )
                    .fold(BTreeSet::new(), |set, (_, s)| set.union(s.penciled()).cloned().collect());
                let values = the_cells[i].penciled().difference(&other_sets).cloned().collect::<Vec<Element>>();
                // If values is greater than 1 element, it would mean the puzzle is invalid due to two unique numbers in one spot.
                assert!(values.len() <= 1, "values: {:?} \nother_sets: {:?}\ncell: {:?}\nIndex: {}\nIteration: {}", values, other_sets, the_cells[index].penciled(), index, iteration);
                if values.len() > 0 {
                    section_add.push((index, values[0]));
                }

            }
            section_add
        }

        adds
    }

    fn naked_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }

    fn hidden_tuple(&mut self) -> SolveTech {
        unimplemented!()
    }