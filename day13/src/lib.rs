use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ListStart,
    ListEnd,
    Number(i32),
}

type Tokens = Vec<Vec<Token>>;

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
    let mut all_tokens = Tokens::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut tokens = Vec::<Token>::new();
        let mut ch_iter = line.chars().into_iter();
        let mut number = String::new();
        while let Some(ch) = ch_iter.next() {
            match ch {
                '[' => {
                    assert!(number.is_empty());
                    tokens.push(Token::ListStart)
                }
                ']' => {
                    if !number.is_empty() {
                        if number == "a" {
                            tokens.push(Token::Number(10));
                        } else {
                            tokens.push(Token::Number(number.parse().unwrap()));
                        }
                        number.clear();
                    }
                    tokens.push(Token::ListEnd)
                }
                ',' => {
                    if !number.is_empty() {
                        if number == "a" {
                            tokens.push(Token::Number(10));
                        } else {
                            tokens.push(Token::Number(number.parse().unwrap()));
                        }
                        number.clear();
                    }
                }
                ch => number.push(ch),
            }
        }
        all_tokens.push(tokens);
    }

    all_tokens
}

pub fn get_answer(tokens: &Tokens) -> usize {
    let mut tokens = tokens.clone();
    let two = Vec::from([
        Token::ListStart,
        Token::ListStart,
        Token::Number(2),
        Token::ListEnd,
        Token::ListEnd,
    ]);
    let six = Vec::from([
        Token::ListStart,
        Token::ListStart,
        Token::Number(6),
        Token::ListEnd,
        Token::ListEnd,
    ]);

    tokens.push(two.clone());
    tokens.push(six.clone());

    let mut resursive_built = Vec::new();

    for token in tokens {
        resursive_built.push(build_recursion(&token));
    }

    resursive_built.sort();

    let two_index = resursive_built
        .binary_search(&build_recursion(&two.clone()))
        .unwrap();

    let six_index = resursive_built
        .binary_search(&build_recursion(&six.clone()))
        .unwrap();

    (two_index + 1) * (six_index + 1)
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Element {
    Number(i32),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(s), Self::Number(o)) => s.partial_cmp(o),
            (Self::List(s), Self::List(o)) => {
                let mut s = s.iter();
                let mut o = o.iter();

                loop {
                    match (s.next(), o.next()) {
                        (Some(a), Some(b)) => {
                            if let Some(ordering) = a.partial_cmp(b) {
                                if ordering != Ordering::Equal {
                                    return Some(ordering);
                                }
                            }
                        }
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                    }
                }
            }
            (Self::Number(a), Self::List(_)) => {
                Self::List(Vec::from([Self::Number(*a)])).partial_cmp(other)
            }
            (Self::List(_), Self::Number(b)) => {
                self.partial_cmp(&Self::List(Vec::from([Self::Number(*b)])))
            }
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn build_recursion(tokens: &Vec<Token>) -> Element {
    let mut vec_of_elements = Vec::new();

    // find start_index and end_index of outermost list

    let mut start_index: Option<usize> = None;
    let mut list_stack = 0;

    for i in 0..tokens.len() {
        match tokens[i] {
            Token::ListStart => {
                list_stack += 1;
                if list_stack == 1 {
                    start_index = Some(i);
                }
            }
            Token::ListEnd => {
                list_stack -= 1;
                if list_stack == 0 {
                    // found inner list, build Element from it !
                    let mut tmp = Vec::new();
                    for j in start_index.unwrap() + 1..i {
                        tmp.push(tokens[j].clone());
                    }
                    vec_of_elements.push(build_recursion(&tmp));
                    start_index = None;
                }
            }
            Token::Number(n) => {
                if list_stack == 0 {
                    // only push number if this is not inner list
                    vec_of_elements.push(Element::Number(n));
                    continue;
                }
            }
        }
    }

    Element::List(vec_of_elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_resursion_1() {
        let tokens = Vec::from([Token::Number(1), Token::Number(2), Token::Number(3)]);
        let expected = Element::List(Vec::from([
            Element::Number(1),
            Element::Number(2),
            Element::Number(3),
        ]));
        assert_eq!(expected, build_recursion(&tokens));
    }

    #[test]
    fn test_build_resursion_2() {
        let tokens = Vec::from([
            Token::Number(1),
            Token::ListStart,
            Token::Number(2),
            Token::Number(3),
            Token::ListEnd,
            Token::Number(4),
        ]);
        let expected = Element::List(Vec::from([
            Element::Number(1),
            Element::List(Vec::from([Element::Number(2), Element::Number(3)])),
            Element::Number(4),
        ]));
        assert_eq!(expected, build_recursion(&tokens));
    }

    #[test]
    fn test_build_resursion_3() {
        let tokens = Vec::from([
            Token::Number(1),
            Token::ListStart,
            Token::Number(2),
            Token::Number(3),
            Token::ListStart,
            Token::Number(4),
            Token::Number(5),
            Token::ListEnd,
            Token::ListEnd,
            Token::Number(6),
        ]);
        let expected = Element::List(Vec::from([
            Element::Number(1),
            Element::List(Vec::from([
                Element::Number(2),
                Element::Number(3),
                Element::List(Vec::from([Element::Number(4), Element::Number(5)])),
            ])),
            Element::Number(6),
        ]));
        assert_eq!(expected, build_recursion(&tokens));
    }

    #[test]
    fn test_build_resursion_4() {
        let tokens = Vec::from([
            Token::Number(1),
            Token::ListStart,
            Token::Number(2),
            Token::Number(3),
            Token::ListEnd,
            Token::Number(4),
            Token::ListStart,
            Token::Number(5),
            Token::Number(6),
            Token::ListEnd,
            Token::Number(7),
        ]);
        let expected = Element::List(Vec::from([
            Element::Number(1),
            Element::List(Vec::from([Element::Number(2), Element::Number(3)])),
            Element::Number(4),
            Element::List(Vec::from([Element::Number(5), Element::Number(6)])),
            Element::Number(7),
        ]));
        assert_eq!(expected, build_recursion(&tokens));
    }

    #[test]
    fn test_pair_1() {
        let first = Vec::from([
            Token::ListStart,
            Token::Number(1),
            Token::Number(1),
            Token::Number(3),
            Token::Number(1),
            Token::Number(1),
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::Number(1),
            Token::Number(1),
            Token::Number(5),
            Token::Number(1),
            Token::Number(1),
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), true);
    }

    #[test]
    fn test_pair_2() {
        let first = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::Number(1),
            Token::ListEnd,
            Token::ListStart,
            Token::Number(2),
            Token::Number(3),
            Token::Number(4),
            Token::ListEnd,
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::Number(1),
            Token::ListEnd,
            Token::Number(4),
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), true);
    }

    #[test]
    fn test_pair_3() {
        let first = Vec::from([Token::ListStart, Token::Number(9), Token::ListEnd]);
        let second = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::Number(8),
            Token::Number(7),
            Token::Number(6),
            Token::ListEnd,
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), false);
    }

    #[test]
    fn test_pair_4() {
        let first = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::Number(4),
            Token::Number(4),
            Token::ListEnd,
            Token::Number(4),
            Token::Number(4),
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::Number(4),
            Token::Number(4),
            Token::ListEnd,
            Token::Number(4),
            Token::Number(4),
            Token::Number(4),
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), true);
    }

    #[test]
    fn test_pair_5() {
        let first = Vec::from([
            Token::ListStart,
            Token::Number(7),
            Token::Number(7),
            Token::Number(7),
            Token::Number(7),
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::Number(7),
            Token::Number(7),
            Token::Number(7),
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), false);
    }

    #[test]
    fn test_pair_6() {
        let first = Vec::from([Token::ListStart, Token::ListEnd]);
        let second = Vec::from([Token::ListStart, Token::Number(3), Token::ListEnd]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), true);
    }

    #[test]
    fn test_pair_7() {
        let first = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::ListStart,
            Token::ListEnd,
            Token::ListEnd,
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::ListStart,
            Token::ListEnd,
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), false);
    }

    #[test]
    fn test_pair_8() {
        let first = Vec::from([
            Token::ListStart,
            Token::Number(1),
            Token::ListStart,
            Token::Number(2),
            Token::ListStart,
            Token::Number(3),
            Token::ListStart,
            Token::Number(4),
            Token::ListStart,
            Token::Number(5),
            Token::Number(6),
            Token::Number(7),
            Token::ListEnd,
            Token::ListEnd,
            Token::ListEnd,
            Token::ListEnd,
            Token::Number(8),
            Token::Number(9),
            Token::ListEnd,
        ]);
        let second = Vec::from([
            Token::ListStart,
            Token::Number(1),
            Token::ListStart,
            Token::Number(2),
            Token::ListStart,
            Token::Number(3),
            Token::ListStart,
            Token::Number(4),
            Token::ListStart,
            Token::Number(5),
            Token::Number(6),
            Token::Number(0),
            Token::ListEnd,
            Token::ListEnd,
            Token::ListEnd,
            Token::ListEnd,
            Token::Number(8),
            Token::Number(9),
            Token::ListEnd,
        ]);

        assert_eq!(build_recursion(&first) < build_recursion(&second), false);
    }
}

pub const CORRECT_ANSWER: usize = 25800;
