use crate::{Element, NUM_CELLS};
use crate::errors::SudError;

/// This is used to generate the vector to initially set the puzzle, along with convert the vector to desired output
pub trait PuzInput {
    fn as_input (&self) -> Result<Vec<Element>, SudError>;
}

impl PuzInput for Vec<Vec<Element>> {
    // Covert a 2-D vector to 1-D
    fn as_input (&self) -> Result<Vec<Element>, SudError> {
        let one: Vec<Element> = self.iter().flatten().cloned().collect();
        if one.len() == NUM_CELLS{
            Ok(one)
        } else {
            Err(SudError::InputParse)
        }
    }
}

/// This only works when the puzzle size is the normal size of 9x9 or less, as each digit is parsed.
/// and assumes a base 10 number.  If using a larger puzzle, use other methods to develop the input.
impl PuzInput for &str {
    fn as_input (&self) -> Result<Vec<Element>, SudError> {
        let RADIX = 10;
        let v = self.chars().into_iter().map(|n| n.to_digit(RADIX).unwrap_or(0) as Element).collect::<Vec<Element>>();
        if v.len() == NUM_CELLS {
            Ok(v)
        } else {
            Err(SudError::InputParse)
        }
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn TwoDVec () {
        let two: Vec<Vec<Element>> = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let expected: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0,
            8, 0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6,
            0, 6, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9
        ];

        assert_eq!(two.as_input().unwrap(), expected);
        let two: Vec<Vec<Element>> = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7],
        ];

        match two.as_input() {
            Err(SudError::InputParse) => (),
            _ => assert!(false),

        }
    }

    #[test]
    fn string_in () {
        let two: Vec<Vec<Element>> = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let str = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

        let expected: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0,
            8, 0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6,
            0, 6, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9
        ];

        assert_eq!(str.as_input().unwrap(), expected);

        let str = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..9";
        match str.as_input() {
            Err(SudError::InputParse) => (),
            _ => assert!(false),

        }

    }
}