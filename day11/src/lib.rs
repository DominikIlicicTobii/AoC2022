use std::fs::File;
use std::io::prelude::*;

type Tokens = String;

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
    input.clone()
}

pub fn get_answer(_tokens: &Tokens) -> i64 {
    let mut monkeys = [
        Monkey {
            items: Vec::from([76, 88, 96, 97, 58, 61, 67]),
            inspections: 0,
            operation: |item| item * 19,
            test: |item| {
                if item % 3 == 0 {
                    Throw { monkey: 2, item }
                } else {
                    Throw { monkey: 3, item }
                }
            },
        },
        Monkey {
            items: Vec::from([93, 71, 79, 83, 69, 70, 94, 98]),
            inspections: 0,
            operation: |item| item + 8,
            test: |item| {
                if item % 11 == 0 {
                    Throw { monkey: 5, item }
                } else {
                    Throw { monkey: 6, item }
                }
            },
        },
        Monkey {
            items: Vec::from([50, 74, 67, 92, 61, 76]),
            inspections: 0,
            operation: |item| item * 13,
            test: |item| {
                if item % 19 == 0 {
                    Throw { monkey: 3, item }
                } else {
                    Throw { monkey: 1, item }
                }
            },
        },
        Monkey {
            items: Vec::from([76, 92]),
            inspections: 0,
            operation: |item| item + 6,
            test: |item| {
                if item % 5 == 0 {
                    Throw { monkey: 1, item }
                } else {
                    Throw { monkey: 6, item }
                }
            },
        },
        Monkey {
            items: Vec::from([74, 94, 55, 87, 62]),
            inspections: 0,
            operation: |item| item + 5,
            test: |item| {
                if item % 2 == 0 {
                    Throw { monkey: 2, item }
                } else {
                    Throw { monkey: 0, item }
                }
            },
        },
        Monkey {
            items: Vec::from([59, 62, 53, 62]),
            inspections: 0,
            operation: |item| item * item,
            test: |item| {
                if item % 7 == 0 {
                    Throw { monkey: 4, item }
                } else {
                    Throw { monkey: 7, item }
                }
            },
        },
        Monkey {
            items: Vec::from([62]),
            inspections: 0,
            operation: |item| item + 2,
            test: |item| {
                if item % 17 == 0 {
                    Throw { monkey: 5, item }
                } else {
                    Throw { monkey: 7, item }
                }
            },
        },
        Monkey {
            items: Vec::from([85, 54, 53]),
            inspections: 0,
            operation: |item| item + 3,
            test: |item| {
                if item % 13 == 0 {
                    Throw { monkey: 4, item }
                } else {
                    Throw { monkey: 0, item }
                }
            },
        },
    ];

    let supermodulo = 3 * 11 * 19 * 5 * 2 * 7 * 17 * 13;

    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items {
                let mut new = (monkeys[i].operation)(item);
                monkeys[i].inspections += 1;
                new %= supermodulo;
                let Throw { monkey, item } = (monkeys[i].test)(new);
                monkeys[monkey].items.push(item);
            }
        }
    }

    let mut inspections: Vec<i64> = monkeys.iter().map(|m| m.inspections).collect();

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

type Item = i64;
type MonekyId = usize;

struct Throw {
    monkey: MonekyId,
    item: Item,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    inspections: i64,
    operation: fn(Item) -> Item,
    test: fn(Item) -> Throw,
}

pub const CORRECT_ANSWER: i64 = 54832778815;
