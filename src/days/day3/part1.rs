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

fn find_max(input: Vec<i32>) -> i32 {
    let mut max = -1;
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let result: i32 = format!("{}{}", input[i], input[j]).parse().expect("not a number");
            if result > max {
                max = result
            }
        }
    }

    max
}


fn process(s: String) -> i32 {
    let result = s.split("")
        .filter(|x| *x != "")
        .map(|x| x.parse().expect("not a number"))
        .collect::<Vec<i32>>();

    let final_result = find_max(result);
    println!("Source: {}, Result: {}", s, final_result);

    final_result
}

pub fn run() {
    let data = data();
    let lines = data.lines();
    let mut sum = 0;
    for line in lines {
        let result = process(String::from(line));
        sum += result;
    }

    println!("Result: {}", sum);
}