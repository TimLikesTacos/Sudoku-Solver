use crate::grid::*;
use crate::solve::brute::BruteForce;
use crate::solve::solution_report::*;
use crate::sq_element::flag::Flag;
use crate::sq_element::*;
use crate::square::*;

#[derive(Debug, Clone)]
pub enum Solutions<G: Square> {
    None,
    One(Grid<G>),
    Multi(Vec<Grid<G>>),
}

pub struct Puzzle<S: Square>
where
    Grid<S>: NewGrid,
{
    pub(crate) board: Grid<S>,
    pub(crate) solution: SolutionReport<S>,
}

type SimpleSudoku = Puzzle<SimpleSquare<IntValue>>;
type RegSudoku = Puzzle<FlagSquare<IntValue, Flag<u16>>>;
type RegFlagSudoku = Puzzle<FlagSquare<Flag<u16>, Flag<u16>>>;
//type FourByFour    = Puzzle<FlagSquare<FlagType<u32>, FlagType<u32>>>;

pub trait PuzzleTrait<S: Square>: BruteForce<S> {
    fn new(input_vec: Vec<u8>) -> Self;
    fn is_solved(&self) -> bool;
    //fn set_initial(&mut self, initial: Vec<u8>);
    fn reset_square(&mut self, index: usize);
}

impl<S: Square> PuzzleTrait<S> for Puzzle<S>
where
    Grid<S>: NewGrid,
{
    fn new(input_vec: Vec<u8>) -> Self {
        let g: Grid<S>;
        g = Grid::new(input_vec);
        let mut p = Puzzle {
            board: g,
            solution: SolutionReport::default(),
        };
        let sol = p.brute_force_solve();
        p.solution = sol;
        p
    }

    fn is_solved(&self) -> bool {
        unimplemented!()
    }

    fn reset_square(&mut self, index: usize) {
        unimplemented!()
    }
}

// impl <Unsolv, Solv> PuzzleTrait for Puzzle <Unsolv, Solv>
//     where Solv: Square + PartialEq<Unsolv>, Unsolv: Square + From<u8> + PartialEq<Solv> {
//
//     type BoardType = Grid<Unsolv>;
//     type SolutionType = Solutions<Solv>;
//
//     fn new(input_vec: Vec<u8>) -> Self {
//         let g: Grid<Unsolv> = Grid::new(input_vec);
//         if Unsolv::has_flags(){
//             g.set_initial_flags();
//         }
//         let sol = Self::get_solution(&g);
//         Puzzle {
//             board: g,
//             solution: sol,
//         }
//     }
//
//     fn set_initial_flags(&mut self) {
//         // do the following for each square.
//         for square in 0..MAX_NUM {
//             let p = self.board[square].set_initial(self.board.single_iterator(square));
//         }
//     }
//     fn get_solution(original: &Self::BoardType) -> Self::SolutionType{
//         let mut copy = original.clone();
//         unimplemented!()
//         //copy.brute_force()
//     }
//
//     fn is_solved(&self) -> bool {
//         match &self.solution {
//             Solutions::None => false,
//             Solutions::One(s) => if self.board == *s {true} else {false},
//             Solutions::Multi(esses) => esses.iter().any(|x| x == &self.board),
//         }
//     }
//
//
//     fn reset_square(&mut self, index: usize) {
//         unimplemented!()
//     }
// }

