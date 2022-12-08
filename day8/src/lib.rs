use std::fs::File;
use std::io::prelude::*;

type Tokens = Vec<Vec<u32>>;

pub fn read_input() -> Result<String, String> {
    let Ok(mut file) = File::open("input/input.txt") else {
        return Err(String::from("File failed to open!")) };

    let mut contents = String::with_capacity(10000);

    match file.read_to_string(&mut contents) {
        Ok(_) => return Ok(contents),
        Err(_) => return Err(String::from("File failed to be read to String!")),
    }
}

pub fn parse_input(input: &String) -> Tokens {
    let mut tokens = Tokens::new();

    for line in input.lines() {
        let mut vec = Vec::new();
        for char in line.chars() {
            vec.push(char.to_digit(10).unwrap());
        }
        tokens.push(vec);
    }

    tokens
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    assert_eq!(tokens.len(), tokens[0].len());
    let size = tokens.len();

    let mut num = 4 * size as i32 - 4;

    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let height = tokens[i][j];
            if is_largest(height, &extract_side(&tokens, i, j, Side::LEFT))
                || is_largest(height, &extract_side(&tokens, i, j, Side::RIGHT))
                || is_largest(height, &extract_side(&tokens, i, j, Side::TOP))
                || is_largest(height, &extract_side(&tokens, i, j, Side::BOTTOM))
            {
                num += 1;
                continue;
            }
        }
    }

    num
}

fn extract_side(tokens: &Vec<Vec<u32>>, i: usize, j: usize, side: Side) -> Vec<u32> {
    let row = {
        if side == Side::LEFT || side == Side::RIGHT {
            tokens[i].clone()
        } else {
            let mut row = Vec::new();
            for r in tokens {
                row.push(r[j]);
            }
            row
        }
    };

    let (from, to) = {
        if side == Side::LEFT {
            (0, j)
        } else if side == Side::RIGHT {
            (j + 1, row.len())
        } else if side == Side::TOP {
            (0, i)
        } else {
            (i + 1, row.len())
        }
    };

    let trees = &row[from..to];
    return Vec::from(trees);
}

fn is_largest(height: u32, trees: &[u32]) -> bool {
    for tree in trees {
        if *tree >= height {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_largest() {
        assert_eq!(is_largest(1, &[3, 5, 1]), false);
        assert_eq!(is_largest(5, &[3, 5, 1]), false);
        assert_eq!(is_largest(6, &[3, 5, 1]), true);
    }

    #[test]
    fn test_extract_side() {
        let trees = Vec::from([
            Vec::from([2, 0, 0, 1, 1, 6]),
            Vec::from([0, 0, 1, 1, 2, 9]),
            Vec::from([3, 1, 1, 2, 1, 2]),
            Vec::from([3, 4, 3, 2, 2, 3]),
            Vec::from([1, 0, 2, 3, 3, 6]),
            Vec::from([5, 2, 3, 9, 7, 2]),
        ]);

        assert_eq!(extract_side(&trees, 3, 2, Side::LEFT), Vec::from([3, 4]));
        assert_eq!(
            extract_side(&trees, 3, 2, Side::RIGHT),
            Vec::from([2, 2, 3])
        );
        assert_eq!(extract_side(&trees, 3, 2, Side::TOP), Vec::from([0, 1, 1]));
        assert_eq!(extract_side(&trees, 3, 2, Side::BOTTOM), Vec::from([2, 3]));
    }
}

pub const CORRECT_ANSWER: i32 = 1733;
