use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

const DRAW: i32 = 3;
const WIN: i32 = 6;
const LOSS: i32 = 0;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSOR: i32 = 3;

#[derive(Debug, PartialEq)]
pub enum Outcome {
    DRAW,
    WIN,
    LOSS,
}

impl Outcome {
    fn value(self: &Self) -> i32 {
        match *self {
            Self::DRAW => DRAW,
            Self::WIN => WIN,
            Self::LOSS => LOSS,
        }
    }

    fn calculate(opponent: &Pick, you: &Pick) -> Self {
        match opponent {
            Pick::ROCK => match you {
                Pick::ROCK => Self::DRAW,
                Pick::PAPER => Self::WIN,
                Pick::SCISSOR => Self::LOSS,
            },
            Pick::PAPER => match you {
                Pick::ROCK => Self::LOSS,
                Pick::PAPER => Self::DRAW,
                Pick::SCISSOR => Self::WIN,
            },
            Pick::SCISSOR => match you {
                Pick::ROCK => Self::WIN,
                Pick::PAPER => Self::LOSS,
                Pick::SCISSOR => Self::DRAW,
            },
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, ()> {
        match input {
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::WIN),
            "X" => Ok(Outcome::LOSS),
            _ => panic!("Pick cant be parsed!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Pick {
    ROCK,
    PAPER,
    SCISSOR,
}

impl Pick {
    fn value(self: &Self) -> i32 {
        match *self {
            Self::ROCK => ROCK,
            Self::PAPER => PAPER,
            Self::SCISSOR => SCISSOR,
        }
    }

    fn calculate(opponent: &Self, outcome: &Outcome) -> Self {
        match opponent {
            Pick::ROCK => match outcome {
                Outcome::DRAW => Self::ROCK,
                Outcome::WIN => Self::PAPER,
                Outcome::LOSS => Self::SCISSOR,
            },
            Pick::PAPER => match outcome {
                Outcome::DRAW => Self::PAPER,
                Outcome::WIN => Self::SCISSOR,
                Outcome::LOSS => Self::ROCK,
            },
            Pick::SCISSOR => match outcome {
                Outcome::DRAW => Self::SCISSOR,
                Outcome::WIN => Self::ROCK,
                Outcome::LOSS => Self::PAPER,
            },
        }
    }
}

impl FromStr for Pick {
    type Err = ();

    fn from_str(input: &str) -> Result<Pick, ()> {
        match input {
            "A" => Ok(Pick::ROCK),
            "B" => Ok(Pick::PAPER),
            "C" => Ok(Pick::SCISSOR),
            _ => panic!("Pick cant be parsed!"),
        }
    }
}

type Tokens = Vec<(Pick, Outcome)>;

pub fn parse_input(mut file: File) -> Tokens {
    let mut contents = String::with_capacity(10000);
    file.read_to_string(&mut contents).unwrap();

    let mut parsed_input = Tokens::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(split.len(), 2);

        let opponent = Pick::from_str(split[0]).unwrap();
        let outcome = Outcome::from_str(split[1]).unwrap();

        parsed_input.push((opponent, outcome));
    }

    parsed_input
}

pub fn get_answer(tokens: &Vec<(Pick, Outcome)>) -> i32 {
    let mut sum = 0;
    for line in tokens {
        let (opponent, outcome) = line;
        let you = Pick::calculate(opponent, outcome);
        sum += outcome.value() + you.value();
    }
    sum
}

pub const CORRECT_ANSWER: i32 = 12725;
