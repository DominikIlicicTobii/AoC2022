use core::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    line: (i32, i32),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let a: Vec<&str> = s.split(',').collect();
        assert_eq!(a.len(), 2);
        Ok(Token {
            line: (a[0].parse().unwrap(), a[1].parse().unwrap()),
        })
    }
}

type Tokens = Vec<Vec<Token>>;

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
    input
        .split('\n')
        .filter(|p| !p.is_empty())
        .map(|f| f.split('>').map(|g| Token::from_str(g).unwrap()).collect())
        .collect()
}

pub fn get_answer(tokens: &Tokens) -> usize {
    let mut cave = Cave::build_cave(&tokens);
    let mut num = 0;
    loop {
        if !cave.pour_sand((cave.sand_origin.0, cave.sand_origin.1)) {
            break;
        } else {
            num += 1;
        }
    }

    num + 1
}

struct Cave {
    cave: Vec<Vec<char>>,
    sand_origin: (usize, usize),
}

enum Move {
    Stay,
    Down,
    DownLeft,
    DownRight,
    Void,
}

impl Cave {
    fn where_to_move(&self, position: (usize, usize)) -> Move {
        if position.0 >= self.cave.len() - 1
            || position.1 >= self.cave[0].len() - 1
            || position.1 <= 0
        {
            return Move::Void;
        }

        let ch = self.cave[position.0][position.1];
        let ch_down = self.cave[position.0 + 1][position.1];
        let ch_downleft = self.cave[position.0 + 1][position.1 - 1];
        let ch_downright = self.cave[position.0 + 1][position.1 + 1];

        if ch == '✺' && ch_down == '✺' && ch_downleft == '✺' && ch_downright == '✺' {
            return Move::Void;
        }
        if ch_down == ' ' {
            return Move::Down;
        }

        if ch_downleft == ' ' {
            return Move::DownLeft;
        }

        if ch_downright == ' ' {
            return Move::DownRight;
        }

        Move::Stay
    }

    fn pour_sand(&mut self, position: (usize, usize)) -> bool {
        let mut position = position;
        loop {
            match self.where_to_move(position) {
                Move::Void => {
                    return false;
                }
                Move::Stay => {
                    if self.cave[position.0][position.1] == ' ' {
                        self.cave[position.0][position.1] = '✺';
                    }
                    return true;
                }
                Move::Down => position.0 += 1,
                Move::DownLeft => position = (position.0 + 1, position.1 - 1),
                Move::DownRight => position = (position.0 + 1, position.1 + 1),
            }
        }
    }

    fn build_cave(tokens: &Tokens) -> Cave {
        let (i_min, i_max, j_min, j_max) = {
            let mut i_min: Option<i32> = None;
            let mut i_max: Option<i32> = None;
            let mut j_min: Option<i32> = None;
            let mut j_max: Option<i32> = None;

            for arr in tokens {
                for Token { line } in arr {
                    let (i, j) = *line;

                    if i_min == None || i_min > Some(i) {
                        i_min = Some(i);
                    }
                    if i_max == None || i_max < Some(i) {
                        i_max = Some(i);
                    }

                    if j_min == None || j_min > Some(j) {
                        j_min = Some(j);
                    }
                    if j_max == None || j_max < Some(j) {
                        j_max = Some(j);
                    }
                }
            }

            if j_min > Some(0) {
                j_min = Some(0);
            }

            assert_ne!(i_min, None);
            assert_ne!(i_max, None);
            assert_ne!(j_min, None);
            assert_ne!(j_max, None);

            (
                i_min.unwrap(),
                i_max.unwrap(),
                j_min.unwrap(),
                j_max.unwrap(),
            )
        };
        let mut cave = Vec::new();
        for _ in 0..=(j_max - j_min) {
            let mut tmp = Vec::new();
            for _ in 0..=(i_max - i_min) {
                tmp.push(' ');
            }
            cave.push(tmp.clone());
        }

        for wall in tokens {
            add_walls(&mut cave, &wall, i_min);
        }

        let sand_origin = (0, 500 - (i_min as usize));
        cave[0][500 - (i_min as usize)] = '✺';

        Self { cave, sand_origin }
    }
}

fn add_walls(cave: &mut Vec<Vec<char>>, wall: &Vec<Token>, i_min: i32) {
    let mut window = wall.windows(2);
    loop {
        match window.next() {
            Some(w) => {
                let p0 = (w[0].line.0 - i_min, w[0].line.1);
                let p1 = (w[1].line.0 - i_min, w[1].line.1);

                let i_min = {
                    if p0.0 < p1.0 {
                        p0.0
                    } else {
                        p1.0
                    }
                };
                let i_max = {
                    if p0.0 > p1.0 {
                        p0.0
                    } else {
                        p1.0
                    }
                };
                let j_min = {
                    if p0.1 < p1.1 {
                        p0.1
                    } else {
                        p1.1
                    }
                };
                let j_max = {
                    if p0.1 > p1.1 {
                        p0.1
                    } else {
                        p1.1
                    }
                };

                for j in j_min..j_max + 1 {
                    for i in i_min..i_max + 1 {
                        cave[j as usize][i as usize] = '█';
                    }
                }
            }
            None => return,
        }
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cave = String::new();
        for row in &self.cave {
            for ch in row {
                cave.push(*ch);
            }
            cave.push('\n');
        }
        write!(f, "{}", cave)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_resursion_1() {
        assert_eq!(1, 1);
    }
}

pub const CORRECT_ANSWER: usize = 22499;
