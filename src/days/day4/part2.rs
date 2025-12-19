use crate::days::day4::part1;

fn process(matrix: &Vec<Vec<String>>) -> (i32, Vec<Vec<String>>) {
    let mut sum = 0;
    let mut new_mat = matrix.clone();

    let mat_len = matrix.len();
    for i in 0..mat_len {
        let row_len = matrix[i].len();
        for j in 0..row_len {
            if part1::is_location_available(&matrix, i as i32, j as i32) {
                new_mat[i][j] = String::from(".");
                sum += 1;
            }
        }
    }

    (sum, new_mat)
}

pub fn run() -> i32 {
    let mut mat = part1::make_matrix();

    let mut sum = 0;
    while true {
        let (curr_sum, new_mat) = process(&mat);
        if curr_sum == 0 {
            break;
        }
        sum += curr_sum;
        mat = new_mat;
    }

    println!("Result: {}", sum);

    sum
}