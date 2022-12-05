use std::fs::File;
use std::io::prelude::*;

type Tokens = Vec<(i32, usize, usize)>;
type Stack = Vec<u8>;

pub fn parse_input(mut file: File) -> (Vec<Stack>, Tokens) {
    let mut contents = String::with_capacity(10000);
    file.read_to_string(&mut contents).unwrap();

    let mut tokens = Tokens::new();
    let mut reading_stacks = true;
    let mut stacks = [
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
    ];
    let mut raw_stacks = Vec::<String>::new();
    for line in contents.lines() {
        if reading_stacks && !line.contains("1") {
            let line = line.to_string();
            assert_eq!(line.len(), 36);
            raw_stacks.push(line.clone());
            continue;
        }
        if line.contains("1") && reading_stacks {
            reading_stacks = false;
            raw_stacks.reverse();
            for line in &raw_stacks {
                for i in 0..9 {
                    let tmp = line[i * 4..i * 4 + 4].to_string();
                    if tmp.contains('[') {
                        stacks[i].push(tmp.as_bytes()[1]);
                    }
                }
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let split: Vec<&str> = line.split(' ').collect();
        let stack = split[1].parse().unwrap();
        let from: i32 = split[3].parse::<i32>().unwrap() - 1;
        let to: i32 = split[5].parse::<i32>().unwrap() - 1;
        tokens.push((stack, from as usize, to as usize));
    }

    (stacks.to_vec(), tokens)
}

pub fn get_answer(stacks: &mut Vec<Stack>, tokens: &Tokens) -> String {
    for (n, from, to) in tokens {
        let mut tmp = Vec::<u8>::new();
        for _ in 0..*n {
            let p = stacks[*from].pop().unwrap();
            tmp.push(p);
        }
        tmp.reverse();
        for p in tmp {
            stacks[*to].push(p);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        let a = char::from_u32(*stack.last().unwrap() as u32).unwrap();
        result.push(a);
    }

    result
}

pub const CORRECT_ANSWER: &str = "RNRGDNFQG";
