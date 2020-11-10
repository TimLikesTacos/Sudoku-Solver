use crate::grid::*;
use crate::sq_element::flag::FlagElement;
use crate::sq_element::sq_element::{FlElement, SqElement};
use crate::square::flag_update::FlagUpdate;
use crate::square::{FlagSquare, SimpleSquare, Square};
use crate::support::*;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Grid<S: Square> {
    pub(crate) grid: Vec<S>,
}

pub trait NewGrid {
    fn new(input_vec: Vec<u8>) -> Self;
}

impl<S: Square> Grid<S> {
    fn new_grid(input_vec: &Vec<u8>) -> Grid<S> {
        Grid {
            grid: input_vec
                .iter()
                .map(|x| {
                    if *x == 0 {
                        S::new(*x, false)
                    } else {
                        S::new(*x, true)
                    }
                })
                .collect(),
        }
    }
}

impl<V: SqElement> NewGrid for Grid<SimpleSquare<V>>
where
    SimpleSquare<V>: Square,
{
    fn new(input_vec: Vec<u8>) -> Self {
        Grid::new_grid(&input_vec)
    }
}

impl<V: SqElement, F: FlElement> NewGrid for Grid<FlagSquare<V, F>>
where
    FlagSquare<V, F>: Square + FlagUpdate,
{
    fn new(input_vec: Vec<u8>) -> Self {
        let mut g: Grid<FlagSquare<V, F>> = Grid::new_grid(&input_vec);
        for i in 0..NUM_CELLS {
            if g[i].fixed {
                continue;
            }
            let it = g.single_iterator(i);
            let mut copy = g[i].clone();
            copy.set_initial(it);
            g[i] = copy;
        }
        g
    }
}
impl<S: Square> Display for Grid<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = fmt::Result::Ok(());
        self.grid
            .iter()
            .enumerate()
            .map(|(i, v)| {
                res = write!(f, "{}", v);
                if i % MAX_NUM == MAX_NUM - 1 {
                    write!(f, "\n").unwrap();
                }
            })
            .all(|_| true);
        res
    }
}

impl<V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy>
    From<Grid<SimpleSquare<V1>>> for Grid<FlagSquare<V2, F>>
where
    FlagSquare<V2, F>: From<SimpleSquare<V1>>,
    SimpleSquare<V1>: Square + Copy,
{
    fn from(other: Grid<SimpleSquare<V1>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(), |mut acc, &x| {
                acc.push(<FlagSquare<V2, F>>::from(x));
                acc
            }),
        }
    }
}

impl<V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy>
    From<Grid<FlagSquare<V2, F>>> for Grid<SimpleSquare<V1>>
where
    SimpleSquare<V1>: Square + From<FlagSquare<V2, F>>,
    FlagSquare<V2, F>: Copy,
{
    fn from(other: Grid<FlagSquare<V2, F>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(), |mut acc, &x| {
                acc.push(<SimpleSquare<V1>>::from(x));
                acc
            }),
        }
    }
}

impl<V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy>
    From<&Grid<SimpleSquare<V1>>> for Grid<FlagSquare<V2, F>>
where
    FlagSquare<V2, F>: From<SimpleSquare<V1>>,
    SimpleSquare<V1>: Square + Copy,
{
    fn from(other: &Grid<SimpleSquare<V1>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(), |mut acc, &x| {
                acc.push(<FlagSquare<V2, F>>::from(x));
                acc
            }),
        }
    }
}

impl<V1: SqElement, V2: SqElement + From<F> + Copy, F: FlElement + From<V2> + Copy>
    From<&Grid<FlagSquare<V2, F>>> for Grid<SimpleSquare<V1>>
where
    SimpleSquare<V1>: Square + From<FlagSquare<V2, F>>,
    FlagSquare<V2, F>: Copy,
{
    fn from(other: &Grid<FlagSquare<V2, F>>) -> Self {
        Grid {
            grid: other.grid.iter().fold(Vec::new(), |mut acc, &x| {
                acc.push(<SimpleSquare<V1>>::from(x));
                acc
            }),
        }
    }
}

impl<S: Square + Clone> Grid<S> {
    // pub fn new(input_vec: Vec<u8>) -> Grid<S>
    // where
    //     S: Square + FlagUpdate,
    // {
    //     let mut g = Grid {
    //         grid: input_vec
    //             .iter()
    //             .map(|x| {
    //                 if *x == 0 {
    //                     S::new(*x, false)
    //                 } else {
    //                     S::new(*x, true)
    //                 }
    //             })
    //             .collect(),
    //     };
    //     if S::has_flags() {
    //         for i in 0..NUM_CELLS {
    //             let it = g.single_iterator(i);
    //             let mut copy = g[i].clone();
    //             copy.set_initial(it);
    //             g[i] = copy;
    //         }
    //     }
    //     g
    // }
    /// Iterate over the entire 1-D row dominate grid vector
    pub fn grid_iter(&self) -> impl Iterator<Item = &S> {
        self.grid.iter()
    }

