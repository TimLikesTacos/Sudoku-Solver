use crate::errors::SudError;
use crate::errors::SudError::OutputParse;
use crate::{Element};
use crate::constants::*;

/// This is used to generate the vector to initially set the puzzle, along with convert the vector to desired output
pub trait PuzInput {
    fn as_input(&self) -> Result<Vec<Element>, SudError>;
}

impl PuzInput for Vec<Vec<Element>> {
    // Covert a 2-D vector to 1-D
    fn as_input(&self) -> Result<Vec<Element>, SudError> {
        let one: Vec<Element> = self.iter().flatten().cloned().collect();
        if one.len() == NUM_CELLS {
            Ok(one)
        } else {
            Err(SudError::InputParse)
        }
    }
}

/// This only works when the puzzle size is the normal size of 9x9 or less, as each digit is parsed.
/// and assumes a base 10 number.  If using a larger puzzle, use other methods to develop the input.
impl PuzInput for &str {
    fn as_input(&self) -> Result<Vec<Element>, SudError> {
        let radix = 10;
        let v = self
            .chars()
            .into_iter()
            .map(|n| n.to_digit(radix).unwrap_or(0) as Element)
            .collect::<Vec<Element>>();
        if v.len() == NUM_CELLS {
            Ok(v)
        } else {
            Err(SudError::InputParse)
        }
    }
}

/// Used to conver the 1D matrix in the puzzle to desired output.
/// Either make into a consolidated string, or a 2D vector
pub trait PuzOutput {
    /// Converts 1-D vector into 1-D consolidated string.  If there is an unsolved cell, it would
    /// display as `.`
    fn as_string(&self) -> Result<String, SudError>;

    /// Conver 1-D vector into 2-D array of type Element.  If there is an unsolved cell, it would
    /// display as `0`
    fn as_2d_vec(&self) -> Result<Vec<Vec<Element>>, SudError>;
}

impl PuzOutput for Vec<Element> {
    fn as_string(&self) -> Result<String, SudError> {
        let radix = 10;
        let str = self
            .iter()
            .map(|dig| {
                if *dig == 0 {
                    '.'
                } else {
                    std::char::from_digit(*dig as u32, radix).unwrap()
                }
            })
            .collect::<String>();

        if str.len() == NUM_CELLS {
            Ok(str)
        } else {
            Err(SudError::OutputParse)
        }
    }

    fn as_2d_vec(&self) -> Result<Vec<Vec<Element>>, SudError> {
        let mut full: Vec<Vec<Element>> = Vec::new();
        let mut row: Vec<Element> = Vec::new();
        for (ind, cell) in self.iter().enumerate() {
            row.push(*cell);
            if ind % MAX_NUM == MAX_NUM - 1 {
                full.push(row.clone());
                row.clear();
            }
        }
        if full.iter().flatten().count() == NUM_CELLS {
            Ok(full)
        } else {
            Err(OutputParse)
        }
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn two_d_vec() {
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
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
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
    fn string_in() {
        let str =
            "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

        let expected: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(str.as_input().unwrap(), expected);

        let str =
            "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..9";
        match str.as_input() {
            Err(SudError::InputParse) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn two_d_vec_out() {
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

        let vec: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(vec.as_2d_vec().unwrap(), two);

        let vec: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 9,
        ];

        match vec.as_2d_vec() {
            Err(SudError::OutputParse) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn string_out() {
        let str =
            "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

        let vec: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(vec.as_string().unwrap(), str);

        let vec: Vec<Element> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 9,
        ];
        match vec.as_string() {
            Err(SudError::OutputParse) => (),
            _ => assert!(false),
        }
    }
}
