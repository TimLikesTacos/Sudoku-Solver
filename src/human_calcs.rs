use crate::*;
use std::collections::HashMap;

pub trait BasicHumanMethods {
    /// Finds cells that have only one possible value, fills it in, and removes pencil marks for
    /// related cells.  A single candidate is where only one value is possible in that cell.
    /// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
    /// Returns a vector of tuples of (index value, number filled in).
    fn single_candidate(&mut self, fill: bool) -> Vec<(usize, Element)>;

    /// Fills in single possibilities, returns vector of what positions were filled.
    /// A single possibility is where only one cell in a row/column/box has the possibility of a value.
    /// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
    /// Returns a vector of tuples of (index value, number filled in).
    fn single_possibility(&mut self, fill: bool) -> Vec<(usize, Element)>;

    /// A naked pair is where two cells have the same potential values, and no others.
    /// For example, two cells have a possibility of either `2`, or `4`. This means that `2` belongs to one of these
    /// cells, and `4` to the other.  These possiblities can be eliminated in other cells in the associated row, column, or box
    /// These can be in the form of pairs, triples, or more.  It will only be evaluated from pairs to quadruples (for a 9 cell game), since
    /// checking for quintuples is the same as quadruples.  Generically, it will be `n/2` with `n` being the MAX_NUM value.
    /// This method does not fill in cells, only eliminates possibilities which can support other methods like
    /// single_candidate or single_possibility.   
    /// Returns a set containing the indicies in which belong to a x-uple.
    fn naked_tuple(&mut self) -> BTreeSet<usize>;

    /// A hidden pair is similar to a naked pair, however it contains other possibilities.
    /// For example, a cell has the possibility of either `2`, `4`, or '7'. Another cell has a possiblity of '2', '4', '6'.  The other cells in the row
    /// do not have the possiblity of '2' or '4'.  Therefore, it can be similified that the first two cells are only '2' or '4', and the remaining
    /// possibilities in these cells can be removed, along with '2' and '4' in the remaining cells.  
    /// This method does not fill in cells, only eliminates possibilities which can support other methods like
    /// single_candidate or single_possibility.   
    /// Returns a set containing the indicies in which belong to a x-uple.
    fn hidden_tuple(&mut self) -> BTreeSet<usize>;
}

impl BasicHumanMethods for Puzzle {
    fn single_candidate(&mut self, fill: bool) -> Vec<(usize, Element)> {
        // This vector contains coordinate and value for cells that are solved for.  This can be used to debug
        // or function modified to return this vector for display.
        let mut count: Vec<(usize, Element)> = Vec::new();

        loop {
            let initial_size = count.len();
            // Iterate through each cell, get its pencil marks, see if there is only one candidate.
            // If only one, update it, remove associated pencil marks, update count vector
            let single_cands = (0..NUM_CELLS)
                .into_iter()
                .filter(|i| self.cells[*i].penciled_mut().len() == 1)
                .collect::<Vec<usize>>();

            for i in single_cands {
                let val: Element = *self.cells[i].penciled().iter().next().unwrap();
                // Mark the cell that has one candidate
                if fill {
                    self.cells[i].set(val);
                    update_pencil_after_add(self, i);
                }
                // Update others that this value is now no longer a candidate
                count.push((i, val));
            }
            // Loop to see if finding single candidates clears the way for new single candidates
            // breaks if no single candidates were found.
            if initial_size == count.len() {
                break;
            }
        }
        count
    }