    /// Iterate mutably over the entire 1-D row dominate grid vector
    pub fn grid_iter_mut(&mut self) -> impl Iterator<Item = &mut S> {
        self.grid.iter_mut()
    }

    /// Iterate starting from the beginning of the row that contains the element called.
    pub fn row_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter()
    }

    /// Iterate starting from the beginning of the column that contains the element called.
    pub fn col_iter(&self, index: usize) -> impl Iterator<Item = &S> {
        let col = index_to_col(index);
        self.grid.iter().skip(col).step_by(MAX_NUM)
    }

    /// Iterate over the box that contains the element called.
    pub fn box_iter(&self, index: usize) -> BoxIter<S> {
        let box_num = index_to_box(index);

        BoxIter {
            it: self.grid.iter().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    /// Iterate mutably starting from the beginning of the row that contains the element called.
    pub fn row_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &'_ mut S> {
        let row = index_to_row(index);
        self.grid[((row * MAX_NUM)..(row * MAX_NUM + MAX_NUM))].iter_mut()
    }

    /// Iterate mutably starting from the beginning of the column that contains the element called.
    pub fn col_iter_mut(&mut self, index: usize) -> impl Iterator<Item = &mut S> {
        let col = index_to_col(index);
        self.grid.iter_mut().skip(col).step_by(MAX_NUM)
    }

    /// Iterate mutably over the box that contains the element called.
    pub fn box_iter_mut<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut S> {
        let box_num = index_to_box(index);

        BoxIterMut {
            it: self.grid.iter_mut().skip(start_of_box(box_num)),
            index: 0,
        }
    }

    /// An iterator that iterates over the row, column, and box that the cell with the parameter `index`.
    /// NOTE: The cell of parameter `index` is iterated over 3 times (once for each iterator).
    /// There is no mutable version of this as it would require 3 mutable borrows at the same time.
    /// todo: create a struct with a slice of the cells and create a mutable iterator to get around borrow issues.
    pub fn single_iterator(&self, index: usize) -> impl Iterator<Item = &'_ S> {
        self.box_iter(index)
            .chain(self.row_iter(index).chain(self.col_iter(index)))
    }
}

impl<V: SqElement + From<F>, F: FlElement + From<V>> Grid<FlagSquare<V, F>>
where
    FlagSquare<V, F>: Square<Type = V> + FlagUpdate<FlagElement = F>,
{
    pub(crate) fn set_value_update_flags<IN: SqElement>(&mut self, index: usize, value: IN)
    where
        V: From<IN>,
        F: From<IN>,
    {
        let f_remove = F::from(value);
        self[index].set(V::from(value));
        self[index].remove_flag(FlElement::max());
        self.row_iter_mut(index)
            .map(|s| s.remove_flag(f_remove))
            .all(|_| true);
        self.col_iter_mut(index)
            .map(|s| s.remove_flag(f_remove))
            .all(|_| true);
        self.box_iter_mut(index)
            .map(|s| s.remove_flag(f_remove))
            .all(|_| true);
        // self.row_iter_mut(index).map(|s|s.flags -= f_remove).all(|_|true);
        // self.col_iter_mut(index).map(|s|s.flags -= f_remove).all(|_|true);
        // self.box_iter_mut(index).map(|s|s.flags -= f_remove).all(|_|true);
    }

    /// O(9 * MAX_NUM^2). Intensive calculation, avoid when possible
    fn undo_set_and_update(&mut self, index: usize) {
        let value = self[index].value;
        self[index].reset_value();
        let row = index_to_row(index);
        let col = index_to_col(index);
        let boxn = index_to_box(index);
        for n in 0..MAX_NUM {
            let rowi = index_from_row(row, n);
            let coli = index_from_col(col, n);
            let boxi = index_from_box(boxn, n);

            let mut new_f: FlagSquare<V, F> = FlagSquare::new(0, false);
            new_f.set_initial(self.single_iterator(rowi));
            self[rowi].flags = new_f.flags;

            new_f.set_initial(self.single_iterator(coli));
            self[coli].flags = new_f.flags;

            new_f.set_initial(self.single_iterator(boxi));
            self[boxi].flags = new_f.flags;
        }
    }
}

