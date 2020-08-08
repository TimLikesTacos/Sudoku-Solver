# Sudoku Solver with Multiple Solutions

This library solves sudoku puzzles with 0, 1, or multiple solutions.

The puzzle is set using a 2-D vector of `usize`.
The solutions are returned using a `vec` of 2-D vectors.  
Each element in the soltuion vector is a 2-D vector of 'usize' representing a solved Sudoku puzzle.

## Example
``` 
let example: Vec<Vec<usize>> = vec![
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

        let expected1: Vec<Vec<usize>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 2, 7],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 7, 2],
        ];

        let expected2: Vec<Vec<usize>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 7, 2],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 2, 7],
        ];
        
        let res = Puzzle::new().set_initial(example).solve();
        assert_eq!(res.len(), 2);
        if res[0] == expected1 {
            assert_eq!(res[0], expected1);
            assert_eq!(res[1], expected2);
        } else {
            assert_eq!(res[0], expected2);
            assert_eq!(res[1], expected1);
        }
```