    fn single_possibility(&mut self, fill: bool) -> Vec<(usize, Element)> {
        enum IterResult {
            None,
            Multiple,
            Single(usize),
        }

        let mut adds: Vec<(usize, Element)> = Vec::new();
        loop {
            let growth = adds.len();
            // For every possible value in each row / column / box...
            for iter_num in 0..MAX_NUM {
                for val in 1..=MAX_NUM as Element {
                    // Find a cell where the number has only one possible location...
                    match self.row_iter(get_cell(iter_num, 0)).enumerate().fold(
                        IterResult::None,
                        |found, (col, cell)| {
                            // If the cell has a possibility of the value, see if it is unique.
                            match cell.penciled.contains(&val) {
                                true => match found {
                                    IterResult::None => IterResult::Single(get_cell(iter_num, col)),
                                    IterResult::Single(_) => IterResult::Multiple,
                                    _ => found,
                                },
                                _ => found,
                            }
                        },
                    ) {
                        IterResult::Single(ind) => {
                            if fill {
                                self.cells[ind].set(val);
                                // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
                                self.cells[ind].fixed = true;
                                update_pencil_after_add(self, ind);
                            }
                            adds.push((ind, val));
                        }
                        _ => (),
                    }

                    // Same thing as above, but with columns
                    match self.col_iter(get_cell(0, iter_num)).enumerate().fold(
                        IterResult::None,
                        |found, (row, cell)| {
                            // If the cell has a possibility of the value, see if it is unique.
                            match cell.penciled.contains(&val) {
                                true => match found {
                                    IterResult::None => IterResult::Single(get_cell(row, iter_num)),
                                    IterResult::Single(_) => IterResult::Multiple,
                                    _ => found,
                                },
                                _ => found,
                            }
                        },
                    ) {
                        IterResult::Single(ind) => {
                            if fill {
                                self.cells[ind].set(val);
                                // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
                                self.cells[ind].fixed = true;
                                update_pencil_after_add(self, ind);
                            }
                            adds.push((ind, val));
                        }
                        _ => (),
                    }

                    // Same thing as above, but with the box
                    match self.box_iter(start_of_box(iter_num)).enumerate().fold(
                        IterResult::None,
                        |found, (index, cell)| {
                            // If the cell has a possibility of the value, see if it is unique.
                            match cell.penciled.contains(&val) {
                                true => match found {
                                    IterResult::None => {
                                        IterResult::Single(index_from_box(iter_num, index))
                                    }
                                    IterResult::Single(_) => IterResult::Multiple,
                                    _ => found,
                                },
                                _ => found,
                            }
                        },
                    ) {
                        IterResult::Single(ind) => {
                            if fill {
                                self.cells[ind].set(val);
                                // Either make the cell fixed, or don't remove pencil marks.  Causes issues if brute force is also used.
                                self.cells[ind].fixed = true;
                                update_pencil_after_add(self, ind);
                            }
                            adds.push((ind, val));
                        }
                        _ => (),
                    }
                }
            }
            // Loop in case a situation where a cell is solved in box checks causes a cell to be solved
            // in row checks.  Breaks the loop if no new ones are found.
            if growth == adds.len() {
                break;
            }
        }
        adds
    }

