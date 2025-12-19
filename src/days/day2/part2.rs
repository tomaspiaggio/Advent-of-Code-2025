use crate::days::day2::part1::{data, mock_data};

fn do_loop(input: &Vec<&str>, pattern: &Vec<&&str>) -> bool {
    if input.len() % pattern.len() != 0 || input.len() / pattern.len() <= 1 {
        return false
    }

    for i in 0..input.len() {
        let left = input.get(i).unwrap();
        let right = pattern.get(i % pattern.len()).unwrap();
        if left != *right {
            return false
        }
    }

    true
}

fn is_invalid(num: i64) -> bool {
    let stringified = format!("{}", num);
    let splitted: Vec<&str> = stringified.split("")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();

    for i in 1..splitted.len() {
        let sublist: Vec<&&str> = splitted[0..i].iter().collect::<Vec<&&str>>();
        let result = do_loop(&splitted, &sublist);
        if (result) {
            return result
        }
    }

    false
}

pub fn run(feeder: Option<fn() -> Vec<(i64, i64)>>) -> i64 {
    let data = feeder.map(|x| x())
        .unwrap_or_else(|| data());

    let mut sum = 0;

    for (left, right) in data {
        println!("About to test {}-{}", left, right);
        for i in left..=right {
            let result = is_invalid(i);
            if (result) {
                println!("Invalid ID: {}", i);
                sum += i;
            }
        }
    }

    println!("Result: {}", sum);

    sum
}