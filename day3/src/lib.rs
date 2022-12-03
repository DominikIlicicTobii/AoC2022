use std::fs::File;
use std::io::prelude::*;

type Tokens = Vec<String>;

pub fn parse_input(mut file: File) -> Tokens {
    let mut contents = String::with_capacity(10000);
    file.read_to_string(&mut contents).unwrap();

    contents.split_whitespace().map(|s| s.to_string()).collect()
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    let mut sum = 0;

    for line in tokens {
        assert_eq!(line.len() % 2, 0);
        let (first, second) = line.split_at(line.len() / 2);
        if let Some(duplicate) = get_duplicate(first, second) {
            sum += get_item_value(duplicate);
        }
    }

    sum
}

fn get_item_value(c: char) -> i32 {
    let value = c as i32;
    if value >= 64 && value <= 90 {
        return value - 38;
    } else {
        return value - 96;
    }
}

fn get_duplicate(first: &str, second: &str) -> Option<char> {
    for c_first in first.chars() {
        for c_second in second.chars() {
            if c_first == c_second {
                return Some(c_first);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let line = "dWlhclDHdFvDCCDfFq";
        assert_eq!(line.len() % 2, 0);
        let (first, second) = line.split_at(line.len() / 2);
        let duplicate = get_duplicate(first, second);

        assert_eq!(duplicate, Some('D'));
    }

    #[test]
    fn test_item_value() {
        assert_eq!(get_item_value('A'), 27);
        assert_eq!(get_item_value('a'), 1);
    }
}

pub const CORRECT_ANSWER: i32 = 8105;
