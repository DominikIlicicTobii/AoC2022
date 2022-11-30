use std::fs::File;

use day0::*;

fn main() {
    let measurements = read_measurements(File::open("input/measurements.txt").unwrap());

    // answer 1
    assert_eq!(num_of_increases(&measurements), 1521);

    // answer 2
    assert_eq!(num_of_increases_on_windows(&measurements), 1543);
}
