use std::fs::File;

use day5::*;

fn main() {
    let (mut stacks, tokens) = parse_input(File::open("input/input.txt").unwrap());

    let answer = get_answer(&mut stacks, &tokens);

    assert_eq!(answer, CORRECT_ANSWER);
}