    fn naked_tuple(&mut self) -> BTreeSet<usize> {


        // Used to find the index when iterating over a row.  'col' comes from the enumeration of the iterator
        let ind_in_row = |row, col| get_cell(row, col);

        // Used to find the index when iterating over a row.  'row' comes from the enumeration of the iterator, therefore these values have to be swapped
        // to be used in a generic function based off of row iteration
        let ind_in_col = |col, row| get_cell(row, col);

        let ind_in_box = |el_box, ind| index_from_box(el_box, ind);

        let mut eliminations: BTreeSet<usize> = BTreeSet::new();
        for iteration in 0..MAX_NUM {
            /*
            The idea behind this algorithm is for each row / column / box, create a map with the pencil marks as
            keys, and the index of the cell as value in the form of a vector.  If a cell has identical pencil marks
            as another cell, it's cell index is added to the value vector.  If the amount of pencil marks equals the
            number of cells with the identical pencil marks, then it can be determined that these numbers cannot appear
            elsewhere is the respective row / column / box.  The pencil marks for these values will be removed from other blocks.
             */

            eliminations = eliminations
                .union(&mut find_tuples(
                    iteration,
                    &mut row_iter(self, iteration),
                    ind_in_row,
                ))
                .cloned()
                .collect::<BTreeSet<usize>>();
            eliminations = eliminations
                .union(&mut find_tuples(
                    iteration,
                    &mut col_iter(self, iteration),
                    ind_in_col,
                ))
                .cloned()
                .collect::<BTreeSet<usize>>();
            eliminations = eliminations
                .union(&mut find_tuples(
                    iteration,
                    &mut box_iter(self, iteration),
                    ind_in_box,
                ))
                .cloned()
                .collect::<BTreeSet<usize>>();
        }

        fn row_iter(puz: &mut Puzzle, row: usize) -> impl Iterator<Item = &mut Cell> {
            puz.row_iter_mut(get_cell(row, 0))
        }

        fn col_iter(puz: &mut Puzzle, col: usize) -> impl Iterator<Item = &mut Cell> {
            puz.col_iter_mut(get_cell(0, col))
        }

        fn box_iter(puz: &mut Puzzle, el_box: usize) -> impl Iterator<Item = &mut Cell> {
            puz.box_iter_mut(start_of_box(el_box))
        }

        fn find_tuples<'a, I, F>(iteration: usize, iter: &mut I, func: F) -> BTreeSet<usize>
        where
            I: Iterator<Item = &'a mut Cell>,
            F: Fn(usize, usize) -> usize,
        {
            // Copy the cell references
            let mut the_cells_mut = iter.collect::<Vec<&'a mut Cell>>();

            let mut initial_sets = the_cells_mut
                .iter()
                .filter(|c| c.penciled().len() > 0)
                .map(|c| c.penciled().clone())
                .collect::<Vec<BTreeSet<Element>>>();

            let mut more_sets = find_new_sets(&initial_sets);
            more_sets.append(&mut initial_sets);

            // Map of pencil mark sets to indices which contain a subset of the marks.
            let mut set_to_index: HashMap<BTreeSet<Element>, BTreeSet<usize>> = HashMap::new();

            // Matches subsets of the set to cells.
            for (i, cell) in the_cells_mut.iter().enumerate().filter(|(_, c)| !c.fixed()) {
                for set in &more_sets {
                    if set.is_superset(cell.penciled()) {
                        let mut insert: BTreeSet<usize> = BTreeSet::new();
                        insert.insert(func(iteration, i));
                        let residual = set_to_index.insert(set.clone(), insert);
                        match residual {
                            None => (),
                            // If the key existed, the value gets removed and returned.  This adds it back in.
                            Some(v) => {
                                let s = set_to_index.get_mut(&set).unwrap();
                                let s = &mut s.union(&v).cloned().collect::<BTreeSet<usize>>();
                                // todo: Fix this line and all the darn cloning
                                set_to_index.insert(set.clone(), s.clone());
                            }
                        }
                    }
                }
            }

            let mut ret: BTreeSet<usize> = BTreeSet::new();
            for (set, cells) in set_to_index {
                // It's a tuple if the size of the set is the same as the size of the number of cells that contain that set
                // of possible values
                if set.len() == cells.len() {
                    // Clear out possiblities for other cells, but not the ones that make the tuple
                    the_cells_mut
                        .iter_mut()
                        .enumerate()
                        .map(|(i, cell)| {
                            let index = func(iteration, i);
                            if !cells.contains(&index) {
                                for val in &set {
                                    cell.remove_possible(*val);
                                }
                            }
                        })
                        .all(|_| true);

                    ret = ret
                        .union(&cells.clone())
                        .cloned()
                        .collect::<BTreeSet<usize>>();
                }
            }

            /*
             Builds a map, mapping a cell's pencil marks to the cell(s) that have it.  If there are multiple
             Cells stored as values, then these cells have _identical_ pencil marks. As in:
             Cell #1 pencil marks: 2, 4, 7
             Cell #2 pencil marks: 2, 4, 7
             Cell #3 pencil marks: 2, 4, 7.

             At this point in the function, it will not consider
             Cell #1 pencil marks: 2, 7
             Cell #2 pencil marks: 2, 4
             Cell #3 pencil marks: 4, 7

             as tuples.  The sets will have to be merged and iterated through a few times.
             This occurs later in the function
            */

            fn find_new_sets(sets: &Vec<BTreeSet<Element>>) -> Vec<BTreeSet<Element>> {
                let mut ret: Vec<BTreeSet<Element>> = Vec::new();
                loop {
                    let size = sets.len();
                    for (i, one) in sets.iter().enumerate() {
                        for other in sets.iter().skip(i + 1) {

                            let joined: BTreeSet<Element> = one.union(&other).map(|x| *x).collect();
                            if joined.len() <= MAX_NUM / 2 {
                                ret.push(joined);
                            }
                        }
                    }
                    // repeat the merging for 3 or more sets.

                    if size == sets.len() {
                        break;
                    }
                }
                ret
            }

            ret
        }

        eliminations
    }

