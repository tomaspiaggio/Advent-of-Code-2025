use std::fs;
use crate::days::day5::part1::{mock_queries, mock_ranges, Cartesian};

fn final_function(ranges: Vec<(i64, i64)>, queries: Vec<i64>) -> i64 {
    let mut cartesian = Cartesian {
        ranges: vec![]
    };

    for range in &ranges {
        cartesian.add_range_v2(range.clone());
    }

    let mut sum = 0;

    for range in cartesian.ranges {
        let (from, to) = range;
        sum += to - from + 1;
    }

    sum
}

pub fn run() -> i64 {
    let range_string = fs::read_to_string("./data/day5/day5_ranges.txt")
        .expect("unable to open file");
    let query_string = fs::read_to_string("./data/day5/day5_queries.txt")
        .expect("unable to open file");

    // let range_string = mock_ranges();
    // let query_string = mock_queries();

    let (ranges, queries) = crate::days::day5::part1::data_parser(range_string, query_string);

    let result = final_function(ranges, queries);

    println!("Result: {}", result);

    result
}