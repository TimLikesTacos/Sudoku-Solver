
mod conv_input_output;
mod errors;
mod grid;
mod human_calcs;
mod puzzle;
mod solve;
mod sq_element;
mod square;
mod support;

#[cfg(test)]
mod library_tests {
    use super::*;
    #[test]
    fn start() {
        assert!(true);
    }
}

//
// #[cfg(test)]
// mod tests {
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
//     #[test]
//     fn valid_puz_test() {
//         let example1: Vec<Vec<Element>> = vec![
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
//         let mut test1 = Puzzle::new();
//         test1.set_initial(example1.as_input().unwrap());
//         assert!(test1.is_solved());
//
//         // Same as example 1 but with a zero cell
//         let example2: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 0, 9],
//         ];
//         let mut test2 = Puzzle::new();
//         test2.set_initial(example2.as_input().unwrap());
//         assert!(!test2.is_solved());
//
//         // Same as example 1 but with a double value
//         let example3: Vec<Vec<Element>> = vec![
//             vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
//             vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
//             vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
//             vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
//             vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
//             vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
//             vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
//             vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
//             vec![3, 4, 5, 2, 8, 6, 1, 3, 9],
//         ];
//         let mut test3 = Puzzle::new();
//         test3.set_initial(example3.as_input().unwrap());
//         assert!(!test3.is_solved());
//     }
//
//     #[test]
//     fn mut_iter_test() {
//         let mut puz = Puzzle::new();
//         puz.set_initial(get_example().as_input().unwrap());
//         let mut iter = puz.col_iter_mut(2);
//         iter.next();
//         let cell = iter.next().unwrap();
//         assert_eq!(cell.num(), 0);
//         assert_eq!(cell.penciled.len(), 3);
//         cell.penciled.remove(&2);
//         assert_eq!(cell.penciled.len(), 2);
//         iter.next();
//         assert_eq!(cell.penciled.len(), 2);
//         assert_eq!(cell.num(), 0);
//     }
//     #[test]
//     fn get_box_test() {
//         assert_eq!(index_to_box(10), 0);
//         assert_eq!(index_to_box(26), 2);
//         assert_eq!(index_to_box(30), 4);
//         assert_eq!(index_to_box(80), 8);
//     }
//     #[test]
//     fn row_iter_test() {
//         let example = get_example();
//
//         let example_copy = example.clone();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//         let iter = res.row_iter(0);
//         for (exp, res) in example_copy[0].iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.row_iter(72);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 7);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//         let iter1 = res.row_iter(0);
//         let iter2 = res.row_iter(6);
//         for (c1, c2) in iter1.zip(iter2){
//             assert_eq!(c1, c2);
//         }
//
//     }
//     #[test]
//     fn col_iter_test() {
//         let example = get_example();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//         let iter = res.col_iter(0);
//         let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 5);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn box_iter_test() {
//         let example = get_example();
//
//         let mut res = Puzzle::new();
//         res.set_initial(example.as_input().unwrap());
//
//         let iter = res.box_iter(60);
//         let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//         let iter = res.box_iter(0);
//         let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.num(), *exp);
//         }
//
//         let mut iter = res.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().num(), 5);
//         assert_eq!(iter.next().unwrap().num(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn set_pencil_test() {
//         let example = get_example();
//         let mut puz = Puzzle::new();
//         puz.set_initial(example.as_input().unwrap());
//
//         assert_eq!(
//             puz.cells[1]
//                 .poss_iter()
//                 .collect::<HashSet<&Element>>()
//                 .len(),
//             0
//         );
//         let mut cell2 = puz.cells[2].poss_iter().collect::<HashSet<&Element>>();
//
//         assert_eq!(cell2.len(), 3);
//         let expected: [Element; 3] = [1, 2, 4];
//         for exp in expected.iter() {
//             assert!(cell2.remove(&exp));
//         }
//         assert!(cell2.is_empty());
//
//         let mut cell78 = puz.cells[78].poss_iter().collect::<HashSet<&Element>>();
//         assert_eq!(cell78.len(), 4);
//         let expected: [Element; 4] = [1, 3, 4, 6];
//         for exp in expected.iter() {
//             assert!(cell78.remove(&exp));
//         }
//         assert!(cell78.is_empty());
//     }
// }
