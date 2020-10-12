// use crate::*;
// use std::collections::HashMap;
// use crate::constants::*;
//
//
// pub trait BasicHumanMethods {
//     /// Finds cells that have only one possible value, fills it in, and removes pencil marks for
//     /// related cells.  A single candidate is where only one value is possible in that cell.
//     /// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
//     /// Returns a vector of tuples of (index value, number filled in).
//     fn single_candidate(&mut self, fill: bool) -> Vec<(usize, Element)>;
//
//
//     /// Fills in single possibilities, returns vector of what positions were filled.
//     /// A single possibility is where only one cell in a row/column/box has the possibility of a value.
//     /// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
//     /// Returns a vector of tuples of (index value, number filled in).
//     fn single_possibility(&mut self, fill: bool) -> Vec<(usize, Element)>;
//     fn single_possibility_slower(&mut self, fill: bool) -> Vec<(usize, Element)>;
//
//     /// A naked pair is where two cells have the same potential values, and no others.
//     /// For example, two cells have a possibility of either `2`, or `4`. This means that `2` belongs to one of these
//     /// cells, and `4` to the other.  These possiblities can be eliminated in other cells in the associated row, column, or box
//     /// These can be in the form of pairs, triples, or more.  It will only be evaluated from pairs to quadruples (for a 9 cell game), since
//     /// checking for quintuples is the same as quadruples.  Generically, it will be `n/2` with `n` being the MAX_NUM value.
//     /// This method does not fill in cells, only eliminates possibilities which can support other methods like
//     /// single_candidate or single_possibility.
//     /// Returns a set containing the indicies in which belong to a x-uple.
//     fn naked_tuple(&mut self) -> BTreeSet<usize>;
//
//     /// A hidden pair is similar to a naked pair, however it contains other possibilities.
//     /// For example, a cell has the possibility of either `2`, `4`, or '7'. Another cell has a possiblity of '2', '4', '6'.  The other cells in the row
//     /// do not have the possiblity of '2' or '4'.  Therefore, it can be similified that the first two cells are only '2' or '4', and the remaining
//     /// possibilities in these cells can be removed, along with '2' and '4' in the remaining cells.
//     /// This method does not fill in cells, only eliminates possibilities which can support other methods like
//     /// single_candidate or single_possibility.
//     /// Returns a set containing the indicies in which belong to a x-uple.
//     fn hidden_tuple(&mut self) -> BTreeSet<usize>;
//     fn locked_candidates_pointing(&mut self, fill: bool) -> Vec<(usize, Element)> ;
//     fn locked_candidates_claiming(&mut self, fill: bool) -> Vec<(usize, Element)>;
// }
//
// impl BasicHumanMethods for Puzzle {
//
//     // O(n) where n is the number of cells
//     fn single_candidate(&mut self, fill: bool) -> Vec<(usize, Element)> {
//         // This vector contains coordinate and value for cells that are solved for.  This can be used to debug
//         // or function modified to return this vector for display.
//         let mut count: Vec<(usize, Element)> = Vec::new();
//
//         loop {
//             let initial_size = count.len();
//             // Iterate through each cell, get its pencil marks, see if there is only one candidate.
//             // If only one, update it, remove associated pencil marks, update count vector
//             let single_cands = (0..NUM_CELLS)
//                 .into_iter()
//                 .filter(|i| self.cells[*i].penciled_mut().len() == 1)
//                 .collect::<Vec<usize>>();
//
//             for i in single_cands {
//
//                 let val: Element = *self.cells[i].penciled().iter().next().expect(format!("index: {}", i).as_str());
//                 // Mark the cell that has one candidate
//                 if fill {
//                     self.cells[i].set(val);
//                     self.cells[i].fixed = true;
//                     update_pencil_after_add(self, i);
//                 }
//                 // Update others that this value is now no longer a candidate
//                 count.push((i, val));
//             }
//             // Loop to see if finding single candidates clears the way for new single candidates
//             // breaks if no single candidates were found.
//             if initial_size == count.len() {
//                 break;
//             }
//         }
//         count
//     }
//
//     fn single_possibility_slower(&mut self, fill: bool) -> Vec<(usize, Element)> {
//
//         let mut adds: Vec<(usize, Element)> = Vec::new();
//
//         let ind_in_row = |row, col| get_cell(row, col);
//         let ind_in_col = |col, row| get_cell(row, col);
//         let ind_in_box = |el_box, ind| index_from_box(el_box, ind);
//
//         let mut end_adds = adds.len();
//         loop {
//             let count = adds.len();
//             for iteration in 0..MAX_NUM {
//
//                  adds.append(&mut find_single (self.row_iter(index_from_row(iteration, 0)), iteration, ind_in_row));
//                  adds.append(&mut find_single (self.col_iter(index_from_col(iteration, 0)), iteration, ind_in_col));
//                  adds.append(&mut find_single (self.box_iter(start_of_box(iteration)), iteration, ind_in_box));
//
//                 for (ind, val) in adds.iter().skip(end_adds) {
//                     if fill {
//                         self.cells[*ind].set(*val);
//                         self.cells[*ind].fixed = true;
//                         update_pencil_after_add(self, *ind);
//                     }
//                 }
//                 end_adds = adds.len();
//
//             }
//             if count == adds.len(){
//                 break;
//             }
//         }
//
//
//         fn find_single <'a, F, I>(iter: I, iteration: usize, func: F) -> Vec<(usize, Element)>
//         where F: Fn(usize, usize) -> usize, I: Iterator<Item = &'a Cell>
//         {
//
//             let the_cells = iter.collect::<Vec<&Cell>>();
//             let mut section_add: Vec<(usize, Element)> = Vec::new();
//             for i in 0..MAX_NUM {
//                 if the_cells[i].fixed() {
//                     continue;
//                 }
//                 let index = func(iteration, i);
//
//                 let other_sets = the_cells.iter()
//                     .enumerate()
//                     .filter (|(step, _)| *step != i )
//                     .fold(BTreeSet::new(), |set, (_, s)| set.union(s.penciled()).cloned().collect());
//                 let values = the_cells[i].penciled().difference(&other_sets).cloned().collect::<Vec<Element>>();
//                 // If values is greater than 1 element, it would mean the puzzle is invalid due to two unique numbers in one spot.
//                 assert!(values.len() <= 1, "values: {:?} \nother_sets: {:?}\ncell: {:?}\nIndex: {}\nIteration: {}", values, other_sets, the_cells[index].penciled(), index, iteration);
//                 if values.len() > 0 {
//                     section_add.push((index, values[0]));
//                 }
//
//             }
//             section_add
//         }
//
//         adds
//     }
//
//     // If the only possiblities in a row or column are in the same block, these values can be removed within the block
//     fn locked_candidates_claiming(&mut self, fill: bool) -> Vec<(usize, Element)> {
//         let mut cand: Vec<(usize, Element)> = Vec::new();
//         // get the set of possiblities in a row not i
//         for box_number in 0..MAX_NUM {
//             let start_ind = start_of_box(box_number);
//             for row in 0..BOX_DIMEN {
//                 let amount_to_skip = (box_number % BOX_DIMEN) * BOX_DIMEN;
//                 let start_row_index = start_ind + row * MAX_NUM;
//                 //let start_col_index = index_to_col(start_ind);
//
//                 // Possible values in the row, but not the cells in that row in the box in question
//                 let row_set = self
//                     .row_iter(start_row_index)
//                     .enumerate()
//                     .filter (|(i, _)| *i < amount_to_skip || *i >= amount_to_skip + BOX_DIMEN)
//                     .fold(BTreeSet::new(), |sets, (_, c)| sets.union(c.penciled()).cloned().collect::<BTreeSet<Element>>());
//
//                 // Possible values of the cells in the row, in the box in question
//                 let row_in_box_set = self
//                     .row_iter(start_row_index)
//                     .skip(amount_to_skip)
//                     .enumerate()
//                     .take_while(|(i, _)| *i < BOX_DIMEN)
//                     .fold(BTreeSet::new(), |sets, (_, c)| sets.union(c.penciled()).cloned().collect::<BTreeSet<Element>>());
//
//                 // Difference between the possible values of the cells in the row in the box compared to the possible values
//                 // in the row, outside the box.  Any values are locked candidates
//                 let locked_cands = row_in_box_set.difference(&row_set).cloned().collect::<BTreeSet<Element>>();
//
//                 //dbg!(&box_number, &row, &row_set, &row_in_box_set, &locked_cands);
//                 let candidates = self.box_iter(box_number)
//                 .skip(amount_to_skip)
//                 .enumerate()
//                 .take_while(|(i, _)|*i < BOX_DIMEN)
//                     .map (|(i, c)| (i + amount_to_skip, c))
//                 .collect::<Vec<(usize, &Cell)>>();
//
//                // dbg!(&candidates.iter().map(|(i, _)| i).cloned().collect::<Vec<usize>>());
//                 // Update the candidate list
//                 for (ind, cell) in candidates {
//                     for value in  cell.penciled().intersection(&locked_cands) {
//                         cand.push((start_row_index + ind, *value));
//                     }
//                 }
//                 //dbg!( &cand);
//                 let affected_cells = self.box_iter_mut(box_number)
//                     .enumerate()
//                     .filter (|(i, _)| *i < row * BOX_DIMEN || *i >= row * BOX_DIMEN + BOX_DIMEN)
//                     .map (|(_, c)| c)
//                     .collect::<Vec< &mut Cell>>();
//
//                 // Clear possibilities from affected cells.
//                 for cell in affected_cells {
//                     let vals = cell.penciled().intersection(&locked_cands).cloned().collect::<Vec<Element>>();
//                     for value in vals {
//                         cell.remove_possible(value);
//                     }
//                 }
//
//
//             }
//         }
//         cand
//     }
//
//     // If the only possiblities in a box are in a row or column, these values can be removed from the rest
//     // of the block / column
//     fn locked_candidates_pointing(&mut self, fill: bool) -> Vec<(usize, Element)> {
//         let mut cand: Vec<(usize, Element)> = Vec::new();
//
//         for box_number in 0..MAX_NUM {
//             let start_ind = start_of_box(box_number);
//             for row in 0..BOX_DIMEN {
//                 let amount_to_skip = row * BOX_DIMEN;
//                 let start_row_index = start_ind + row * MAX_NUM;
//                 let start_col_index = index_to_col(start_ind);
//                //dbg!(&box_number, &row, &start_ind, &amount_to_skip, &start_row_index, &start_col_index);
//
//                 // Get a set of what is in the 6 other cells in the box, compare it to the three in question for the row
//                 let box_set = self
//                     .box_iter(start_ind)
//                     .enumerate()
//                     .filter(|(i,c)| (*i < amount_to_skip || *i >= (amount_to_skip + BOX_DIMEN)) && c.penciled().len() != 0 )
//                     .fold(BTreeSet::new(), |acc, (_, s)| acc.union(&s.penciled()).cloned().collect::<BTreeSet<Element>>());
//
//                 let row_set = self
//                     .row_iter(start_row_index)
//                     .skip(start_col_index)
//                     .enumerate()
//                     .filter(|(_, c)| c.penciled().len() != 0)
//                     .take_while(|(i, _)| *i < BOX_DIMEN)
//                     .fold (BTreeSet::new(), |acc, (_, s)| acc.union(&s.penciled()).cloned().collect::<BTreeSet<Element>>());
//
//                 // Get difference between sets.  Any difference will be a locked candidate in a box and other elements in the row can
//                 // have their possible values modified
//                 let locked_cands_num = row_set.difference(&box_set).cloned().collect::<Vec<Element>>();
//
//                 for num in locked_cands_num {
//                     let locked_cells = self
//                         .row_iter(start_row_index)
//                         .skip(start_col_index)
//                         .enumerate()
//                         .take_while(|(i, _)| *i < BOX_DIMEN)
//                         .filter(|(_, c)| c.penciled().contains(&num))
//                         .map(|(i,_)| i)
//                         .collect::<Vec<usize>>();
//
//                     let affected_cells = self
//                         .row_iter_mut(start_row_index)
//                         .enumerate()
//                         .filter (|(i, c)| (*i < start_col_index || *i >= start_col_index + BOX_DIMEN) && c.penciled().contains(&num))
//                         .map (|(_, c)| c)
//                         .collect::<Vec<&mut Cell>>();
//
//
//                     let locked_present = affected_cells.len() > 0;
//                     for c in affected_cells {
//                         c.remove_possible(num);
//                     }
//
//                     if locked_present {
//                         for i in locked_cells {
//                             cand.push((i + start_row_index , num));
//                         }
//                     }
//
//                 }
//
//             }
//
//             for col in 0..BOX_DIMEN {
//                 //let amount_to_skip = row * BOX_DIMEN;
//                 let start_row_index = index_to_row(start_ind);
//                 let start_col_index = start_ind + col;
//                 //dbg!(&box_number, &row, &start_ind, &start_row_index, &start_col_index);
//
//                 // Get a set of what is in the 6 other cells in the box, compare it to the three in question for the row
//                 let box_set = self
//                     .box_iter(start_ind)
//                     .enumerate()
//                     .filter(|(i,c)| *i % BOX_DIMEN != col && c.penciled().len() != 0 )
//                     .fold(BTreeSet::new(), |acc, (_, s)| acc.union(&s.penciled()).cloned().collect::<BTreeSet<Element>>());
//                 //dbg!(&box_set);
//                 let col_set = self
//                     .col_iter(start_col_index)
//                     .skip(start_row_index)
//                     .enumerate()
//                     .filter(|(_, c)| c.penciled().len() != 0)
//                     .take_while(|(i, _)| *i < BOX_DIMEN)
//                     .fold (BTreeSet::new(), |acc, (_, s)| acc.union(&s.penciled()).cloned().collect::<BTreeSet<Element>>());
//                 //dbg!(&col_set);
//                 // Get difference between sets.  Any difference will be a locked candidate in a box and other elements in the row can
//                 // have their possible values modified
//                 let locked_cands_num = col_set.difference(&box_set).cloned().collect::<Vec<Element>>();
//
//                 for num in locked_cands_num {
//                     let locked_cells = self
//                         .col_iter(start_col_index)
//                         .skip(start_row_index)
//                         .enumerate()
//                         .take_while(|(i, _)| *i < BOX_DIMEN)
//                         .filter(|(_, c)| c.penciled().contains(&num))
//                         .map(|(i,_)| i)
//                         .collect::<Vec<usize>>();
//                     // for i in &locked_cells {
//                     //     cand.push((i + start_row_index + start_col_index , num));
//                     // }
//                     let affected_cells = self
//                         .col_iter_mut(start_col_index)
//                         .enumerate()
//                         .filter (|(i, c)| (*i < start_row_index || *i >= start_row_index + BOX_DIMEN) && c.penciled().contains(&num))
//                         .map (|(_, c)| c)
//                         .collect::<Vec<&mut Cell>>();
//
//
//                     let locked_present = affected_cells.len() > 0;
//                     for c in affected_cells {
//                         c.remove_possible(num);
//                     }
//
//                     if locked_present {
//                         for i in locked_cells {
//                             cand.push((start_ind + col + i * MAX_NUM, num));
//                         }
//                     }
//
//                 }
//
//             }
//
//         }
//
//         cand
//     }
//
//     fn single_possibility(&mut self, fill: bool) -> Vec<(usize, Element)> {
//         enum IterResult {
//             None,
//             Multiple,
//             Single(usize),
//         }
//
//
//         let mut adds: Vec<(usize, Element)> = Vec::new();
//         loop {
//             let growth = adds.len();
//             // For every possible value in each row / column / box...
//             for iter_num in 0..MAX_NUM {
//                 for val in 1..=MAX_NUM as Element {
//                     // Find a cell where the number has only one possible location...
//                     match self.row_iter(get_cell(iter_num, 0)).enumerate().fold(
//                         IterResult::None,
//                         |found, (col, cell)| {
//                             // If the cell has a possibility of the value, see if it is unique.
//                             match cell.penciled.contains(&val) {
//                                 true => match found {
//                                     IterResult::None => IterResult::Single(get_cell(iter_num, col)),
//                                     IterResult::Single(_) => IterResult::Multiple,
//                                     _ => found,
//                                 },
//                                 _ => found,
//                             }
//                         },
//                     ) {
//                         IterResult::Single(ind) => {
//                             if fill {
//                                 self.cells[ind].set(val);
//                                 // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
//                                 self.cells[ind].fixed = true;
//                                 update_pencil_after_add(self, ind);
//                             }
//                             adds.push((ind, val));
//                         }
//                         _ => (),
//                     }
//
//                     // Same thing as above, but with columns
//                     match self.col_iter(get_cell(0, iter_num)).enumerate().fold(
//                         IterResult::None,
//                         |found, (row, cell)| {
//                             // If the cell has a possibility of the value, see if it is unique.
//                             match cell.penciled.contains(&val) {
//                                 true => match found {
//                                     IterResult::None => IterResult::Single(get_cell(row, iter_num)),
//                                     IterResult::Single(_) => IterResult::Multiple,
//                                     _ => found,
//                                 },
//                                 _ => found,
//                             }
//                         },
//                     ) {
//                         IterResult::Single(ind) => {
//                             if fill {
//                                 self.cells[ind].set(val);
//                                 // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
//                                 self.cells[ind].fixed = true;
//                                 update_pencil_after_add(self, ind);
//                             }
//                             adds.push((ind, val));
//                         }
//                         _ => (),
//                     }
//
//                     // Same thing as above, but with the box
//                     match self.box_iter(start_of_box(iter_num)).enumerate().fold(
//                         IterResult::None,
//                         |found, (index, cell)| {
//                             // If the cell has a possibility of the value, see if it is unique.
//                             match cell.penciled.contains(&val) {
//                                 true => match found {
//                                     IterResult::None => {
//                                         IterResult::Single(index_from_box(iter_num, index))
//                                     }
//                                     IterResult::Single(_) => IterResult::Multiple,
//                                     _ => found,
//                                 },
//                                 _ => found,
//                             }
//                         },
//                     ) {
//                         IterResult::Single(ind) => {
//                             if fill {
//                                 self.cells[ind].set(val);
//                                 // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
//                                 self.cells[ind].fixed = true;
//                                 update_pencil_after_add(self, ind);
//                             }
//                             adds.push((ind, val));
//                         }
//                         _ => (),
//                     }
//                 }
//             }
//             // Loop in case a situation where a cell is solved in box checks causes a cell to be solved
//             // in row checks.  Breaks the loop if no new ones are found.
//             if growth == adds.len() {
//                 break;
//             }
//         }
//
//         adds
//     }
//
//     fn naked_tuple(&mut self) -> BTreeSet<usize> {
//         // Used to find the index when iterating over a row.  'col' comes from the enumeration of the iterator
//         let ind_in_row = |row, col| get_cell(row, col);
//
//         // Used to find the index when iterating over a row.  'row' comes from the enumeration of the iterator, therefore these values have to be swapped
//         // to be used in a generic function based off of row iteration
//         let ind_in_col = |col, row| get_cell(row, col);
//
//         let ind_in_box = |el_box, ind| index_from_box(el_box, ind);
//
//         let mut eliminations: BTreeSet<usize> = BTreeSet::new();
//         for iteration in 0..MAX_NUM {
//             /*
//             The idea behind this algorithm is for each row / column / box, create a map with the pencil marks as
//             keys, and the index of the cell as value in the form of a vector.  If a cell has identical pencil marks
//             as another cell, it's cell index is added to the value vector.  If the amount of pencil marks equals the
//             number of cells with the identical pencil marks, then it can be determined that these numbers cannot appear
//             elsewhere is the respective row / column / box.  The pencil marks for these values will be removed from other blocks.
//              */
//
//             eliminations = eliminations
//                 .union(&mut find_tuples(
//                     iteration,
//                     &mut row_iter(self, iteration),
//                     ind_in_row,
//                 ))
//                 .cloned()
//                 .collect::<BTreeSet<usize>>();
//             eliminations = eliminations
//                 .union(&mut find_tuples(
//                     iteration,
//                     &mut col_iter(self, iteration),
//                     ind_in_col,
//                 ))
//                 .cloned()
//                 .collect::<BTreeSet<usize>>();
//             eliminations = eliminations
//                 .union(&mut find_tuples(
//                     iteration,
//                     &mut box_iter(self, iteration),
//                     ind_in_box,
//                 ))
//                 .cloned()
//                 .collect::<BTreeSet<usize>>();
//         }
//
//         fn row_iter(puz: &mut Puzzle, row: usize) -> impl Iterator<Item = &mut Cell> {
//             puz.row_iter_mut(get_cell(row, 0))
//         }
//
//         fn col_iter(puz: &mut Puzzle, col: usize) -> impl Iterator<Item = &mut Cell> {
//             puz.col_iter_mut(get_cell(0, col))
//         }
//
//         fn box_iter(puz: &mut Puzzle, el_box: usize) -> impl Iterator<Item = &mut Cell> {
//             puz.box_iter_mut(start_of_box(el_box))
//         }
//
//         fn find_tuples<'a, I, F>(iteration: usize, iter: &mut I, func: F) -> BTreeSet<usize>
//         where
//             I: Iterator<Item = &'a mut Cell>,
//             F: Fn(usize, usize) -> usize,
//         {
//             // Copy the cell references for the row / column / box
//             let mut the_cells_mut = iter.collect::<Vec<&'a mut Cell>>();
//
//
//             /*
//              Builds a map, mapping a cell's pencil marks to the cell(s) that have it.  If there are multiple
//              Cells stored as values, then these cells have _identical_ pencil marks. As in:
//              Cell #1 pencil marks: 2, 4, 7
//              Cell #2 pencil marks: 2, 4, 7
//              Cell #3 pencil marks: 2, 4, 7.
//
//              At this point in the function, it will not consider
//              Cell #1 pencil marks: 2, 7
//              Cell #2 pencil marks: 2, 4
//              Cell #3 pencil marks: 4, 7
//
//              as tuples.  The sets will have to be merged and iterated through a few times.
//              This occurs later in the function
//             */
//
//             let mut initial_sets = the_cells_mut
//                 .iter()
//                 .filter(|c| c.penciled().len() > 0)
//                 .map(|c| c.penciled().clone())
//                 .collect::<Vec<BTreeSet<Element>>>();
//
//             // More sets, including the (2,7), (2,4), (4,7) case above are found here.
//             let mut more_sets = find_new_sets(&initial_sets);
//             more_sets.append(&mut initial_sets);
//
//             // Map of pencil mark sets to indices which contain a subset of the marks.
//             // The indicies are relative to the iterator over the set.  I.e. index 2 on row 3 would be cell 20
//             let mut set_to_index: HashMap<BTreeSet<Element>, BTreeSet<usize>> = HashMap::new();
//
//             // Matches subsets of the set to cells.
//             for (i, cell) in the_cells_mut
//                 .iter()
//                 .enumerate()
//                 .filter(|(_, c)| !c.fixed())
//             {
//                 for set in &more_sets {
//                     if set.is_superset(cell.penciled()) {
//                         let mut insert: BTreeSet<usize> = BTreeSet::new();
//                         insert.insert( i);
//                         let residual = set_to_index.insert(set.clone(), insert);
//                         match residual {
//                             None => (),
//                             // If the key existed, the value gets removed and returned.  This adds it back in.
//                             Some(v) => {
//                                 let s = set_to_index.get_mut(&set).unwrap();
//                                 let s = &mut s.union(&v).cloned().collect::<BTreeSet<usize>>();
//                                 // todo: Fix this line and all the darn cloning
//                                 set_to_index.insert(set.clone(), s.clone());
//                             }
//                         }
//                     }
//                 }
//             }
//
//
//             let mut ret: BTreeSet<usize> = BTreeSet::new();
//             for (set, cells) in set_to_index {
//                 // It's a tuple if the size of the set is the same as the size of the number of cells that contain that set
//                 // of possible values
//                 let union_of_involved_sets = the_cells_mut.iter()
//                     .enumerate()
//                     .filter (|(i, _)| cells.contains(i))
//                     .fold(BTreeSet::new(), |acc, (_, c)| acc.union(c.penciled()).cloned().collect());
//                 if set.len() == cells.len() && set == union_of_involved_sets {
//                     // Clear out possiblities for other cells, but not the ones that make the tuple
//                     let nums_to_elim : BTreeSet<Element>= set.intersection(&union_of_involved_sets).cloned().collect();
//                     the_cells_mut
//                         .iter_mut()
//                         .enumerate()
//                         .map(|(i, cell)| {
//
//                             if !cells.contains(&i) {
//                                 for val in &nums_to_elim {
//
//                                     cell.remove_possible(*val);
//                                 }
//                             }
//                         })
//                         .all(|_| true);
//
//                     ret = ret
//                         .union(&cells.iter().map(|i| func(iteration,*i)).collect())
//                         .cloned()
//                         .collect::<BTreeSet<usize>>();
//                 }
//             }
//
//             // Used to merge sets together, so that sets like (2,7), (2,4), (4,7) can be paired.
//             fn find_new_sets(sets: &Vec<BTreeSet<Element>>) -> Vec<BTreeSet<Element>> {
//                 let mut ret: Vec<BTreeSet<Element>> = Vec::new();
//                 loop {
//                     let size = sets.len();
//                     for (i, one) in sets.iter().enumerate() {
//                         for other in sets.iter().skip(i + 1) {
//                             let joined: BTreeSet<Element> = one.union(&other).cloned().collect();
//                             if !ret.contains(&joined)  {
//                                 ret.push(joined);
//                             }
//                         }
//                     }
//                     // repeat the merging for 3 or more sets.
//
//                     if size == sets.len() {
//                         break;
//                     }
//                 }
//
//                 ret
//             }
//
//             ret
//         }
//
//         eliminations
//     }
//
//     fn hidden_tuple(&mut self) -> BTreeSet<usize> {
//         todo!();
//     }
// }
//
// /// After a number in a cell is added, removes this number from affected cell's penciled sets.
// pub fn update_pencil_after_add(puz: &mut Puzzle, index: usize) {
//     let val = puz.cells[index].num();
//     // I would have liked to have used .zip() for .chain() on these iterators, but
//     // cannot have multiple mutable borrows of 'puz'
//     for cell in puz.col_iter_mut(index) {
//         cell.remove_possible(val);
//     }
//     for cell in puz.row_iter_mut(index) {
//         cell.remove_possible(val);
//     }
//     for cell in puz.box_iter_mut(index) {
//         cell.remove_possible(val);
//     }
// }
// #[cfg(test)]
// mod human_method_tests {
//     use super::*;
//
//     fn get_example() -> Vec<Vec<Element>> {
//         vec![
//             vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
//             vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
//             vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
//             vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
//             vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
//             vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
//             vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
//             vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
//             vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
//         ]
//     }
//
//     fn get_puzzle() -> Puzzle {
//         let mut puz = Puzzle::new();
//         puz.set_initial(get_example().as_input().unwrap());
//         puz
//     }
//
//     #[test]
//     fn remove_pencil_test() {
//         let mut puz = get_puzzle();
//         let cell1 = &puz.cells[2];
//         let cell2 = &puz.cells[10];
//         let cell3 = &puz.cells[17];
//         let cell4 = &puz.cells[73];
//         let cells = [cell1, cell2, cell3, cell4];
//         for c in cells.iter() {
//             assert!(c.penciled.contains(&2));
//         }
//         puz.cells[10].set(2);
//         update_pencil_after_add(&mut puz, 10);
//
//         let cell1 = &puz.cells[2];
//         let cell2 = &puz.cells[10];
//         let cell3 = &puz.cells[17];
//         let cell4 = &puz.cells[73];
//         let cells = [cell1, cell2, cell3, cell4];
//
//         for c in cells.iter() {
//             assert!(!c.penciled.contains(&2));
//         }
//     }
//
//     #[test]
//     fn single_cand() {
//         let mut puz = Puzzle::new();
//         puz.set_initial(
//             (vec![
//                 vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
//                 vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
//                 vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
//                 vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
//                 vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
//                 vec![0, 0, 0, 0, 2, 0, 0, 0, 6],
//                 vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
//                 vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
//                 vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
//             ])
//             .as_input()
//             .unwrap(),
//         );
//
//         let expected: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
//         ];
//
//         let res = puz.single_candidate(true);
//
//         assert!(res.contains(&(10, 7)));
//         assert!(res.contains(&(18, 1)));
//
//         // This puzzle is solved by 100% single candidates.
//         for (act, exp) in puz.cells.iter().zip(expected.iter().flatten()) {
//             assert_eq!(act.num(), *exp);
//         }
//     }
//
//     #[test]
//     fn locked_cand_test () {
//         let mut puz = Puzzle::new();
//         let str = "984........25...4...19.4..2..6.9723...36.2...2.9.3561.195768423427351896638..9751";
//         puz.set_initial(str.as_input().unwrap());
//         assert!(puz.brute_force_solve().len() == 1);
//
//         let mut puz = Puzzle::new();
//         let str = "984........25...4...19.4..2..6.9723...36.2...2.9.3561.195768423427351896638..9751";
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.locked_candidates_pointing(true);
//
//         assert!(res.contains(&(get_cell(2,0), 5)));
//         assert!(res.contains(&(get_cell(2,1), 5)));
//         assert!(!puz.cells[get_cell(2, 6)].penciled().contains(&5));
//
//         let str = "34...6.7..8....93...2.3..6.....1.....9736485......2...............6.8.9....923785";
//
//         let mut puz = Puzzle::new();
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.locked_candidates_pointing(true);
//         assert!(res.contains(&(get_cell(6,3), 1)));
//         assert!(res.contains(&(get_cell(6,5), 1)));
//         assert!(!puz.cells[get_cell(6, 6)].penciled().contains(&1));
//
//         let str = "58.4.1.2994.2....12.15.94.....91...6.9.64.1...1.82.79...4192...1.9......82..5.91.";
//         let mut puz = Puzzle::new();
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.locked_candidates_pointing(true);
//         assert!(res.contains(&(55, 5)));
//         assert!(res.contains(&(64, 5)));
//         assert!(!puz.cells[get_cell(3, 1)].penciled().contains(&5));
//
//     }
//
//     #[test]
//     fn locked_candidate_claiming_test () {
//         let str = "318..54.6...6.381...6.8.5.3864952137123476958795318264.3.5..78......73.5....39641";
//         let mut puz = Puzzle::new();
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.locked_candidates_claiming(true);
//         dbg!(&res);
//         assert!(res.contains(&(10, 7)));
//         assert!(res.contains(&(11, 7)));
//         assert!(!res.contains(&(19, 7)));
//         assert!(puz.cells[19].penciled().contains(&7));
//
//         let str = "762..8..198......615.....87478..3169526..98733198..425835..1692297685314641932758";
//         let mut puz = Puzzle::new();
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.locked_candidates_claiming(true);
//         assert!(res.contains(&(15, 4)));
//         assert!(res.contains(&(24,4)));
//         assert!(!puz.cells[3].penciled().contains(&4));
//         assert!(!puz.cells[13].penciled().contains(&4));
//         assert!(!puz.cells[21].penciled().contains(&4));
//
//
//
//     }
//
//     #[test]
//     fn diff_single_poss () {
//         let mut puz1 = get_puzzle();
//         let mut puz2 = get_puzzle();
//         puz1.single_possibility(true);
//         puz2.single_possibility_slower(true);
//
//         assert_eq!(puz1, puz2);
//
//     }
//
//     #[test]
//     fn single_possibility_test() {
//         let mut puz = Puzzle::new();
//         puz.set_initial(
//             (vec![
//                 vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
//                 vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
//                 vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
//                 vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
//                 vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
//                 vec![0, 1, 0, 0, 2, 0, 0, 0, 6],
//                 vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
//                 vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
//                 vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
//             ])
//             .as_input()
//             .unwrap(),
//         );
//
//         let expected: Vec<Element> = (vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
//         ])
//         .as_input()
//         .unwrap();
//
//         let res = puz.single_possibility(true);
//
//         assert!(res.contains(&(get_cell(2, 6), 5)));
//         assert!(res.contains(&(get_cell(5, 6), 8)));
//         assert!(res.contains(&(get_cell(6, 2), 1)));
//         assert!(res.contains(&(get_cell(3, 5), 1)));
//
//         /* This puzzle does not get completely solved using this method.
//          * The remaining portion of the puzzle with be brute forced solved to ensure that the solving is correct
//          */
//         let finished = puz.brute_force_solve();
//         assert_eq!(finished.len(), 1);
//         assert_eq!(finished[0], expected);
//     }
//
//     #[test]
//     fn single_possibility_faster_test() {
//         let mut puz = Puzzle::new();
//         puz.set_initial(
//             (vec![
//                 vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
//                 vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
//                 vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
//                 vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
//                 vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
//                 vec![0, 1, 0, 0, 2, 0, 0, 0, 6],
//                 vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
//                 vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
//                 vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
//             ])
//                 .as_input()
//                 .unwrap(),
//         );
//
//         let expected: Vec<Element> = (vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
//         ])
//             .as_input()
//             .unwrap();
//
//         let res = puz.single_possibility_slower(true);
//
//         assert!(res.contains(&(get_cell(2, 6), 5)));
//         assert!(res.contains(&(get_cell(5, 6), 8)));
//         assert!(res.contains(&(get_cell(6, 2), 1)));
//         assert!(res.contains(&(get_cell(3, 5), 1)));
//
//         /* This puzzle does not get completely solved using this method.
//          * The remaining portion of the puzzle with be brute forced solved to ensure that the solving is correct
//          */
//         let finished = puz.brute_force_solve();
//         assert_eq!(finished.len(), 1);
//         assert_eq!(finished[0], expected);
//     }
//
//     #[test]
//     fn naked_tuples_test() {
//         let mut puz = Puzzle::new();
//
//         /*
//         Naked double check
//          */
//         puz.set_initial(
//             (vec![
//                 vec![7, 0, 0, 8, 4, 9, 0, 3, 0],
//                 vec![9, 2, 8, 1, 3, 5, 0, 0, 6],
//                 vec![4, 0, 0, 2, 6, 7, 0, 8, 9],
//                 vec![6, 4, 2, 7, 8, 3, 9, 5, 1],
//                 vec![3, 9, 7, 4, 5, 1, 6, 2, 8],
//                 vec![8, 1, 5, 6, 9, 2, 3, 0, 0],
//                 vec![2, 0, 4, 5, 1, 6, 0, 9, 3],
//                 vec![1, 0, 0, 0, 0, 8, 0, 6, 0],
//                 vec![5, 0, 0, 0, 0, 4, 0, 1, 0],
//             ])
//             .as_input()
//             .unwrap(),
//         );
//
//         let res = puz.naked_tuple();
//
//         assert!(res.contains(&get_cell(7, 2)));
//         assert!(res.contains(&get_cell(7, 3)));
//         assert!(res.contains(&get_cell(1, 6)));
//         assert!(res.contains(&get_cell(1, 7)));
//
//         //ensure puzzle is solvable after pencil mark eliminations. This puzzle is solvable with
//         // single candidate which uses the pencil marks
//         let count = puz.single_candidate(true);
//         assert_eq!(count.len(), 25);
//
//         // Quick validity check over all filled in blocks.
//         let res = puz.brute_force_solve();
//         assert_eq!(res.len(), 1);
//
//         /*
//         Naked triple check
//          */
//         let example = vec![
//             vec![0, 0, 0, 2, 9, 4, 3, 8, 0],
//             vec![0, 0, 0, 1, 7, 8, 6, 4, 0],
//             vec![4, 8, 0, 3, 5, 6, 1, 0, 0],
//             vec![0, 0, 4, 8, 3, 7, 5, 0, 1],
//             vec![0, 0, 0, 4, 1, 5, 7, 0, 0],
//             vec![5, 0, 0, 6, 2, 9, 8, 3, 4],
//             vec![9, 5, 3, 7, 8, 2, 4, 1, 6],
//             vec![1, 2, 6, 5, 4, 3, 9, 7, 8],
//             vec![0, 4, 0, 9, 6, 1, 2, 5, 3],
//         ];
//
//         let mut puz = Puzzle::new();
//         puz.set_initial(example.as_input().unwrap());
//
//         let res = puz.naked_tuple();
//         assert!(res.contains(&get_cell(1, 1)));
//         assert!(res.contains(&get_cell(3, 1)));
//         assert!(res.contains(&get_cell(4, 1)));
//         assert!(!puz.cells[1].penciled.contains(&6));
//
//         assert!(res.contains(&get_cell(3, 7)));
//         assert!(res.contains(&get_cell(4, 7)));
//         assert!(res.contains(&get_cell(4, 8)));
//
//
//         // Quick validity check over all filled in blocks.
//         let res = puz.brute_force_solve();
//         assert_eq!(res.len(), 1);
//
//         let example = vec![
//             vec![3, 9, 0, 0, 0, 0, 7, 0, 0],
//             vec![0, 0, 0, 0, 0, 0, 6, 5, 0],
//             vec![5, 0, 7, 0, 0, 0, 3, 4, 9],
//             vec![0, 4, 9, 3, 8, 0, 5, 0, 6],
//             vec![6, 0, 1, 0, 5, 4, 9, 8, 3],
//             vec![8, 5, 3, 0, 0, 0, 4, 0, 0],
//             vec![9, 0, 0, 8, 0, 0, 1, 3, 4],
//             vec![0, 0, 2, 9, 4, 0, 8, 6, 5],
//             vec![4, 0, 0, 0, 0, 0, 2, 9, 7],
//         ];
//
//         let mut puz = Puzzle::new();
//         puz.set_initial(example.as_input().unwrap());
//         // There are in fact no single candidates.  Using pairs will free some up and be able to
//         // be solved later.
//         let uns = puz.single_candidate(true);
//         assert_eq!(uns.len(), 0);
//
//         let res = puz.naked_tuple();
//
//         assert!(res.contains(&get_cell(0, 4)));
//         assert!(res.contains(&get_cell(2, 3)));
//         assert!(res.contains(&get_cell(2, 4)));
//
//         assert!(puz.cells[get_cell(2, 3)].penciled.contains(&1));
//         assert!(!puz.cells[get_cell(1, 4)].penciled.contains(&1));
//
//         // Puzzle is solvable with single_candidate now that some pencil marks were eliminated by tuples.
//         let count = puz.single_candidate(true);
//         assert!(count.len() == 39); // 39 empty cells
//
//         // Quick validity check over all filled in blocks.
//         let res = puz.brute_force_solve();
//         assert_eq!(res.len(), 1);
//
//         // The following test for hidden triples
//         let str = "5..62..37..489........5....93........2....6.57.......3.....9............68.57...2";
//         let mut puz = Puzzle::new();
//         puz.set_initial(str.as_input().unwrap());
//         let res = puz.naked_tuple();
//         let threefive = get_cell (3,5);
//         assert!(res.contains(&threefive));
//         assert!(puz.cells[threefive].penciled().contains(&5));
//         assert!(puz.cells[threefive].penciled().contains(&2));
//         assert!(puz.cells[threefive].penciled().contains(&6));
//         assert!(!puz.cells[threefive].penciled().contains(&1));
//
//
//     }
//
//     #[test]
//     fn royles_17 () {
//         let str = "003400000050000090000200000000096000000000300001000204060080000000000400890050000";
//         let strin = str.chars().into_iter().map(|c| match c {
//             c if c == '0' => '.',
//             c if c == '4' => '8',
//             c if c == '8' => '4',
//             c => c,
//         }).collect::<String>();
//
//         let mut puz = Puzzle::new();
//         puz.set_initial(strin.as_str().as_input().unwrap());
//        puz.single_candidate(true);
//        puz.single_possibility(true);
//        puz.naked_tuple();
//
//
//         puz.single_possibility(true);
//         puz.single_candidate(true);
//         for (_, cell) in puz.cells.iter().enumerate() {
//             assert!({
//                 if cell.fixed() {
//                     true
//                 } else {
//                     cell.penciled().len() > 0
//                 }
//             })
//         }
//         let res = puz.brute_force_solve();
//         assert_eq!(res.len(), 1);
//
//     }
// }
