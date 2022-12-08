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

    let mut highest_scenic_score = 0;

    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let height = tokens[i][j];
            let (l, r, t, b) = (
                scenic_score(height, &extract_side(&tokens, i, j, Side::LEFT), Side::LEFT),
                scenic_score(
                    height,
                    &extract_side(&tokens, i, j, Side::RIGHT),
                    Side::RIGHT,
                ),
                scenic_score(height, &extract_side(&tokens, i, j, Side::TOP), Side::TOP),
                scenic_score(
                    height,
                    &extract_side(&tokens, i, j, Side::BOTTOM),
                    Side::BOTTOM,
                ),
            );
            let candidate = l * r * t * b;
            if candidate > highest_scenic_score {
                highest_scenic_score = candidate;
            }
        }
    }

    highest_scenic_score
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

fn scenic_score(height: u32, trees: &[u32], side: Side) -> i32 {
    let trees = {
        if side == Side::LEFT || side == Side::TOP {
            let mut tmp = Vec::from(trees);
            tmp.reverse();
            tmp
        } else {
            Vec::from(trees)
        }
    };

    let mut iter = trees.iter().enumerate();
    loop {
        match iter.next() {
            Some((i, tree)) => {
                if *tree >= height {
                    return 1 + i as i32;
                }
            }
            None => return trees.len() as i32,
        }
    }
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
        assert_eq!(scenic_score(1, &[3, 5, 1], Side::LEFT), 1);
        assert_eq!(scenic_score(5, &[3, 5, 1], Side::LEFT), 2);
        assert_eq!(scenic_score(6, &[3, 5, 1], Side::LEFT), 3);
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

pub const CORRECT_ANSWER: i32 = 284648;
