use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    L(i32),
    D(i32),
    U(i32),
    R(i32),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(input: &str) -> Result<Token, ()> {
        let split: Vec<&str> = input.split_whitespace().collect();
        if split.len() != 2 {
            panic!("Input line not as expected!")
        }
        let (left, right) = (split[0], split[1]);
        match left {
            "L" => Ok(Token::L(right.parse().unwrap())),
            "D" => Ok(Token::D(right.parse().unwrap())),
            "U" => Ok(Token::U(right.parse().unwrap())),
            "R" => Ok(Token::R(right.parse().unwrap())),
            _ => unreachable!("Pick cant be parsed!"),
        }
    }
}

type Tokens = Vec<Token>;

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
        tokens.push(Token::from_str(line).unwrap());
    }

    tokens
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    let mut field = Field {
        size: 2000,
        spots: '.',
        rope: Vec::from([
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
            (1000, 1000),
        ]),
        visited: Vec::from([(1000, 1000)]),
    };

    field.render(&tokens);

    field.visited.len() as i32
}
struct Field {
    size: i32,
    spots: char,
    rope: Vec<(i32, i32)>,
    visited: Vec<(i32, i32)>,
}

fn move_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let mut tail = tail;
    let i_diff = (head.0 - tail.0).abs();
    let j_diff = (head.1 - tail.1).abs();

    if i_diff <= 1 && j_diff <= 1 {
        return tail;
    }

    if i_diff == 0 {
        tail.1 = if head.1 > tail.1 {
            head.1 - 1
        } else {
            head.1 + 1
        };
        return tail;
    }

    if j_diff == 0 {
        tail.0 = if head.0 > tail.0 {
            head.0 - 1
        } else {
            head.0 + 1
        };
        return tail;
    }

    if j_diff == 2 && i_diff == 2 {
        tail.1 = if head.1 > tail.1 {
            head.1 - 1
        } else {
            head.1 + 1
        };
        tail.0 = if head.0 > tail.0 {
            head.0 - 1
        } else {
            head.0 + 1
        };
        return tail;
    }

    if j_diff == 2 {
        // move diagonally
        tail.0 = head.0;
        tail.1 = if head.1 > tail.1 {
            head.1 - 1
        } else {
            head.1 + 1
        };
    }

    if i_diff == 2 {
        // move diagonally
        tail.0 = if head.0 > tail.0 {
            head.0 - 1
        } else {
            head.0 + 1
        };
        tail.1 = head.1;
    }

    return tail;
}

impl Field {
    fn move_rope(&mut self, prev_rope: &Vec<(i32, i32)>) {
        for i in 1..prev_rope.len() {
            self.rope[i] = move_tail(prev_rope[i], self.rope[i - 1]);
        }

        if !self.visited.contains(&self.rope[prev_rope.len() - 1]) {
            self.visited.push(self.rope[prev_rope.len() - 1]);
        }
    }

    fn render(&mut self, tokens: &Tokens) {
        let mut prev_rope;
        for token in tokens {
            match token {
                Token::L(steps) => {
                    for _ in 1..=*steps {
                        prev_rope = self.rope.clone();
                        self.rope[0].0 -= 1;
                        self.move_rope(&prev_rope);
                    }
                }
                Token::D(steps) => {
                    for _ in 1..=*steps {
                        prev_rope = self.rope.clone();
                        self.rope[0].1 -= 1;
                        self.move_rope(&prev_rope);
                    }
                }
                Token::U(steps) => {
                    for _ in 1..=*steps {
                        prev_rope = self.rope.clone();
                        self.rope[0].1 += 1;
                        self.move_rope(&prev_rope);
                    }
                }
                Token::R(steps) => {
                    for _ in 1..=*steps {
                        prev_rope = self.rope.clone();
                        self.rope[0].0 += 1;
                        self.move_rope(&prev_rope);
                    }
                }
            }
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut spots: Vec<Vec<char>> = {
            let mut tmp_spots: Vec<Vec<char>> = Vec::new();
            for _ in 0..self.size {
                let tmp = iter::repeat(self.spots).take(self.size as usize).collect();
                tmp_spots.push(tmp);
            }

            tmp_spots
        };

        for visit in &self.visited {
            spots[visit.1 as usize][visit.0 as usize] = 's';
        }

        for i in (1..self.rope.len()).rev() {
            spots[self.rope[i].1 as usize][self.rope[i].0 as usize] =
                char::from_digit(i as u32, 10).unwrap();
        }
        spots[self.rope[0].1 as usize][self.rope[0].0 as usize] = 'H';

        let mut string = String::with_capacity(self.size as usize * self.size as usize);

        spots.reverse();

        for vec in spots {
            for ch in vec {
                string.push(ch);
            }
            string.push('\n');
        }

        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_moving_twn() {
        let mut field = Field {
            size: 5,
            spots: '.',
            rope: Vec::from([
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
            ]),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([
            Token::R(4),
            Token::U(4),
            Token::L(3),
            Token::D(1),
            Token::R(4),
            Token::D(1),
            Token::L(5),
            Token::R(2),
        ]);
        field.render(&tokens);

        assert_eq!(field.to_string(), ".....\n.....\n.1H3.\n.5...\n6....\n");
    }
}

pub const CORRECT_ANSWER: i32 = 2522;
