use crate::flag::*;
use crate::grid::*;
use crate::square::*;
use crate::*;

#[derive(Debug, Clone)]
pub enum Solutions<G> {
    None,
    One(Grid<G>),
    Multi(Vec<Grid<G>>),
}
pub struct Puzzle<Unsolv: SquareFlagTrait, Solv: SquareTrait> {
    grid: Grid<Unsolv>,
    solution: Solutions<Solv>,
}

pub trait PuzzleTrait<'a> {
    type Solution;
    fn new_from_vec(input_vec: Vec<u8>) -> Self;
    fn get_solution(&self) -> Self::Solution;
    fn is_solved(&'a self) -> bool;
    //fn set_initial(&mut self, initial: Vec<u8>);
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