//
// impl <'a, S: SquareTrait> PuzzleTrait <'a> for SimplePuzzle<S> {
//     type Solution = Solutions<S>;
//
//     fn from_vec(input_vec: Vec<u8>) -> Self {
//         let mut puz: SimplePuzzle<S> = SimplePuzzle {
//             grid: Grid::new(),
//             solution: Solutions::None,
//         };
//         input_vec
//             .iter()
//             .enumerate()
//             .filter(|(_, &c)| c != 0)
//             .map(|(i, &c)| puz.grid[i].setv(S::input_convert(c)))
//             .all(|_| true);
//
//         puz
//     }
//
//     fn reset_square (&mut self, index:usize) {
//         self.grid[index].reset_value();
//     }
//
//     fn get_solution(&self) -> Self::Solution {
//         self.solution.clone()
//     }
//
//     fn is_solved(&'a self) -> bool {
//         match &self.solution {
//             Solutions::None => false,
//             Solutions::One(g) => *g == self.grid,
//             Solutions::Multi(sols) => sols.contains(&self.grid),
//         }
//     }
//
//     /// Not used for Simplepuzzle that uses squares without flags.
//     fn initial_flags (&mut self) {}
// }
//
// impl<
//         'a,
//         Unsolv: 'a + SquareFlagTrait + PartialEq<Solv>,
//         Solv: 'a + SquareTrait + PartialEq<Unsolv> + From<&'a Unsolv> + Copy,
//     > PuzzleTrait<'a> for Puzzle<Unsolv, Solv>
// {
//     type Solution = Solutions<Solv>;
//     fn from_vec(input_vec: Vec<u8>) -> Puzzle<Unsolv, Solv> {
//         let mut puz: Puzzle<Unsolv, Solv> = Puzzle {
//             board: Grid::new(),
//             solution: Solutions::None,
//         };
//         input_vec
//             .iter()
//             .enumerate()
//             .filter(|(_, &c)| c != 0)
//             .map(|(i, &c)| {puz.board[i].setv(Unsolv::input_convert(c));
//                 puz.board[i].fix();})
//             .all(|_| true);
//         puz.initial_flags();
//         puz
//     }
//
//     fn reset_square (&mut self, index: usize) {
//         if self.board[index].fixed() {
//             return;
//         }
//         let old_num = self.board[index].getv();
//         self.board[index].reset_value();
//         /* Undo all the flags */
//         let col = index_to_col(index);
//         let row = index_to_row(index);
//         let nbox = index_to_box(index);
//
//         // undo the square by updating the flags for all affected squares.
//         for i  in 0..MAX_NUM {
//             let coli = index_from_col(col, i);
//             let rowi = index_from_row (row, i);
//             let boxi = index_from_box(nbox, i);
//
//             let v = self.board.single_iterator(coli).fold(Vec::new(), |mut acc, s| {acc.push(s.getv()); acc});
//             self.board[coli].initial_setp(&v);
//             let v = self.board.single_iterator(rowi).fold(Vec::new(), |mut acc, s| {acc.push(s.getv()); acc});
//             self.board[rowi].initial_setp(&v);
//             let v = self.board.single_iterator(boxi).fold(Vec::new(), |mut acc, s| {acc.push(s.getv()); acc});
//             self.board[boxi].initial_setp(&v);
//
//         }
//
//     }
//
//     fn get_solution(&self) -> Self::Solution {
//         self.solution.clone()
//     }
//
//     fn is_solved(&'a self) -> bool {
//         match &self.solution {
//             Solutions::None => false,
//             Solutions::One(g) => *g == Grid::from(&self.board),
//             Solutions::Multi(sols) => sols.contains(&Grid::from(&self.board)),
//         }
//     }
//
//     fn initial_flags (&mut self) {
//         for i in 0..NUM_CELLS {
//             if self.board[i].fixed() {
//                 continue;
//             }
//             let vals:Vec<Unsolv::Value> = self.board.single_iterator(i).
//                 fold(Vec::new(), |mut acc, x| {acc.push(x.getv()); acc});
//             self.board[i].initial_setp(&vals);
//         }
//
//     }

//Sets a new puzzle using 2-D vector parameter
// fn set_initial(&mut self, initial: Vec<u8>) {
//     initial
//         .iter()
//         .enumerate()
//         .filter(|(_, &c)| c != 0)
//         .map(|(i, &c)| {
//
//             self.grid[i].setv(S::input_convert(c))})
//         .all(|_| true);
//
//     // ret.set_pencil();
// }

// // Assumes that the puzzle has already been initially set
// fn set_pencil(&mut self)  {
//     // For all cells that are not fixed...
//     for i in 0..NUM_CELLS {
//         if self.grid[i].fixed() {
//             continue;
//         }
//         let new_pencil:Vec<S::Value> = self
//             .single_iterator(i)
//             .fold(Vec::new(), |mut acc, x| {acc.push(x.getv()); acc});
//
//         self.grid[i].initial_setp(&new_pencil);
//
//     }
// }

