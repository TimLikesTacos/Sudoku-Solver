use crate::*;
use crate::grid::*;
use crate::square::*;
use crate::flag::*;

pub struct Puzzle <Unsolv: SquareFlagTrait, Solv: SquareTrait> {
    grid: Grid<Unsolv>,
    solution: Grid<Solv>,
}

pub trait  PuzzleTrait {

    type Solution;
    fn new_from_vec (input_vec: Vec<usize>) -> Self;
    fn get_solution (&self) -> Self::Solution;
    fn is_solved (&self) -> bool;

}

impl <Unsolved: SquareFlagTrait, Solv: SquareTrait> PuzzleTrait for Puzzle <Unsolved, Solv> {
    type Solution = Grid<Solv>;
    fn new_from_vec (input_vec: Vec<usize>) -> Puzzle<Unsolved, Solv> {
        Puzzle {
            grid: Grid::set_initial(input_vec),
            solution: Grid::new(),
        }
    }

    fn get_solution (&self) -> Self::Solution {self.solution.clone()}

    fn is_solved (&self) -> bool {
        self.grid.equal_values(&self.solution)
    }
}