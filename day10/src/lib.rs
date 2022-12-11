use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Noop(i32),
    Addx(i32, i32),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(input: &str) -> Result<Token, ()> {
        let mut split = input.split_whitespace();
        let first = split.next().unwrap();
        let second = split.next();
        match first {
            "noop" => Ok(Token::Noop(1)),
            "addx" => Ok(Token::Addx(2, second.unwrap().parse().unwrap())),
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
    let mut num = 0;
    let mut reg_value: i32 = 1;

    let mut iter = tokens.iter();

    let (mut wait_cycles, mut wait_token) = {
        let token = iter.next().unwrap();
        match token {
            Token::Noop(cycles) => (*cycles, Token::Noop(*cycles)),
            Token::Addx(cycles, awd) => (*cycles, Token::Addx(*cycles, *awd)),
        }
    };

    for cycle in 1..=240 {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            num += cycle * reg_value;
        }

        wait_cycles -= 1;
        if wait_cycles != 0 {
            continue;
        }

        match wait_token {
            Token::Noop(_cycles) => {}
            Token::Addx(_cycles, value) => reg_value += value,
        };

        (wait_cycles, wait_token) = {
            let token = iter.next();
            match token {
                Some(Token::Noop(cycles)) => (*cycles, Token::Noop(*cycles)),
                Some(Token::Addx(cycles, value)) => (*cycles, Token::Addx(*cycles, *value)),
                None => {
                    break;
                }
            }
        };
    }

    num
}

pub const CORRECT_ANSWER: i32 = 14340;
