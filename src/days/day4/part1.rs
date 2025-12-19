use std::fs;

fn mock_data() -> String {
    String::from(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
    )
}

fn data() -> String {
    let result = fs::read_to_string("./data/day4/part1.txt")
        .expect("file not found");

    result
}

pub(crate) fn is_location_available(matrix: &Vec<Vec<String>>, row: i32, col: i32) -> bool {
    if matrix[row as usize][col as usize] != "@" {
        return false;
    }

    let values_to_check = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /*noting*/ (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    let mut sum = 0;

    for (curr_row, curr_col) in values_to_check {
        let row_to_check = row + curr_row;
        let col_to_check = col + curr_col;
        if row_to_check >= matrix.len() as i32 ||
            row_to_check < 0 ||
            col_to_check >= matrix[row_to_check as usize].len() as i32 ||
            col_to_check < 0 {
            continue;
        }

        if matrix[row_to_check as usize][col_to_check as usize] == "@" {
            sum += 1;
        }
    }

    sum < 4
}

fn process(matrix: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    let mat_len = matrix.len();
    for i in 0..mat_len {
        let row_len = matrix[i].len();
        for j in 0..row_len {
            if is_location_available(&matrix, i as i32, j as i32) {
                sum += 1;
            }
        }
    }

    sum
}

pub(crate) fn make_matrix() -> Vec<Vec<String>> {
    data()
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            x.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>()
}

pub fn run() -> i32 {
    let input = make_matrix();

    let result = process(input);
    println!("Result: {}", result);

    result
}