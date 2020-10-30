use crate::grid::Grid;
use crate::square::Square;
use std::fmt::{Display, Formatter};
use std::fmt;

/// SolutionReport struct holds the solution, which is an enum describing the number of solutions,
/// and a vector of the methods used to solve the puzzle.  Brute force only uses SolveTech::Guesses.
#[derive(Clone, Debug)]
pub struct SolutionReport<S: Square> {
    pub sol: Solution<S>,
    pub data: Vec<SolveTech>,
}

/// Enum for storing the solution to the puzzle.  None if there is no solution, One if there is one, and Multi
/// if more than one.  Multi contains the solutions in a Vector
#[derive(Clone, Debug)]
pub enum Solution<S: Square> {
    None,
    One(Grid<S>),
    Multi(Vec<Grid<S>>),
}

/// Enum for different ways to solve the puzzle.  Guesses is done by brute force solving.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum SolveTech {
    Guesses(usize),
    SingleCandidates(usize),
    LockedCandidates(usize),
    HiddenTuples((usize, usize)),
    PointingTuples((usize, usize)),
}

/* Default impl */
impl <S: Square> Default for SolutionReport<S> {
    fn default() -> Self {
        SolutionReport {
            sol: Solution::None,
            data: Vec::new(),
        }
    }
}

/* Display impl */
impl Display for SolveTech {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SolveTech::Guesses(v) =>  write!(f, "Guesses: {}", v),
            SolveTech::SingleCandidates(v) =>  write!(f, "Single Candidates: {}", v),
            SolveTech::LockedCandidates(v) =>  write!(f, "Locked Candidates: {}", v),
            SolveTech::HiddenTuples((s, v)) =>  write!(f, "Hidden {}-uple: {}", s, v),
            SolveTech::PointingTuples((s, v))=>  write!(f, "Pointing {}-uples: {}", s, v),
        }
    }
}

impl <S: Square> Display for Solution<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const DISPLAY_MAX: u8 = 10;
        match self {
            Solution::None => write!(f, "No solution"),
            Solution::One(v) => write!(f, "{}", v),
            Solution::Multi(vec) =>
                {
                    let mut r = Ok(());
                    for r in vec.iter().take(DISPLAY_MAX as usize)
                        .map(|s| write!(f, "{}", s)){
                        if r.is_err() {
                            return r;
                        }
                    }
                    r
                }

        }
    }
}

impl<S: Square> SolutionReport<S> {
    pub fn new(solution: Solution<S>, in_data: Vec<SolveTech>) -> SolutionReport<S> {
        SolutionReport {
            sol: solution,
            data: in_data,
        }
    }

    pub fn num_solutions(&self) -> usize {
        match &self.sol {
            Solution::None => 0,
            Solution::One(_) => 1,
            Solution::Multi(v) => v.len(),
        }
    }
    pub fn get_solution(&self) -> &Solution<S> {
        &self.sol
    }

    pub fn add_tech(&mut self, tech: SolveTech){
        self.data.push(tech)
    }
    pub fn all_tech_iter(&self) -> impl Iterator<Item = &SolveTech> {
        self.data.iter()
    }
    pub fn tech_iter<T>(&self, tech: fn((T)) -> SolveTech) -> impl Iterator<Item = &SolveTech> {
        self.data
            .iter()
            .filter(|t| if let tech = t { true } else { false })
    }
}


#[cfg(test)]
mod solution_report_tests {
    use super::*;
    use crate::sq_element::IntType;
    use crate::square::SimpleSquare;

    fn sol_tech () {
        let mut a :SolutionReport<SimpleSquare<IntType<u16>>>= SolutionReport{
            sol: Solution::None,
            data: Vec::new(),
        };

        a.add_tech(SolveTech::Guesses(43));
        a.add_tech(SolveTech::HiddenTuples((2, 8)));
        a.add_tech(SolveTech::Guesses(100));

        let mut it = a.tech_iter(SolveTech::Guesses);
        assert_eq!(if let SolveTech::Guesses(n) = it.next().unwrap(){*n}else{0}, 43);
        assert_eq!(if let SolveTech::Guesses(n) = it.next().unwrap(){*n}else{0}, 100);
        assert!(it.next().is_none());

        let mut it = a.tech_iter(SolveTech::PointingTuples);
        assert!(it.next().is_none());

        let mut it = a.tech_iter(SolveTech::HiddenTuples);
        assert_eq!(if let SolveTech::HiddenTuples(n) = it.next().unwrap(){*n}else{(0,0)}, (2, 8));
        assert!(it.next().is_none());
    }
}