impl<S1: Square, S2: Square + PartialEq<S1>> PartialEq<Grid<S1>> for Grid<S2> {
    fn eq(&self, other: &Grid<S1>) -> bool {
        self.grid_iter()
            .zip(other.grid_iter())
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

// impl<S1: Square, S2: Square> PartialEq<Grid<S2>> for Grid<S1> {
//     fn eq(&self, other: &Grid<S2>) -> bool {
//         unimplemented!()
//     }
// }

impl<S: Square> Index<usize> for Grid<S> {
    type Output = S;
    fn index<'a>(&'a self, i: usize) -> &'a S {
        &self.grid[i]
    }
}

impl<S: Square> IndexMut<usize> for Grid<S> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut S {
        &mut self.grid[i]
    }
}

#[cfg(test)]
mod grid_tests {

    use super::*;
    use crate::sq_element::{Flag, IntValue};
    use std::convert::TryInto;

    fn test_vec() -> Vec<u8> {
        let mut v = Vec::new();
        for n in (0..NUM_CELLS).rev() {
            let n: u8 = n.try_into().unwrap();
            let m: u8 = MAX_NUM.try_into().unwrap();
            v.push(n % m + 1);
        }
        v
    }

    #[test]
    fn new_test() {
        let vec: Vec<u8> = test_vec();
        let grid: Grid<SimpleSquare<IntValue>> = Grid::new(vec);
        let mut iter = grid.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert_eq!(iter.next().unwrap().getv(), 8);
        assert_eq!(iter.next().unwrap().getv(), 7);
        assert_eq!(iter.next().unwrap().getv(), 6);
        assert_eq!(iter.next().unwrap().getv(), 5);
        assert_eq!(iter.next().unwrap().getv(), 4);
        assert_eq!(iter.next().unwrap().getv(), 3);
        assert_eq!(iter.next().unwrap().getv(), 2);
        assert_eq!(iter.next().unwrap().getv(), 1);
        let mut iter = iter.skip(71);
        assert_eq!(iter.next().unwrap().getv(), 1);
        assert_eq!(iter.next(), Option::None);

        let vec: Vec<u8> = test_vec();
        let grid: Grid<FlagSquare<Flag<u16>, Flag<u16>>> = Grid::new(vec);
        let mut iter = grid.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 0b100000000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000000);
        assert_eq!(iter.next().unwrap().getv(), 0b100000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000);
        assert_eq!(iter.next().unwrap().getv(), 0b100);
        assert_eq!(iter.next().unwrap().getv(), 0b10);
        assert_eq!(iter.next().unwrap().getv(), 0b1);
        let mut iter = iter.skip(71);

        assert_eq!(iter.next().unwrap().getv(), 0b1);
        assert_eq!(iter.next(), Option::None);
    }

    #[test]
    fn from_test() {
        let vec: Vec<u8> = test_vec();
        let grid: Grid<FlagSquare<Flag<u16>, Flag<u16>>> = Grid::new(vec);
        let g2: Grid<SimpleSquare<IntValue>> = Grid::from(&grid);
        let mut iter = g2.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 9);
        assert_eq!(iter.next().unwrap().getv(), 8);
        assert_eq!(iter.next().unwrap().getv(), 7);
        assert_eq!(iter.next().unwrap().getv(), 6);
        assert_eq!(iter.next().unwrap().getv(), 5);
        assert_eq!(iter.next().unwrap().getv(), 4);
        assert_eq!(iter.next().unwrap().getv(), 3);
        assert_eq!(iter.next().unwrap().getv(), 2);
        assert_eq!(iter.next().unwrap().getv(), 1);
        let mut iter = iter.skip(71);

        assert_eq!(iter.next().unwrap().getv(), 1);
        assert_eq!(iter.next(), Option::None);
        assert_eq!(grid, g2);

        let g3: Grid<SimpleSquare<Flag<u32>>> = Grid::from(&grid);
        let mut iter = g3.grid_iter();
        assert_eq!(iter.next().unwrap().getv(), 0b100000000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000000);
        assert_eq!(iter.next().unwrap().getv(), 0b100000);
        assert_eq!(iter.next().unwrap().getv(), 0b10000);
        assert_eq!(iter.next().unwrap().getv(), 0b1000);
        assert_eq!(iter.next().unwrap().getv(), 0b100);
        assert_eq!(iter.next().unwrap().getv(), 0b10);
        assert_eq!(iter.next().unwrap().getv(), 0b1);
        let mut iter = iter.skip(71);
        assert_eq!(iter.next().unwrap().getv(), 0b1);

        assert_eq!(iter.next(), Option::None);
        assert_eq!(grid, g2);
        assert_eq!(grid, g3);
        assert_eq!(g2, g3);
    }
}
