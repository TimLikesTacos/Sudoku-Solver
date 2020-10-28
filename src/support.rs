use crate::grid::constants::*;

// convert 2-D coordinate to 1-D
pub fn get_cell(row: usize, col: usize) -> usize {
    row * MAX_NUM + col
}

// covert 1-D coordinate to 2-D
pub fn index_to_row(index: usize) -> usize {
    index / MAX_NUM
}
pub fn index_to_col(index: usize) -> usize {
    index % MAX_NUM
}

pub fn index_to_box(index: usize) -> usize {
    let (r, c) = (index_to_row(index), index_to_col(index));
    (r / BOX_DIMEN) * BOX_DIMEN + (c / BOX_DIMEN)
}

/// Returns the index of the first (upper-left) cell in the box, which
/// is the first element in the iterator
pub fn start_of_box(box_num: usize) -> usize {
    let row = (box_num / BOX_DIMEN) * BOX_DIMEN;
    let col = (box_num % BOX_DIMEN) * BOX_DIMEN;
    get_cell(row, col)
}

/// Given the box number and the nth cell in that box, returns index in puzzle of the cell.
/// Useful for getting coordinates from iterators
pub fn index_from_box(box_num: usize, index: usize) -> usize {
    let start = start_of_box(box_num);
    let row_cont = (index / BOX_DIMEN) * MAX_NUM;
    start + row_cont + (index % BOX_DIMEN)
}

pub fn index_from_row(row: usize, index: usize) -> usize {
    row * MAX_NUM + index
}

pub fn index_from_col(col: usize, index: usize) -> usize {
    index * MAX_NUM + col
}

#[cfg(test)]
mod support_tests {

    use super::*;
    #[test]
    fn index_from_box_test() {
        assert_eq!(index_from_box(0, 0), 0);
        assert_eq!(index_from_box(0, 8), 20);
        assert_eq!(index_from_box(1, 0), 3);
        assert_eq!(index_from_box(8, 8), 80);
        assert_eq!(index_from_box(2, 5), 17);
    }
}
