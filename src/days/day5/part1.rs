use std::cmp::{max, min};
use std::{fmt, fs};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Cartesian {
    pub ranges: Vec<(i64, i64)>
}

impl Cartesian {
    pub fn add_range(&mut self, value: (i64, i64)) {
        if self.ranges.len() == 0 {
            self.ranges.append(&mut vec![value]);
            return;
        }
        let (from, to) = value;

        let mut i = 0;
        while i < self.ranges.len() {
            let (curr_from, curr_to) = self.ranges[i];
            if curr_from >= from && to <= curr_to {
                return;
            }

            if curr_from <= from && from <= curr_to {
                let mut j = i + 1;
                if j >= self.ranges.len() {
                    self.ranges.remove(i);
                    self.ranges.insert(i, (curr_from, max(to, curr_to)));
                    return;
                }

                while j < self.ranges.len() {
                    let (next_from, next_to) = self.ranges[j];
                    if to > next_to {
                        self.ranges.remove(j);
                        i = max(i as i32 - 1, 0) as usize;
                        j = max(min(j as i32, (self.ranges.len() as i32) - 1), 0) as usize;
                        if self.ranges.len() == 0 {
                            self.ranges.append(&mut vec![(from, to)]);
                            return;;
                        }
                        continue;
                    }

                    if to < next_from {
                        self.ranges.insert(j, (curr_from, max(to, curr_to)));
                        return;
                    }

                    if to >= next_from {
                        self.ranges.remove(j);
                        self.ranges.remove(j - 1);
                        self.ranges.insert(j - 1, (curr_from, next_to));
                        return;
                    }

                    j += 1;
                }
            }

            if to < curr_from {
                self.ranges.insert(i, (from, to));
                return;
            }

            if from > curr_to {
                if i + 1 >= self.ranges.len() {
                    self.ranges.append(&mut vec![(from, to)]);
                    return;
                }
            }

            i += 1;
        }
    }

    pub fn is_spoiled(&self, value: i64) -> bool {
        for (from, to) in &self.ranges {
            if value >= *from && value <= *to {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Cartesian {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.ranges.len() == 0 {
            return write!(f, "Cartesian [empty]");
        }

        let result = self.ranges
            .iter()
            .map(|(from, to)| format!("(from: {}, to: {})", from, to))
            .collect::<Vec<String>>()
            .join(", ");

        println!("result {}, length {}", &result, self.ranges.len());

        write!(f, "Cartesian [{}]", result)
    }
}

fn data_parser(range_string: String, query_string: String) -> (Vec<(i64, i64)>, Vec<i64>) {
    let ranges: Vec<(i64, i64)> = range_string.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.split("-")
            .filter(|x| *x != "")
            .map(|x| x.parse::<i64>().expect("not a number"))
            .collect::<Vec<i64>>())
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(i64, i64)>>();

    let queries: Vec<i64> = query_string.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.parse::<i64>().expect("not a number"))
        .collect::<Vec<i64>>();

    (ranges, queries)
}

fn mock_ranges() -> String {
    String::from(
        "3-5
10-14
16-20
12-18"
    )
}

fn mock_queries() -> String {
    String::from(
        "1
5
8
11
17
32"
    )
}

pub fn run() -> i64 {
    let mut cartesian = Cartesian {
        ranges: vec![]
    };
    
    let range_string = fs::read_to_string("./data/day5/day5_ranges.txt")
        .expect("unable to open file");
    let query_string = fs::read_to_string("./data/day5/day5_queries.txt")
        .expect("unable to open file");

    let (ranges, queries) = data_parser(range_string, query_string);

    for range in ranges {
        cartesian.add_range(range);
    }

    let mut fresh_count = 0;
    
    for query in queries {
        if cartesian.is_spoiled(query) {
            fresh_count += 1;
        }
    }
    
    println!("Result: {}", fresh_count);
    
    fresh_count
}