use crate::*;

/// Fills in single candidates, returns vector of what positions were filled.
/// A single candidate is where only one value is possible in that cell.
/// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
/// Returns a vector of tuples of (index value, number filled in).
fn single_candidate (puz: &mut Puzzle, fill: bool) -> Vec<(usize, usize)> {

    // This vector contains coordinate and value for cells that are solved for.  This can be used to debug
    // or function modified to return this vector for display.
    let mut count: Vec<(usize, usize)> = Vec::new();

    loop {
        let initial_size = count.len();
        // Iterate through each cell, get its pencil marks, see if there is only one candidate.
        // If only one, update it, remove associated pencil marks, update count vector
        let single_cands = (0..NUM_CELLS).into_iter()
            .filter(|i| puz.cells[*i].get_penciled().len() == 1)
            // Mark the cell that has one candidate, update pencil marks for others
            .collect::<Vec<usize>>();

        for i in single_cands {

            let val = puz.cells[i].get_penciled().drain().collect::<Vec<usize>>()[0];
            puz.cells[i].set(val);

            update_pencil_after_add(puz, i);
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

/// After a number in a cell is added, removes this number from affected cell's penciled sets.
pub fn update_pencil_after_add (puz: &mut Puzzle, index: usize) {
    let val = puz.cells[index].num();
    // I would have liked to have used .zip() for .chain() on these iterators, but got into
    // errors regarding more than one mutable borrow due to mutable iterators.
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
    use std::mem::needs_drop;
    use std::ptr::drop_in_place;

    fn get_example() -> Vec<Vec<usize>> {
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
        puz.set_initial(get_example());
        puz
    }

    #[test]
    fn remove_pencil_test () {
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
    fn single_cand () {
        let mut puz = Puzzle::new();
        puz.set_initial(vec![
            vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![0, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        let expected: Vec<Vec<usize>> = vec![
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

        let res = single_candidate(&mut puz, true);

        assert!(res.contains(&(10, 7)));
        assert!(res.contains(&(18, 1)));

        // This puzzle is solved by 100% single candidates.
        for (act, exp) in puz.cells.iter().zip(expected.iter().flatten()){
            assert_eq!(act.num(), *exp);
        }


    }
}