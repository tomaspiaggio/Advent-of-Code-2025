use std::cmp::{min, Reverse};
use std::fmt::format;
use std::fs;

fn mock_data() -> String {
    String::from("987654321111111
811111111111119
234234234234278
818181911112111")
}

fn data() -> String {
    fs::read_to_string("./data/day3/part2.txt").expect("file not found")
}

pub fn check_done(input: &Vec<i32>, max: i32) -> bool {
    let len_minus_one = (input.len() - 1) as i32;
    for i in (0..=len_minus_one).rev() {
        if input[i as usize] != max - (len_minus_one - i) - 1 {
            return false;
        }
    }

    true
}

pub fn increment_once(mut input: Vec<i32>, max: i32, index: i32) -> Option<(i32, Vec<i32>)> {
    if check_done(&input, max) {
        return None;
    }

    if index < 0 || index >= input.len() as i32 {
        return None;
    }

    let i = index as usize;
    if input[i] + 1 >= max {
        let maybe_result = increment_once(input, max - 1, index - 1);
        match maybe_result {
            None => return None,
            Some((result, mut input)) => {
                input[i] = result + 1;
                return Some((input[i], input));
            }
        }
    }

    input[i] += 1;

    Some((input[i], input))
}

pub fn final_function(subset_length: i32, number_string: &String) -> i64 {
    let split_string = number_string.split("")
        .map(|x| String::from(x))
        .filter(|x| x != "")
        .collect::<Vec<String>>();
    let mut initial_accumulator = vec![0; subset_length as usize];
    let max = number_string.len() as i32;
    let initial_index = subset_length - 1;

    for i in 0..subset_length {
        initial_accumulator[i as usize] = i;
    }

    let mut found_mex: i64 = 0;

    let mut iterations = 0;

    while true {
        let maybe_result = increment_once(initial_accumulator, max, initial_index);
        iterations += 1;
        if iterations % 10_000 == 0 {
            println!("Iteration {}", iterations);
        }

        match maybe_result {
            None => {
                break;
            }
            Some((i, result)) => {
                initial_accumulator = result;

                let mut current_number = String::from("");
                for i in &initial_accumulator {
                    let value: String = split_string.get(*i as usize).unwrap().clone();
                    current_number = format!("{}{}", current_number, value);
                }

                let error_message = format!("{} not a number", &current_number);
                let result: i64 = current_number.parse().expect(error_message.as_str());
                if result > found_mex {
                    found_mex = result
                }
            }
        }
    }

    found_mex
}

fn max_number_in_subset(from: i32, to: i32, number_string: &String) -> (i64, i32) {
    let mut max: i64 = 0;
    let mut index = from;
    let split_string: Vec<i32> = number_string
        .split("")
        .filter(|x| *x != "")
        .map(|x| x.parse().expect("not a number"))
        .collect::<Vec<i32>>();

    for i in from..to {
        let value = split_string[i as usize] as i64;
        if max < value {
            max = value;
            index = i;
        }
    }

    (max, index)
}

// returns the index
fn find_position(subset_length: i32, number_string: &String, from: i32, to: i32) -> String {
    let (max, index) = max_number_in_subset(from, to, number_string);
    if subset_length <= 0 {
        return format!("{}", max);
    }

    let next_found = find_position(subset_length - 1, number_string, index + 1, to + 1);

    format!("{}{}", max, next_found)
}

pub fn final_function_v2(subset_length: i32, number_string: &String) -> i64 {
    let fixed_subset_length = subset_length - 1;
    let result = find_position(fixed_subset_length, number_string, 0, (number_string.len() as i32) - fixed_subset_length);
    let num_result: i64 = result.parse().expect("not a number");
    num_result
}

pub fn run() {
    let data = data();
    let lines = data.lines();
    let mut sum = 0;
    for line in lines {
        println!("Doing line {}", line);
        // let result = final_function(12, &String::from(line));
        let result = final_function_v2(12, &String::from(line));
        sum += result;
    }

    println!("Result: {}", sum);
}