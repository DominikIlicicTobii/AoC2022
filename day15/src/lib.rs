use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let a: Vec<&str> = s.split(' ').filter(|p| p.contains('=')).collect();
        let mut values = Vec::new();
        for s in a {
            let tmp = s.split('=').collect::<Vec<&str>>();
            let mut last = String::from(*tmp.last().unwrap());
            if !last.chars().last().unwrap().is_ascii_digit() {
                last.remove(last.len() - 1);
            }
            values.push(last.parse::<i64>().unwrap());
        }

        assert_eq!(values.len(), 4);

        Ok(Token {
            sensor: (values[0], values[1]),
            beacon: (values[2], values[3]),
        })
    }
}

type Tokens = Vec<Token>;

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
    input
        .split('\n')
        .filter(|p| !p.is_empty())
        .map(|g| Token::from_str(g).unwrap())
        .collect()
}

pub fn get_answer(_tokens: &Tokens) -> i64 {
    let map = Map::from(_tokens);

    let mut all_outer_edge_points = Vec::with_capacity(94476656);

    for sensor in &map.sensors {
        for point in sensor.outer_edge() {
            all_outer_edge_points.push(point);
        }
    }

    'outer: for point in all_outer_edge_points {
        if point.x < 0 || point.y < 0 || point.x > 4000000 || point.y > 4000000 {
            continue 'outer;
        }

        for sensor in &map.sensors {
            if sensor.inside_manhattan(point.clone()) {
                continue 'outer;
            }
        }
        return point.x * 4000000 + point.y;
    }

    unreachable!()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    point: Point,
    manhattan: i64,
}

impl Sensor {
    fn outer_edge(&self) -> Vec<Point> {
        let manhattan = self.manhattan + 1;

        let mut points = Vec::new();

        for row in self.point.y..(self.point.y + manhattan + 1) {
            let n_chars = 2 * manhattan + 1 - (row - self.point.y) * 2;
            let left = self.point.x - n_chars / 2;
            let right = self.point.x + n_chars / 2;

            points.push(Point { x: left, y: row });
            if row != self.point.y {
                points.push(Point {
                    x: left,
                    y: self.point.y - (row - self.point.y),
                })
            };
            if left != right {
                points.push(Point { x: right, y: row });
                if row != self.point.y {
                    points.push(Point {
                        x: right,
                        y: self.point.y - (row - self.point.y),
                    });
                }
            }
        }

        points
    }

    fn inside_manhattan(&self, point: Point) -> bool {
        let manhattan_to_point = (self.point.x - point.x).abs() + (self.point.y - point.y).abs();

        if manhattan_to_point > self.manhattan {
            return false;
        } else {
            return true;
        }
    }
}

#[derive(Debug)]
struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn from(tokens: &Tokens) -> Map {
        let mut sensors = Vec::new();

        for token in tokens {
            let sensor = Sensor {
                point: Point {
                    x: token.sensor.0,
                    y: token.sensor.1,
                },
                manhattan: {
                    (token.sensor.0 - token.beacon.0).abs()
                        + (token.sensor.1 - token.beacon.1).abs()
                },
            };

            sensors.push(sensor);
        }

        Map { sensors }
    }
}

pub const CORRECT_ANSWER: i64 = 12274327017867;
