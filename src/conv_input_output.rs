use crate::errors::SudError;
use crate::errors::SudError::OutputParse;
use crate::grid::constants::*;

/// This is used to generate the vector to initially set the puzzle, along with convert the vector to desired output
pub trait PuzInput<V> {
    fn as_input(&self) -> Result<Vec<V>, SudError>;
}

impl<V: Clone> PuzInput<V> for Vec<Vec<V>> {
    // Covert a 2-D vector to 1-D
    fn as_input(&self) -> Result<Vec<V>, SudError> {
        let one: Vec<V> = self.iter().flatten().cloned().collect();
        if one.len() == NUM_CELLS {
            Ok(one)
        } else {
            Err(SudError::InputParse)
        }
    }
}

/// This only works when the puzzle size is the normal size of 9x9 or less, as each digit is parsed.
/// and assumes a base 10 number.  If using a larger puzzle, use other methods to develop the input.
impl<V: From<u32>> PuzInput<V> for &str {
    fn as_input(&self) -> Result<Vec<V>, SudError> {
        let radix = 10;
        let v = self
            .chars()
            .into_iter()
            .map(|n| V::from(n.to_digit(radix).unwrap_or(0)))
            .collect::<Vec<V>>();
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
    fn as_2d_vec(&self) -> Result<Vec<Vec<u32>>, SudError>;
}

impl<T: Into<u32> + From<u8> + Clone + PartialEq> PuzOutput for Vec<T> {
    fn as_string(&self) -> Result<String, SudError> {
        let radix = 10;
        let str = self
            .iter()
            .map(|dig| {
                if *dig == T::from(0u8) {
                    '.'
                } else {
                    std::char::from_digit(dig.clone().into(), radix).unwrap()
                }
            })
            .collect::<String>();

        if str.len() == NUM_CELLS {
            Ok(str)
        } else {
            Err(SudError::OutputParse)
        }
        // Ok(String::from("hi"))
    }

    fn as_2d_vec(&self) -> Result<Vec<Vec<u32>>, SudError> {
        let mut full: Vec<Vec<u32>> = Vec::new();
        let mut row: Vec<u32> = Vec::new();
        for (ind, cell) in self.iter().enumerate() {
            row.push(cell.clone().into());
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
        let two: Vec<Vec<u16>> = vec![
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

        let expected: Vec<u16> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(two.as_input().unwrap(), expected);
        let two: Vec<Vec<u16>> = vec![
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

        let expected: Vec<u32> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        let res: Vec<u32> = str.as_input().unwrap();
        assert_eq!(res, expected);

        let str =
            "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..9";
        let strin: Result<Vec<u32>, SudError> = str.as_input();
        match strin {
            Err(SudError::InputParse) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn two_d_vec_out() {
        let two: Vec<Vec<u32>> = vec![
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

        let vec: Vec<u16> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(vec.as_2d_vec().unwrap(), two);

        let vec: Vec<u32> = vec![
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

        let vec: Vec<u16> = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(vec.as_string().unwrap(), str);

        let vec: Vec<u16> = vec![
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
