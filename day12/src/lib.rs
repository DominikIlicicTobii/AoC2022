use std::collections::HashMap;
use std::io::prelude::*;
use std::{collections::HashSet, fs::File};

use petgraph::algo::dijkstra;
use petgraph::graph::Graph;

type Tokens = Vec<Vec<u8>>;

pub fn read_input() -> Result<String, String> {
    let Ok(mut file) = File::open("input/input.txt") else {
        return Err(String::from("File failed to open!")) };

    let mut contents = String::with_capacity(10000);

    match file.read_to_string(&mut contents) {
        Ok(_) => return Ok(contents),
        Err(_) => return Err(String::from("File failed to be read to String!")),
    }
}

pub fn parse_input(_input: &String) -> Tokens {
    let mut tokens = Tokens::new();

    let mut tmp = Vec::new();
    for ch in _input.chars() {
        if ch != '\n' {
            tmp.push(ch as u8);
        } else {
            tokens.push(tmp.clone());
            tmp.clear();
        }
    }

    tokens
}

pub fn get_answer(tokens: &Tokens) -> i32 {
    calculate(&tokens).unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    elevation: u8,
    coordinates: (usize, usize),
}

fn calculate(tokens: &Tokens) -> Option<i32> {
    let height = tokens.len();
    let width = tokens[0].len();

    let tokens = tokens.clone();

    let mut edges = HashSet::new();
    let mut g = Graph::new();
    let mut id_to_node_index = HashMap::new();
    let mut node_index_to_id = HashMap::new();

    let mut s_id = 0;
    let mut e_id = 0;

    for i in 0..height {
        for j in 0..width {
            let middle_id = j + i * width;
            let middle_elevation = {
                if tokens[i][j] == 'S' as u8 {
                    s_id = middle_id;
                    'a' as u8
                } else if tokens[i][j] == 'E' as u8 {
                    e_id = middle_id;
                    'z' as u8
                } else {
                    tokens[i][j]
                }
            };

            if j != width - 1 {
                let right_id = middle_id + 1;
                let right_elevation = {
                    if tokens[i][j + 1] == 'S' as u8 {
                        'a' as u8
                    } else if tokens[i][j + 1] == 'E' as u8 {
                        'z' as u8
                    } else {
                        tokens[i][j + 1]
                    }
                };
                if (middle_elevation).abs_diff(right_elevation) <= 1 {
                    if edges.insert((middle_id, right_id)) && edges.insert((right_id, middle_id)) {
                        let a = match id_to_node_index.get(&middle_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(middle_id);
                                id_to_node_index.insert(middle_id, tmp);
                                node_index_to_id.insert(tmp, middle_id);
                                tmp
                            }
                        };
                        let b = match id_to_node_index.get(&right_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(right_id);
                                id_to_node_index.insert(right_id, tmp);
                                node_index_to_id.insert(tmp, right_id);
                                node_index_to_id.insert(tmp, right_id);
                                tmp
                            }
                        };
                        g.add_edge(a, b, 1);
                        g.add_edge(b, a, 1);
                    }
                } else if !edges.contains(&(middle_id, right_id))
                    && !edges.contains(&(right_id, middle_id))
                {
                    let a = match id_to_node_index.get(&middle_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(middle_id);
                            id_to_node_index.insert(middle_id, tmp);
                            node_index_to_id.insert(tmp, middle_id);
                            tmp
                        }
                    };
                    let b = match id_to_node_index.get(&right_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(right_id);
                            id_to_node_index.insert(right_id, tmp);
                            node_index_to_id.insert(tmp, right_id);
                            node_index_to_id.insert(tmp, right_id);
                            tmp
                        }
                    };

                    if middle_elevation > right_elevation {
                        g.add_edge(a, b, 1);
                        edges.insert((middle_id, right_id));
                    } else {
                        g.add_edge(b, a, 1);
                        edges.insert((right_id, middle_id));
                    }
                }
            }

            if j != 0 {
                let left_id = middle_id - 1;
                let left_elevation = {
                    if tokens[i][j - 1] == 'S' as u8 {
                        'a' as u8
                    } else if tokens[i][j - 1] == 'E' as u8 {
                        'z' as u8
                    } else {
                        tokens[i][j - 1]
                    }
                };
                if (middle_elevation).abs_diff(left_elevation) <= 1 {
                    if edges.insert((middle_id, left_id)) && edges.insert((left_id, middle_id)) {
                        let a = match id_to_node_index.get(&middle_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(middle_id);
                                id_to_node_index.insert(middle_id, tmp);
                                node_index_to_id.insert(tmp, middle_id);
                                tmp
                            }
                        };
                        let b = match id_to_node_index.get(&left_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(left_id);
                                id_to_node_index.insert(left_id, tmp);
                                node_index_to_id.insert(tmp, left_id);
                                tmp
                            }
                        };
                        g.add_edge(a, b, 1);
                        g.add_edge(b, a, 1);
                    }
                } else if !edges.contains(&(middle_id, left_id))
                    && !edges.contains(&(left_id, middle_id))
                {
                    let a = match id_to_node_index.get(&middle_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(middle_id);
                            id_to_node_index.insert(middle_id, tmp);
                            node_index_to_id.insert(tmp, middle_id);
                            tmp
                        }
                    };
                    let b = match id_to_node_index.get(&left_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(left_id);
                            id_to_node_index.insert(left_id, tmp);
                            node_index_to_id.insert(tmp, left_id);
                            node_index_to_id.insert(tmp, left_id);
                            tmp
                        }
                    };

                    if middle_elevation > left_elevation {
                        g.add_edge(a, b, 1);
                        edges.insert((middle_id, left_id));
                    } else {
                        g.add_edge(b, a, 1);
                        edges.insert((left_id, middle_id));
                    }
                }
            }

            if i != 0 {
                let up_id = middle_id - width;
                let up_elevation = {
                    if tokens[i - 1][j] == 'S' as u8 {
                        'a' as u8
                    } else if tokens[i - 1][j] == 'E' as u8 {
                        'z' as u8
                    } else {
                        tokens[i - 1][j]
                    }
                };
                if (middle_elevation).abs_diff(up_elevation) <= 1 {
                    if edges.insert((middle_id, up_id)) && edges.insert((up_id, middle_id)) {
                        let a = match id_to_node_index.get(&middle_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(middle_id);
                                id_to_node_index.insert(middle_id, tmp);
                                node_index_to_id.insert(tmp, middle_id);
                                tmp
                            }
                        };
                        let b = match id_to_node_index.get(&up_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(up_id);
                                id_to_node_index.insert(up_id, tmp);
                                node_index_to_id.insert(tmp, up_id);
                                tmp
                            }
                        };
                        g.add_edge(a, b, 1);
                        g.add_edge(b, a, 1);
                    }
                } else if !edges.contains(&(middle_id, up_id))
                    && !edges.contains(&(up_id, middle_id))
                {
                    let a = match id_to_node_index.get(&middle_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(middle_id);
                            id_to_node_index.insert(middle_id, tmp);
                            node_index_to_id.insert(tmp, middle_id);
                            tmp
                        }
                    };
                    let b = match id_to_node_index.get(&up_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(up_id);
                            id_to_node_index.insert(up_id, tmp);
                            node_index_to_id.insert(tmp, up_id);
                            node_index_to_id.insert(tmp, up_id);
                            tmp
                        }
                    };

                    if middle_elevation > up_elevation {
                        g.add_edge(a, b, 1);
                        edges.insert((middle_id, up_id));
                    } else {
                        g.add_edge(b, a, 1);
                        edges.insert((up_id, middle_id));
                    }
                }
            }

            if i != height - 1 {
                let down_id = middle_id + width;
                let down_elevation = {
                    if tokens[i + 1][j] == 'S' as u8 {
                        'a' as u8
                    } else if tokens[i + 1][j] == 'E' as u8 {
                        'z' as u8
                    } else {
                        tokens[i + 1][j]
                    }
                };
                if (middle_elevation).abs_diff(down_elevation) <= 1 {
                    if edges.insert((middle_id, down_id)) && edges.insert((down_id, middle_id)) {
                        let a = match id_to_node_index.get(&middle_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(middle_id);
                                id_to_node_index.insert(middle_id, tmp);
                                node_index_to_id.insert(tmp, middle_id);
                                tmp
                            }
                        };
                        let b = match id_to_node_index.get(&down_id) {
                            Some(node_index) => *node_index,
                            None => {
                                let tmp = g.add_node(down_id);
                                id_to_node_index.insert(down_id, tmp);
                                node_index_to_id.insert(tmp, down_id);
                                tmp
                            }
                        };
                        g.add_edge(a, b, 1);
                        g.add_edge(b, a, 1);
                    }
                } else if !edges.contains(&(middle_id, down_id))
                    && !edges.contains(&(down_id, middle_id))
                {
                    let a = match id_to_node_index.get(&middle_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(middle_id);
                            id_to_node_index.insert(middle_id, tmp);
                            node_index_to_id.insert(tmp, middle_id);
                            tmp
                        }
                    };
                    let b = match id_to_node_index.get(&down_id) {
                        Some(node_index) => *node_index,
                        None => {
                            let tmp = g.add_node(down_id);
                            id_to_node_index.insert(down_id, tmp);
                            node_index_to_id.insert(tmp, down_id);
                            node_index_to_id.insert(tmp, down_id);
                            tmp
                        }
                    };

                    if middle_elevation > down_elevation {
                        g.add_edge(a, b, 1);
                        edges.insert((middle_id, down_id));
                    } else {
                        g.add_edge(b, a, 1);
                        edges.insert((down_id, middle_id));
                    }
                }
            }
        }
    }

    let start_node_index = id_to_node_index.get(&s_id).unwrap().clone();

    let end_node_index = id_to_node_index.get(&e_id).unwrap().clone();

    let res = dijkstra(&g, start_node_index, None, |_| 1);

    match res.get(&end_node_index) {
        Some(n) => return Some(*n),
        None => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_largest() {
        let tokens = parse_input(&String::from(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
",
        ));
        let a = calculate(&tokens);

        assert_eq!(a.unwrap(), 31);
    }

    #[test]
    fn test_is_largest6() {
        let tokens = parse_input(&String::from(
            "SabcabcdefghijklmnopqrstuvwxyzE
",
        ));
        let a = calculate(&tokens);

        assert_eq!(a.unwrap(), 30);
    }
}

pub const CORRECT_ANSWER: i32 = 437;
