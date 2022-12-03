use std::fs::File;

use day3::*;

fn main() {
    let parsed_input = parse_input(File::open("input/input.txt").unwrap());

    let answer = get_answer(&parsed_input);

    assert_eq!(answer, CORRECT_ANSWER);
}
