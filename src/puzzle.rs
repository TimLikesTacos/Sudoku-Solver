use crate::flag::*;
use crate::grid::*;
use crate::square::*;
use crate::*;
use crate::conv_input_output::PuzInput;
use crate::constants::*;

#[derive(Debug, Clone)]
pub enum Solutions<G> {
    None,
    One(Grid<G>),
    Multi(Vec<Grid<G>>),
}
pub struct Puzzle<Unsolv:  SquareFlagTrait, Solv: SquareTrait> {
    grid: Grid<Unsolv>,
    solution: Solutions<Solv>,
}

pub struct SimplePuzzle<Square: SquareTrait> {
    grid: Grid<Square>,
    solution: Solutions<Square>,
}

pub trait PuzzleTrait<'a> {
    type Solution;
    fn new_from_vec(input_vec: Vec<u8>) -> Self;
    fn get_solution(&self) -> Self::Solution;
    fn is_solved(&'a self) -> bool;
    //fn set_initial(&mut self, initial: Vec<u8>);
    fn initial_flags(&mut self);

}

impl <'a, S: SquareTrait> PuzzleTrait <'a> for SimplePuzzle<S> {
    type Solution = Solutions<S>;

    fn new_from_vec(input_vec: Vec<u8>) -> Self {
        let mut puz: SimplePuzzle<S> = SimplePuzzle {
            grid: Grid::new(),
            solution: Solutions::None,
        };
        input_vec
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, &c)| puz.grid[i].setv(S::input_convert(c)))
            .all(|_| true);

        puz
    }

    fn get_solution(&self) -> Self::Solution {
        self.solution.clone()
    }

    fn is_solved(&'a self) -> bool {
        match &self.solution {
            Solutions::None => false,
            Solutions::One(g) => *g == self.grid,
            Solutions::Multi(sols) => sols.contains(&self.grid),
        }
    }

    /// Not used for Simplepuzzle that uses squares without flags.
    fn initial_flags (&mut self) {}
}

impl<
        'a,
        Unsolv: 'a + SquareFlagTrait + PartialEq<Solv>,
        Solv: 'a + SquareTrait + PartialEq<Unsolv> + From<&'a Unsolv> + Copy,
    > PuzzleTrait<'a> for Puzzle<Unsolv, Solv>
{
    type Solution = Solutions<Solv>;
    fn new_from_vec(input_vec: Vec<u8>) -> Puzzle<Unsolv, Solv> {
        let mut puz: Puzzle<Unsolv, Solv> = Puzzle {
            grid: Grid::new(),
            solution: Solutions::None,
        };
        input_vec
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, &c)| puz.grid[i].setv(Unsolv::input_convert(c)))
            .all(|_| true);
        puz.initial_flags();
        puz
    }

    fn get_solution(&self) -> Self::Solution {
        self.solution.clone()
    }

    fn is_solved(&'a self) -> bool {
        match &self.solution {
            Solutions::None => false,
            Solutions::One(g) => *g == Grid::from(&self.grid),
            Solutions::Multi(sols) => sols.contains(&Grid::from(&self.grid)),
        }
    }

    fn initial_flags (&mut self) {
        for i in 0..MAX_NUM {
            let vals:Vec<Unsolv::Value> = self.grid.single_iterator(i).
                fold(Vec::new(), |mut acc, x| {acc.push(x.getv()); acc});
            self.grid[i].initial_setp(&vals);
        }

    }

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
}

mod grid_and_puzzle_tests {
    use super::*;

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

    #[test]
    fn row_iter_test() {
        let example:Vec<Vec<u8>> = get_example();

        let example_copy = example.clone();

        let mut res: SimplePuzzle<SimpleSquare<u16>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());

        let iter = res.grid.row_iter(0);
        for (exp, res) in example_copy[0].iter().zip(iter) {
            assert_eq!(res.getv(), *exp as u16);
        }

        let mut iter = res.grid.row_iter(72);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().getv(), 7);
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert!(iter.next().is_none());
        let iter1 = res.grid.row_iter(0);
        let iter2 = res.grid.row_iter(6);
        for (c1, c2) in iter1.zip(iter2){
            assert_eq!(c1, c2);
        }

    }
    #[test]
    fn col_iter_test() {
        let example = get_example();

        let mut res: SimplePuzzle<SimpleSquare<u32>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());
        let iter = res.grid.col_iter(0);
        let expected = [5, 6, 0, 8, 4, 7, 0, 0, 0];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.getv(), *exp);
        }

        let mut iter = res.grid.col_iter(17);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().getv(), 5);
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert!(iter.next().is_none());
    }

    #[test]
    fn box_iter_test() {
        let example = get_example();

        let mut res: SimplePuzzle<FlagSquare<u16, Flag<u16>>> = SimplePuzzle::new_from_vec(example.as_input().unwrap());

        let iter = res.grid.box_iter(60);
        let expected = [2, 8, 0, 0, 0, 5, 0, 7, 9];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.getv(), *exp);
        }
        let iter = res.grid.box_iter(0);
        let expected = [5, 3, 0, 6, 0, 0, 0, 9, 8];
        for (exp, res) in expected.iter().zip(iter) {
            assert_eq!(res.getv(), *exp);
        }

        let mut iter = res.grid.col_iter(17);

        iter.nth(6);

        assert_eq!(iter.next().unwrap().getv(), 5);
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert!(iter.next().is_none());
    }

    #[test]
    fn mut_iter_test() {

        let mut puz: Puzzle<FlagSquare<u16, Flag<u16>>, SimpleSquare<u16>> = Puzzle::new_from_vec(get_example().as_input().unwrap());
        let mut iter = puz.grid.col_iter_mut(2);
        iter.next();
        let cell = iter.next().unwrap();
        assert_eq!(cell.getv(), 0);
        assert_eq!(cell.getp().count(), 3);
        cell.getp_mut().remove_num(2);
        assert_eq!(cell.getp().count(), 2);
        iter.next();
        assert_eq!(cell.getp().count(), 2);
        assert_eq!(cell.getv(), 0);
    }
}
