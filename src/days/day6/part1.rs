use std::fs;

pub(crate) fn mock_data() -> String {
    String::from(
"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ")
}

pub(crate) fn data() -> String {
    fs::read_to_string("./data/day6/part1.txt").expect("file not found")
}

pub(crate) fn parse_data(input: &String) -> Vec<Vec<String>> {
    input.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.split(" ")
            .filter(|x| x.trim() != "")
            .map(|x| String::from(x))
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn run() {
    // let input = mock_data();
    let input = data();
    let parsed = parse_data(&input);

    for v in &parsed {
        for value in v {
            print!("{}, ", value);
        }
        println!();
    }

    let cols = parsed[0].len();
    let rows = parsed.len() - 1;

    println!("rows {}, cols {}", rows, cols);

    let mut grand_total: i64 = 0;

    for j in 0..cols {
        let operator = &parsed[rows][j];
        let mut result: i64 = 0;
        if operator == "+" {
            for i in 0..rows {
                let value = &parsed[i][j];
                let parsed_value: i64 = value.parse().expect("not a number");
                result += parsed_value;
                println!("{} += {}", result, parsed_value);
            }
        } else {
            for i in 0..rows {
                let value = &parsed[i][j];
                let parsed_value: i64 = value.parse().expect("not a number");
                if result == 0 {
                    result = parsed_value;
                    println!("result = {}", parsed_value);
                    continue;
                }
                println!("{} *= {}", result, parsed_value);
                result *= parsed_value;
            }
        }

        grand_total += result;
    }

    println!("result {}", grand_total);
    
    return;
}
