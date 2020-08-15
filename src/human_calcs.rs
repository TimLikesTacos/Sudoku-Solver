use crate::*;

/// Fills in single candidates, returns vector of what positions were filled.
/// If optional 'fill' parameter is true, will mark the position, else, just calculates how many and where.
fn single_candidate (puz: &mut Puzzle, fill: bool) -> Vec<(usize, usize)> {

    // This vector contains coordinate and value for cells that are solved for.  This can be used to debug
    // or function modified to return this vector for display.
    let mut count: Vec<(usize, usize)> = Vec::new();

    // Iterate through each cell, get its pencil marks, see if there is only one candidate.
    // If only one, update it, remove associated pencil marks, update count vector
    let single_cands = (0..NUM_CELLS).into_iter()
        .filter(|i| puz.cells[*i].get_penciled().len() == 1)
        // Mark the cell that has one candidate, update pencil marks for others
        .collect::<Vec<usize>>();

    for i in single_cands {
        assert!(puz.cells[i].get_penciled().len() == 1);
        let val = puz.cells[i].get_penciled().drain().collect::<Vec<usize>>()[0];
        puz.cells[i].set(val);
        assert!(puz.cells[i].get_penciled().is_empty());
        for mut cell in puz.single_iterator_mut(i) {
            cell.get_penciled().remove(&val);
            count.push((i, val));
        }
    }
    count
}

#[cfg(tests)]
mod human_method_tests {
    use super::*;
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


    #[test]
    fn single_cand () {
        let mut puz = Puzzle::new();
        puz.set_initial(vec![
            vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);
        let res = single_candidate(&mut puz, true);
        dbg!(&res);
        assert!(res.contains(&(10, 7)));
        assert!(res.contains(&(18, 1)));


    }
}