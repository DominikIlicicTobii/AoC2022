use day6::*;

fn main() {
    let contents = match read_input() {
        Ok(contents) => contents,
        Err(error) => panic!("{error}"),
    };

    let parsed_input = parse_input(&contents);

    let answer = get_answer(&parsed_input);

    assert_eq!(answer, CORRECT_ANSWER);
}
