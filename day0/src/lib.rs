use std::fs::File;
use std::io::prelude::*;

pub fn read_measurements(mut file: File) -> Vec<i32> {
    let mut contents = String::with_capacity(3000);
    file.read_to_string(&mut contents).unwrap();
    let mut vec = Vec::<i32>::with_capacity(3000);
    for line in contents.lines() {
        vec.push(line.parse::<i32>().unwrap());
    }
    vec
}
pub fn num_of_increases(measurements: &[i32]) -> i32 {
    let mut number_of_increases = 0;
    let mut prev_depth = measurements[0];
    for &depth in measurements {
        if depth > prev_depth {
            number_of_increases += 1;
        }
        prev_depth = depth;
    }
    number_of_increases
}

pub fn num_of_increases_on_windows(measurements: &[i32]) -> i32 {
    let _windows = measurements.windows(3);
    let mut prev_sum = measurements[0] + measurements[1] + measurements[2];
    let mut number_of_increases = 0;
    for window in _windows {
        let curr_sum = window[0] + window[1] + window[2];
        if curr_sum > prev_sum {
            number_of_increases += 1;
        }
        prev_sum = curr_sum;
    }
    number_of_increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_increases() {
        let a = num_of_increases(&[1, 3, 9, 3, 5]);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_num_of_increases_on_windows() {
        let a = num_of_increases_on_windows(&[1, 3, 9, 3, 5]);
        assert_eq!(a, 2);
    }
}
