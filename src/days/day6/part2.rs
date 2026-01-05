use std::ptr::null;
use crate::days::day6::part1::{data, mock_data, parse_data};

fn new_parse_data(input: &String) -> (Vec<Vec<Vec<String>>>, Vec<String>) {
    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +
    // we know how to split the string based on where the operator is. at what position
    // the order of the ints on the columns matter. look at the first column. the 45 is to the right
    // while on the rightmost column, the 64 is on the left.
    let rows = input.split("\n").collect::<Vec<&str>>();
    let operators = rows[rows.len() - 1].split("")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();

    let mut from_tos: Vec<(i32, i32)> = vec![];
    let mut from = 0;
    for i in 1..operators.len() {
        let value = operators[i];
        if value == "*" || value == "+" {
            from_tos.push((from, i as i32 -1));
            from = i as i32;
        }
    }

    from_tos.push((from, operators.len() as i32));

    let mut results: Vec<Vec<Vec<String>>> = vec![];
    for i in 0..rows.len() - 1 {
        let row = rows[i];
        let mut cols = vec![];
        for (from, to) in &from_tos {
            let mut substr = "";
            if *to >= row.len() as i32 {
                substr = &row[(*from as usize)..row.len()];
            } else {
                substr = &row[(*from as usize)..(*to as usize)];
            }
            let split_result = substr.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
            cols.push(split_result);
        }

        results.push(cols);
    }

    (
        results,
        operators.iter()
            .filter(|x| **x != "")
            .filter(|x| **x != " ")
            .map(|x| String::from(*x))
            .collect::<Vec<String>>()
    )
}

pub fn run() {
    // let input = mock_data();
    let input = data();

    let (parsed, operators) = new_parse_data(&input);

    let rows = parsed.len();
    let cols = parsed[0].len();

    println!("rows {}, cols {}", rows, cols);
    println!("mock data\n{}", input);

    let mut grand_total: i64 = 0;

    for j in 0..cols {
        let operator = &operators[j];

        let mut max_length = 0;
        for m in 0..rows {
            let len = parsed[m][j].len();
            if len > max_length {
                max_length = len;
            }
        }

        let mut result: i64 = 0;
        let mut built_number = String::from("");
        for s in 0..max_length {
            for i in 0..rows {
                if s >= parsed[i][j].len() || parsed[i][j][s] == " " {
                    continue
                }
                built_number = format!("{}{}", built_number, parsed[i][j][s]);
            }

            let parsed = built_number.parse::<i64>().expect("not a number");

            if operator == "+" {
                println!("result {} += parsed {}", result, parsed);
                result += parsed;
            } else {
                if result == 0 {
                    println!("result {} = parsed {}", result, parsed);
                    result = parsed;
                } else {
                    println!("result {} *= parsed {}", result, parsed);
                    result *= parsed;
                }
            }

            built_number = String::from("");
        }

        grand_total += result;
    }

    if grand_total <= 10389131400417 {
        panic!("number is too low according to last test. current value {} <= 10389131400417", grand_total)
    }

    println!("result {}", grand_total);

    return;
}