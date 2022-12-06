use std::fs::File;
use std::io::prelude::*;

type Tokens = Vec<u8>;

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
    input.as_bytes().to_vec()
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    let mut num = 0;

    'outer: for window in tokens.windows(4) {
        let mut window = window.to_vec();
        window.sort();
        for subwindow in window.windows(2) {
            if subwindow[0] == subwindow[1] {
                num += 1;
                continue 'outer;
            }
        }

        return num + 4;
    }

    unreachable!("There must be answer before end of tokens!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_answer() {
        let tokens = Vec::<u8>::from([3, 5, 1, 7, 8, 1, 9, 7, 2]);
        let answer = get_answer(&tokens);
        assert_eq!(answer, 4);
    }
}

pub const CORRECT_ANSWER: i32 = 1929;
