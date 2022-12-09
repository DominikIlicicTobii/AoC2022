use core::panic;
use std::fs::File;
use std::io::prelude::*;
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
        head: (1000, 1000),
        tail: (1000, 1000),
        visited: Vec::from([(1000, 1000)]),
    };

    field.render(&tokens);

    field.visited.len() as i32
}
struct Field {
    size: i32,
    spots: char,
    head: (i32, i32),
    tail: (i32, i32),
    visited: Vec<(i32, i32)>,
}

impl Field {
    fn move_tail(&mut self) {
        let i_diff = (self.head.0 - self.tail.0).abs();
        let j_diff = (self.head.1 - self.tail.1).abs();

        if i_diff <= 1 && j_diff <= 1 {
            return;
        }

        if i_diff == 0 {
            self.tail.1 = if self.head.1 > self.tail.1 {
                self.head.1 - 1
            } else {
                self.head.1 + 1
            };
            if !self.visited.contains(&(self.tail.0, self.tail.1)) {
                self.visited.push((self.tail.0, self.tail.1));
            }
            return;
        }

        if j_diff == 0 {
            self.tail.0 = if self.head.0 > self.tail.0 {
                self.head.0 - 1
            } else {
                self.head.0 + 1
            };
            if !self.visited.contains(&(self.tail.0, self.tail.1)) {
                self.visited.push((self.tail.0, self.tail.1));
            }
            return;
        }

        if j_diff == 2 {
            // move diagonally
            self.tail.0 = self.head.0;
            self.tail.1 = if self.head.1 > self.tail.1 {
                self.head.1 - 1
            } else {
                self.head.1 + 1
            };
            if !self.visited.contains(&(self.tail.0, self.tail.1)) {
                self.visited.push((self.tail.0, self.tail.1));
            }
        }

        if i_diff == 2 {
            // move diagonally
            self.tail.0 = if self.head.0 > self.tail.0 {
                self.head.0 - 1
            } else {
                self.head.0 + 1
            };
            self.tail.1 = self.head.1;
            if !self.visited.contains(&(self.tail.0, self.tail.1)) {
                self.visited.push((self.tail.0, self.tail.1));
            }
        }
    }

    fn render(&mut self, tokens: &Tokens) {
        for token in tokens {
            match token {
                Token::L(steps) => {
                    for _ in 1..=*steps {
                        self.head.0 -= 1;
                        self.move_tail();
                    }
                }
                Token::D(steps) => {
                    for _ in 1..=*steps {
                        self.head.1 -= 1;
                        self.move_tail();
                    }
                }
                Token::U(steps) => {
                    for _ in 1..=*steps {
                        self.head.1 += 1;
                        self.move_tail();
                    }
                }
                Token::R(steps) => {
                    for _ in 1..=*steps {
                        self.head.0 += 1;
                        self.move_tail();
                    }
                }
            }
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut spots = Vec::<Vec<char>>::new();

        for j in 0..self.size {
            let mut tmp = Vec::<char>::new();
            for i in 0..self.size {
                if self.head.0 == i && self.head.1 == j {
                    tmp.push('H');
                    continue;
                }
                if self.tail.0 == i && self.tail.1 == j {
                    tmp.push('T');
                    continue;
                }
                if self.visited.contains(&(i, j)) {
                    tmp.push('s');
                    continue;
                }
                tmp.push(self.spots);
            }
            spots.push(tmp);
        }

        spots.reverse();
        let mut string = String::new();
        for vec_ch in spots {
            for ch in vec_ch {
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
    fn test_field_to_string() {
        let field = Field {
            size: 10,
            spots: '.',
            head: (3, 1),
            tail: (2, 1),
            visited: Vec::from([(0, 0), (1, 0)]),
        };

        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\n..........\n..........\n..........\n..TH......\nss........\n".to_string()
        );
    }

    #[test]
    fn test_field_moving_right() {
        let mut field = Field {
            size: 10,
            spots: '.',
            head: (0, 0),
            tail: (0, 0),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([Token::R(5)]);
        field.render(&tokens);

        assert_eq!(field.head, (5, 0));
        assert_eq!(field.tail, (4, 0));
        assert_eq!(field.visited, [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\n..........\n..........\n..........\n..........\nssssTH....\n".to_string()
        );
    }

    #[test]
    fn test_field_moving_up() {
        let mut field = Field {
            size: 10,
            spots: '.',
            head: (0, 0),
            tail: (0, 0),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([Token::U(3)]);
        field.render(&tokens);

        assert_eq!(field.head, (0, 3));
        assert_eq!(field.tail, (0, 2));
        assert_eq!(field.visited, [(0, 0), (0, 1), (0, 2)]);
        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\n..........\nH.........\nT.........\ns.........\ns.........\n".to_string()
        );
    }

    #[test]
    fn test_field_moving_right_up_right() {
        let mut field = Field {
            size: 10,
            spots: '.',
            head: (0, 0),
            tail: (0, 0),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([Token::R(3), Token::U(4), Token::R(5)]);
        field.render(&tokens);

        assert_eq!(field.head, (8, 4));
        assert_eq!(field.tail, (7, 4));
        assert_eq!(
            field.visited,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (4, 4),
                (5, 4),
                (6, 4),
                (7, 4)
            ]
        );
        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\n....sssTH.\n...s......\n...s......\n...s......\nsss.......\n".to_string()
        );
    }

    #[test]
    fn test_field_moving_right_up_left() {
        let mut field = Field {
            size: 10,
            spots: '.',
            head: (0, 0),
            tail: (0, 0),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([Token::R(3), Token::U(4), Token::L(3)]);
        field.render(&tokens);

        assert_eq!(field.head, (0, 4));
        assert_eq!(field.tail, (1, 4));
        assert_eq!(
            field.visited,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 4),
                (1, 4)
            ]
        );
        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\nHTs.......\n...s......\n...s......\n...s......\nsss.......\n".to_string()
        );
    }

    #[test]
    fn test_field_moving_right_up_down_right() {
        let mut field = Field {
            size: 10,
            spots: '.',
            head: (0, 0),
            tail: (0, 0),
            visited: Vec::from([(0, 0)]),
        };

        let tokens = Vec::from([Token::R(3), Token::U(4), Token::D(3), Token::R(3)]);
        field.render(&tokens);

        assert_eq!(field.head, (6, 1));
        assert_eq!(field.tail, (5, 1));
        assert_eq!(
            field.visited,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (4, 1),
                (5, 1)
            ]
        );
        assert_eq!(
            field.to_string(),
            "..........\n..........\n..........\n..........\n..........\n..........\n...s......\n...s......\n...ssTH...\nsss.......\n".to_string()
        );
    }
}

pub const CORRECT_ANSWER: i32 = 6212;
