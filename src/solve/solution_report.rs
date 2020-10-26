use crate::grid::Grid;

pub struct SolutionReport<S> {
    sol: Solution<S>,
    data: Vec<SolveTech>
}

pub enum Solution <S>{
    None,
    One(Grid<S>),
    Multi(Vec<Grid<S>>),
}

pub enum SolveTech {
    Guesses(usize),
    SingleCandidates(usize),
    LockedCandidates(usize),
    HiddenTuples(u8, usize),
}

impl <S> SolutionReport <S>{
    pub fn new (solution: Solution<S>, in_data: Vec<SolveTech>) -> SolutionReport<S>{
        SolutionReport{
            sol: solution,
            data: in_data,
        }
    }

    pub fn num_solutions (&self) -> usize {
        match &self.sol {
            Solution::None => 0,
            Solution::One(_) => 1,
            Solution::Multi(v) => v.len(),
        }
    }

    pub fn tech_iter (&self) -> impl Iterator<Item=&SolveTech> {
        self.data.iter()
    }
}