    fn hidden_tuple(&mut self) -> BTreeSet<usize> {
        todo!();
    }
}

/// After a number in a cell is added, removes this number from affected cell's penciled sets.
pub fn update_pencil_after_add(puz: &mut Puzzle, index: usize) {
    let val = puz.cells[index].num();
    // I would have liked to have used .zip() for .chain() on these iterators, but
    // cannot have multiple mutable borrows of 'puz'
    for cell in puz.col_iter_mut(index) {
        cell.remove_possible(val);
    }
    for cell in puz.row_iter_mut(index) {
        cell.remove_possible(val);
    }
    for cell in puz.box_iter_mut(index) {
        cell.remove_possible(val);
    }
}
#[cfg(test)]
mod human_method_tests {
    use super::*;

    fn get_example() -> Vec<Vec<Element>> {
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

    fn get_puzzle() -> Puzzle {
        let mut puz = Puzzle::new();
        puz.set_initial(get_example().as_input().unwrap());
        puz
    }

    #[test]
    fn remove_pencil_test() {
        let mut puz = get_puzzle();
        let cell1 = &puz.cells[2];
        let cell2 = &puz.cells[10];
        let cell3 = &puz.cells[17];
        let cell4 = &puz.cells[73];
        let cells = [cell1, cell2, cell3, cell4];
        for c in cells.iter() {
            assert!(c.penciled.contains(&2));
        }
        puz.cells[10].set(2);
        update_pencil_after_add(&mut puz, 10);

        let cell1 = &puz.cells[2];
        let cell2 = &puz.cells[10];
        let cell3 = &puz.cells[17];
        let cell4 = &puz.cells[73];
        let cells = [cell1, cell2, cell3, cell4];

        for c in cells.iter() {
            assert!(!c.penciled.contains(&2));
        }
    }

    #[test]
    fn single_cand() {
        let mut puz = Puzzle::new();
        puz.set_initial(
            (vec![
                vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
                vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                vec![0, 0, 0, 0, 2, 0, 0, 0, 6],
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
            ])
            .as_input()
            .unwrap(),
        );

        let expected: Vec<Vec<Element>> = vec![
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

        let res = puz.single_candidate( true);

        assert!(res.contains(&(10, 7)));
        assert!(res.contains(&(18, 1)));

        // This puzzle is solved by 100% single candidates.
        for (act, exp) in puz.cells.iter().zip(expected.iter().flatten()) {
            assert_eq!(act.num(), *exp);
        }
    }

    #[test]
    fn single_possibility_test() {
        let mut puz = Puzzle::new();
        puz.set_initial(
            (vec![
                vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
                vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                vec![0, 1, 0, 0, 2, 0, 0, 0, 6],
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
            ])
            .as_input()
            .unwrap(),
        );

        let expected: Vec<Element> = (vec![
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

        let res = puz.single_possibility( true);

        assert!(res.contains(&(get_cell(2, 6), 5)));
        assert!(res.contains(&(get_cell(5, 6), 8)));
        assert!(res.contains(&(get_cell(6, 2), 1)));
        assert!(res.contains(&(get_cell(3, 5), 1)));

        /* This puzzle does not get completely solved using this method.
         * The remaining portion of the puzzle with be brute forced solved to ensure that the solving is correct
         */
        let finished = puz.brute_force_solve();
        assert_eq!(finished.len(), 1);
        assert_eq!(finished[0], expected);
    }

    #[test]
    fn naked_tuples_test() {
        let mut puz = Puzzle::new();

        /*
        Naked double check
         */
        puz.set_initial(
            (vec![
                vec![7, 0, 0, 8, 4, 9, 0, 3, 0],
                vec![9, 2, 8, 1, 3, 5, 0, 0, 6],
                vec![4, 0, 0, 2, 6, 7, 0, 8, 9],
                vec![6, 4, 2, 7, 8, 3, 9, 5, 1],
                vec![3, 9, 7, 4, 5, 1, 6, 2, 8],
                vec![8, 1, 5, 6, 9, 2, 3, 0, 0],
                vec![2, 0, 4, 5, 1, 6, 0, 9, 3],
                vec![1, 0, 0, 0, 0, 8, 0, 6, 0],
                vec![5, 0, 0, 0, 0, 4, 0, 1, 0],
            ])
            .as_input()
            .unwrap(),
        );

        let res = puz.naked_tuple();

        assert!(res.contains(&get_cell(7, 2)));
        assert!(res.contains(&get_cell(7, 3)));
        assert!(res.contains(&get_cell(1, 6)));
        assert!(res.contains(&get_cell(1, 7)));


        //ensure puzzle is solvable after pencil mark eliminations. This puzzle is solvable with
        // single candidate which uses the pencil marks
        let count = puz.single_candidate( true);
        assert_eq!(count.len(), 25);

        // Quick validity check over all filled in blocks.
        let res = puz.brute_force_solve();
        assert_eq!(res.len(), 1);

        /*
        Naked triple check
         */
        let example = vec![
            vec![0, 0, 0, 2, 9, 4, 3, 8, 0],
            vec![0, 0, 0, 1, 7, 8, 6, 4, 0],
            vec![4, 8, 0, 3, 5, 6, 1, 0, 0],
            vec![0, 0, 4, 8, 3, 7, 5, 0, 1],
            vec![0, 0, 0, 4, 1, 5, 7, 0, 0],
            vec![5, 0, 0, 6, 2, 9, 8, 3, 4],
            vec![9, 5, 3, 7, 8, 2, 4, 1, 6],
            vec![1, 2, 6, 5, 4, 3, 9, 7, 8],
            vec![0, 4, 0, 9, 6, 1, 2, 5, 3],
        ];

        let mut puz = Puzzle::new();
        puz.set_initial(example.as_input().unwrap());

        let res = puz.naked_tuple();
        assert!(res.contains(&get_cell(1, 1)));
        assert!(res.contains(&get_cell(3, 1)));
        assert!(res.contains(&get_cell(4, 1)));
        assert!(!puz.cells[1].penciled.contains(&6));

        assert!(res.contains(&get_cell(3, 7)));
        assert!(res.contains(&get_cell(4, 7)));
        assert!(res.contains(&get_cell(4, 8)));

        // let count = single_candidate(&mut puz, true);
        // // assert_eq!(count.len(), 23);

        // todo redo brute force to use pencil marks
        // Quick validity check over all filled in blocks.
        let res = puz.brute_force_solve();
        assert_eq!(res.len(), 1);

        let example = vec![
            vec![3, 9, 0, 0, 0, 0, 7, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 6, 5, 0],
            vec![5, 0, 7, 0, 0, 0, 3, 4, 9],
            vec![0, 4, 9, 3, 8, 0, 5, 0, 6],
            vec![6, 0, 1, 0, 5, 4, 9, 8, 3],
            vec![8, 5, 3, 0, 0, 0, 4, 0, 0],
            vec![9, 0, 0, 8, 0, 0, 1, 3, 4],
            vec![0, 0, 2, 9, 4, 0, 8, 6, 5],
            vec![4, 0, 0, 0, 0, 0, 2, 9, 7],
        ];

        let mut puz = Puzzle::new();
        puz.set_initial(example.as_input().unwrap());
        // There are in fact no single candidates.  Using pairs will free some up and be able to
        // be solved later.
        let uns = puz.single_candidate( true);
        assert_eq!(uns.len(), 0);

        let res = puz.naked_tuple();

        assert!(res.contains(&get_cell(0, 4)));
        assert!(res.contains(&get_cell(2, 3)));
        assert!(res.contains(&get_cell(2, 4)));

        assert!(puz.cells[get_cell(2, 3)].penciled.contains(&1));
        assert!(!puz.cells[get_cell(1, 4)].penciled.contains(&1));

        // Puzzle is solvable with single_candidate now that some pencil marks were eliminated by tuples.
        let count = puz.single_candidate( true);
        assert!(count.len() == 39); // 39 empty cells

        // Quick validity check over all filled in blocks.
        let res = puz.brute_force_solve();
        assert_eq!(res.len(), 1);
    }
}
