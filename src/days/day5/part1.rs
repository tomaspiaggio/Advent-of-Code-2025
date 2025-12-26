use std::cmp::{max, min};
use std::{fmt, fs};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Cartesian {
    pub ranges: Vec<(i64, i64)>
}

impl Cartesian {
    #[deprecated]
    pub fn add_range(&mut self, value: (i64, i64)) {
        let (from, to) = value;

        if from == 526790381878899 && to == 527847721743560 {
            println!("stop here");
        }

        if self.ranges.len() == 0 {
            self.ranges.append(&mut vec![value]);
            return;
        } else {
            let (last_from, last_to) = self.ranges[self.ranges.len() - 1];
            if from > last_to {
                self.ranges.append(&mut vec![value]);
                return;
            }

            if from == last_to {
                self.ranges.remove(self.ranges.len() - 1);
                self.ranges.append(&mut vec![(last_from.clone(), to)]);
                return;
            }

            // TODO: the last one is included on the problematic one. need to remove it and add that one.
            if last_from <= from && from <= last_to {
                self.ranges.remove(self.ranges.len() - 1);
                self.ranges.append(&mut vec![(last_from.clone(), max(last_to, to))]);
                return;
            }
        }

        let mut i = 0;
        while i < self.ranges.len() {
            let (curr_from, curr_to) = self.ranges[i];
            if i == 24 {
                print!("stop here");
            }
            if curr_from <= from && curr_to >= to {
                return;
            }

            if i == 0 && to < curr_from {
                self.ranges.insert(0, (from, to));
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

            if from > curr_to && i + 1 >= self.ranges.len() {
                self.ranges.append(&mut vec![(from, to)]);
                return;
            }

            i += 1;
        }
    }

    pub fn add_range_v2(&mut self, (from, to): (i64, i64)) {
        if self.ranges.len() == 0 {
            self.ranges.append(&mut vec![(from, to)]);
            return;
        }

        // remove any that might be inside this one
        for i in (0..self.ranges.len()).rev() {
            let (curr_from, curr_to) = self.ranges[i];
            if curr_from >= from && to >= curr_to {
                self.ranges.remove(i);
            }
        }

        // if value is inside some other value, discard
        for (curr_from, curr_to) in &self.ranges {
            if *curr_from <= from && to <= *curr_to {
                return;
            }
        }

        // does "from" overlap? find index
        let mut from_overlap: i64 = -1; // if it overlaps it will show the index, otherwise, -1
        for i in 0..self.ranges.len() {
            let (curr_from, curr_to) = self.ranges[i];
            if curr_from <= from && from <= curr_to {
                from_overlap = i as i64;
                break;
            }
        }

        if from_overlap >= 0 {
            // does "to" overlap? because we removed the values that were fully included, to can
            // only overlap with the next or with none
            // does "to" overlap with the next one? if it does, we need to merge.
            let (overlapper_from, _) = self.ranges[from_overlap as usize];

            let next_position = (from_overlap + 1) as usize;
            if next_position >= self.ranges.len() {
                self.ranges.remove(from_overlap as usize);
                self.ranges.append(&mut vec![(overlapper_from, to)]);
                return;
            }

            let (next_from, next_to) = self.ranges[next_position];
            if next_from <= to && to <= next_to {
                // remove position
                self.ranges.remove(from_overlap as usize);
                // remove position + 1 (but position was already removed)
                self.ranges.remove(from_overlap as usize);
                self.ranges.insert(from_overlap as usize, (overlapper_from, next_to));
                return;
            }

            // if "to" doesn't overlap, then we insert the overlapper until "to".
            self.ranges.remove(from_overlap as usize);
            self.ranges.insert(from_overlap as usize, (overlapper_from, to));
            return;
        }

        // find "from" index for from because we can ensure it doesn't overlap
        let mut found_from_index = 0;
        for i in 0..self.ranges.len() {
            let (_, curr_to) = self.ranges[i];
            if from > curr_to {
                found_from_index = i + 1;
            }
        }

        // if the index is the last value (because it's the largest value), insert last
        if self.ranges.len() == 0 || found_from_index >= self.ranges.len() {
            self.ranges.append(&mut vec![(from, to)]);
            return;
        }

        // does "to" overlap with the next one? if it does, we need to merge.
        let (next_from, next_to) = self.ranges[found_from_index];
        if next_from <= to && to <= next_to {
            self.ranges.remove(found_from_index);
            self.ranges.insert(found_from_index, (from, next_to));
            return;
        }

        // "to" does not overlap, then we need to insert in the middle
        self.ranges.insert(found_from_index, (from, to));
    }

    pub fn is_spoiled(&self, value: i64) -> bool {
        for (from, to) in &self.ranges {
            if *from <= value && value <= *to {
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

pub fn data_parser(range_string: String, query_string: String) -> (Vec<(i64, i64)>, Vec<i64>) {
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

pub(crate) fn mock_ranges() -> String {
    String::from(
        "3-5
10-14
16-20
12-18"
    )
}

pub(crate) fn mock_queries() -> String {
    String::from(
        "1
5
8
11
17
32"
    )
}

pub fn final_function(ranges: Vec<(i64, i64)>, queries: Vec<i64>) -> i64 {
    let mut cartesian = Cartesian {
        ranges: vec![]
    };

    for range in &ranges {
        cartesian.add_range_v2(range.clone());
    }

    let mut fresh_count = 0;

    for query in queries {
        if !cartesian.is_spoiled(query) {
            fresh_count += 1;
        } else {
            for (from, to) in &ranges {
                if *from <= query && query <= *to {
                    panic!("query: {} fitted on range ({}, {}) and cartesian fucked up", query, from, to);
                }
            }
        }
    }

    println!("Result: {}", fresh_count);

    fresh_count
}

#[deprecated]
pub fn final_function_v2(ranges: &Vec<(i64, i64)>, queries: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for query in queries {
        for (from, to) in ranges {
            if *from <= *query && *query <= *to {
                println!("from {} < query {} < to {}", *from, *query, *to);
                sum += 1;
                break;
            }
        }
    }

    sum
}

pub fn run() -> i64 {
    // let range_string = fs::read_to_string("./data/day5/day5_ranges.txt")
    //     .expect("unable to open file");
    // let query_string = fs::read_to_string("./data/day5/day5_queries.txt")
    //     .expect("unable to open file");

    let range_string = mock_ranges();
    let query_string = mock_queries();

    let (ranges, queries) = data_parser(range_string, query_string);

    let result = final_function(ranges, queries);

    println!("Result: {}", result);

    result
}