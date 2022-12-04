use std::fs::File;
use std::io::prelude::*;

type Tokens = Vec<(CleaningRange, CleaningRange)>;

pub fn parse_input(mut file: File) -> Tokens {
    let mut contents = String::with_capacity(10000);
    file.read_to_string(&mut contents).unwrap();

    let mut tokens = Vec::<(CleaningRange, CleaningRange)>::new();

    for line in contents.lines() {
        let parsed: Vec<i32> = line
            .split(&[',', '-'])
            .map(|c| c.parse().unwrap())
            .collect();

        assert_eq!(parsed.len(), 4);
        if let [first, second, third, firth] = parsed[..] {
            tokens.push(((first, second), (third, firth)));
        }
    }

    tokens
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    let mut sum = 0;

    for token in tokens {
        if overlaps_at_all(&token.0, &token.1) {
            sum += 1;
        }
    }

    sum
}

type CleaningRange = (i32, i32);

fn overlaps_at_all(first: &CleaningRange, second: &CleaningRange) -> bool {
    if first.1 - first.0 > second.1 - second.0 {
        if second.0 >= first.0 && second.0 <= first.1 || second.1 >= first.0 && second.1 <= first.1
        {
            return true;
        }
    } else {
        if first.0 >= second.0 && first.0 <= second.1 || first.1 >= second.0 && first.1 <= second.1
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps_at_all() {
        let first = (1, 3);
        let second = (3, 4);
        assert_eq!(overlaps_at_all(&second, &first), true);
    }
}

pub const CORRECT_ANSWER: i32 = 839;