// mod grid_and_puzzle_tests {
//     use super::*;
//
//     fn get_example() -> Vec<Vec<u8>> {
//             vec![
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
//     fn proper_init_value () {
//         let puz: SimplePuzzle<SimpleSquare<u16>> = SimplePuzzle::new_from_vec(get_example().as_input().unwrap());
//         let expected: [u16;9] = [5, 3, 0, 0, 7, 0, 0, 0, 0];
//         assert!(puz.grid.row_iter(3).zip(expected.iter()).all(|(v1, v2)| v1.getv() == *v2));
//
//
//
//         let puz: SimplePuzzle<FlagSquare<u16, Flag<u16>>> = SimplePuzzle::new_from_vec(get_example().as_input().unwrap());
//         let expected: [u16;9] = [5, 3, 0, 0, 7, 0, 0, 0, 0];
//         assert!(puz.grid.row_iter(3).zip(expected.iter()).all(|(v1, v2)| v1.getv() == *v2));
//         let box_expected: [u16;9] = [0, 0, 11, 0, 74, 74, 3, 0, 0];
//
//         //assert!(puz.grid.box_iter(9).zip(box_expected.iter()).all(|(v1, v2)| v1.getp().get_flags() == *v2));
//
//         let puz: Puzzle<FlagSquare<u16, Flag<u16>>, SimpleSquare<u16>> = Puzzle::new_from_vec(get_example().as_input().unwrap());
//         let expected: [u16;9] = [5, 3, 0, 0, 7, 0, 0, 0, 0];
//         assert!(puz.board.row_iter(3).zip(expected.iter()).all(|(v1, v2)| v1.getv() == *v2));
//         let box_expected: [u16;9] = [0, 0, 11, 0, 74, 74, 3, 0, 0];
//        // dbg!(puz.grid.box_iter(9).fold(Vec::new(), |mut acc,x| {acc.push(x.getp()); acc}));
//         assert!(puz.board.box_iter(9).zip(box_expected.iter()).all(|(v1, v2)| v1.getp().get_flags() == *v2));
//
//         let puz: Puzzle<FlagSquare<u16, Flag<u16>>, FlagSquare<u16, Flag<u16>>> = Puzzle::new_from_vec(get_example().as_input().unwrap());
//         let expected: [u16;9] = [5, 3, 0, 0, 7, 0, 0, 0, 0];
//         assert!(puz.board.row_iter(3).zip(expected.iter()).all(|(v1, v2)| v1.getv() == *v2));
//         let box_expected: [u16;9] = [0, 0, 11, 0, 74, 74, 3, 0, 0];
//         assert!(puz.board.box_iter(9).zip(box_expected.iter()).all(|(v1, v2)| v1.getp().get_flags() == *v2));
//     }
//     #[test]
//     fn row_iter_test() {
//         let example:Vec<Vec<u8>> = get_example();
//
//         let example_copy = example.clone();
//
//         let mut res: SimplePuzzle<SimpleSquare<u16>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());
//
//         let iter = res.grid.row_iter(0);
//         for (exp, res) in example_copy[0].iter().zip(iter) {
//             assert_eq!(res.getv(), *exp as u16);
//         }
//
//         let mut iter = res.grid.row_iter(72);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().getv(), 7);
//         assert_eq!(iter.next().unwrap().getv(), 9);
//         assert!(iter.next().is_none());
//         let iter1 = res.grid.row_iter(0);
//         let iter2 = res.grid.row_iter(6);
//         for (c1, c2) in iter1.zip(iter2){
//             assert_eq!(c1, c2);
//         }
//
//     }
//     #[test]
//     fn col_iter_test() {
//         let example = get_example();
//
//         let mut res: SimplePuzzle<SimpleSquare<u32>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());
//         let iter = res.grid.col_iter(0);
//         let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.getv(), *exp);
//         }
//
//         let mut iter = res.grid.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().getv(), 5);
//         assert_eq!(iter.next().unwrap().getv(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn box_iter_test() {
//         let example = get_example();
//
//         let mut res: SimplePuzzle<FlagSquare<u16, Flag<u16>>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());
//
//         let iter = res.grid.box_iter(60);
//         let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.getv(), *exp);
//         }
//         let iter = res.grid.box_iter(0);
//         let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
//         for (exp, res) in expected.iter().zip(iter) {
//             assert_eq!(res.getv(), *exp);
//         }
//
//         let mut iter = res.grid.col_iter(17);
//
//         iter.nth(6);
//
//         assert_eq!(iter.next().unwrap().getv(), 5);
//         assert_eq!(iter.next().unwrap().getv(), 9);
//         assert!(iter.next().is_none());
//     }
//
//     #[test]
//     fn mut_iter_test() {
//
//         let mut puz: Puzzle<FlagSquare<u16, Flag<u16>>, SimpleSquare<u16>> = Puzzle::new_from_vec(get_example().as_input().unwrap());
//         let mut iter = puz.board.col_iter_mut(2);
//         iter.next();
//         let cell = iter.next().unwrap();
//         assert_eq!(cell.getv(), 0);
//         assert_eq!(cell.getp().count(), 3);
//         cell.getp_mut().remove_num(2);
//         assert_eq!(cell.getp().count(), 2);
//         iter.next();
//         assert_eq!(cell.getp().count(), 2);
//         assert_eq!(cell.getv(), 0);
//     }
// }
