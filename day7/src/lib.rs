use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Command {
    CD(String),
    LS(Vec<Entry>),
}

type Tokens = Vec<Command>;

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

    let mut ls = Vec::new();
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(line) => {
                if line[0..4] == *"$ cd" {
                    if !ls.is_empty() {
                        tokens.push(Command::LS(ls.clone()));
                        ls.clear();
                    }
                    let s = String::from(line);
                    let cd = Command::CD(s[5..].to_string());
                    tokens.push(cd);
                    continue;
                } else if line == "$ ls" || line == "" {
                    if !ls.is_empty() {
                        tokens.push(Command::LS(ls.clone()));
                        ls.clear();
                    }
                    continue;
                } else {
                    if line[0..3] == *"dir" {
                        ls.push(Entry::DIR);
                        continue;
                    } else {
                        let mut split = line.split_whitespace();
                        let size = split.next().unwrap().parse::<i32>().unwrap();
                        ls.push(Entry::FILE(size));
                        continue;
                    }
                }
            }
            None => {
                assert_eq!(ls.is_empty(), false);
                tokens.push(Command::LS(ls.clone()));
                break;
            }
        }
    }

    tokens
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    let mut num = 0;

    let mut dirs = HashMap::new();
    let mut last_dir = String::new();

    for token in tokens {
        match token {
            Command::CD(name) => {
                if name == ".." {
                    assert_eq!(last_dir.remove(last_dir.len() - 1), '/');
                    loop {
                        match last_dir.pop() {
                            Some('/') => {
                                last_dir.push('/');
                                break;
                            }
                            Some(_) => continue,
                            None => break,
                        }
                    }
                } else {
                    last_dir += (name.to_owned() + "/").to_string().as_str();
                    assert_eq!(dirs.insert(last_dir.clone(), 0), None);
                }
            }
            Command::LS(entries) => {
                for entry in entries {
                    match entry {
                        Entry::DIR => continue,
                        Entry::FILE(file_size) => {
                            let Some(size) = dirs.get_mut(&last_dir) else {unreachable!("No directory entry!")};
                            *size += *file_size;
                        }
                    }
                }
            }
        }
    }

    let mut dirs_recursive_size = dirs.clone();

    for dir in dirs.keys() {
        let subdirs: Vec<String> = dirs
            .clone()
            .into_keys()
            .filter(|k| k.starts_with(dir) && k != dir)
            .collect();

        let Some(v) = dirs_recursive_size.get_mut(dir) else { unreachable!() };
        for subdir in subdirs {
            match dirs.get(&subdir) {
                Some(a) => *v += a,
                None => {}
            }
        }
    }

    for (_, size) in dirs_recursive_size {
        if size < 100000 {
            num += size;
        }
    }

    num
}

#[derive(Debug, Clone)]
pub enum Entry {
    FILE(i32),
    DIR,
}

pub const CORRECT_ANSWER: i32 = 1243729;
