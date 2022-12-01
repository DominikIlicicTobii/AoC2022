use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

type Calori = i32;

#[derive(Debug, PartialEq)]
enum LineParser {
    ELF(Calori),
    EMPTY,
}

impl FromStr for LineParser {
    type Err = ();

    fn from_str(input: &str) -> Result<LineParser, Self::Err> {
        if input == "" {
            return Ok(LineParser::EMPTY);
        } else {
            let calori = input.parse();
            match calori {
                Ok(c) => Ok(LineParser::ELF(c)),
                Err(_) => Err(()),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Elf {
    calories: Vec<i32>,
    len: usize,
    sum: i32,
}

pub fn parse_input(mut file: File) -> Vec<Elf> {
    let mut contents = String::with_capacity(10000);
    file.read_to_string(&mut contents).unwrap();

    let mut elfs: Vec<Elf> = Vec::new();
    let mut calories: Vec<i32> = Vec::new();
    for l in contents.lines() {
        let parsed_line = LineParser::from_str(l);
        match parsed_line {
            Ok(line) => match line {
                LineParser::EMPTY => {
                    elfs.push(Elf {
                        calories: calories.clone(),
                        len: calories.len(),
                        sum: calories.iter().sum(),
                    });
                    calories.clear();
                }
                LineParser::ELF(calori) => calories.push(calori),
            },
            Err(_) => panic!("WARNING: Line was not parsed"),
        }
    }

    elfs
}

pub fn get_answer(elfs: &Vec<Elf>) -> i32 {
    let mut tmp: Vec<Elf> = elfs.clone();

    tmp.sort_by_key(|e| e.sum);

    tmp.iter().rev().take(3).map(|e| e.sum).sum()
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

pub const CORRECT_ANSWER: i32 = 201524;
