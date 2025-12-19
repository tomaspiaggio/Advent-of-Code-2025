// Integration tests for day1::part1.txt
// These tests import the library crate and test its public API

#[cfg(test)]
mod tests {
    // Import the library crate
    use advent_of_code_2025::days::day3;

    #[test]
    fn test_final_function() {
        let input_string = String::from("987654321111111");
        let result = day3::part2::final_function(12, &input_string);

        let expected: i64 = 987654321111;
        assert_eq!(expected, result);

        let input_string = String::from("811111111111119");
        let result = day3::part2::final_function(12, &input_string);

        let expected: i64 = 811111111119;
        assert_eq!(expected, result);

        let input_string = String::from("234234234234278");
        let result = day3::part2::final_function(12, &input_string);

        let expected: i64 = 434234234278;
        assert_eq!(expected, result);

        let input_string = String::from("818181911112111");
        let result = day3::part2::final_function(12, &input_string);

        let expected: i64 = 888911112111;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_final_function_v2() {
        let input_string = String::from("987654321111111");
        let result = day3::part2::final_function(12, &input_string);
        let result2 = day3::part2::final_function_v2(12, &input_string);

        let expected: i64 = 987654321111;
        assert_eq!(expected, result);
        assert_eq!(result, result2);

        let input_string = String::from("811111111111119");
        let result = day3::part2::final_function(12, &input_string);
        let result2 = day3::part2::final_function_v2(12, &input_string);

        let expected: i64 = 811111111119;
        assert_eq!(expected, result);
        assert_eq!(result, result2);

        let input_string = String::from("234234234234278");
        let result = day3::part2::final_function(12, &input_string);
        let result2 = day3::part2::final_function_v2(12, &input_string);

        let expected: i64 = 434234234278;
        assert_eq!(expected, result);
        assert_eq!(result, result2);

        let input_string = String::from("818181911112111");
        let result = day3::part2::final_function(12, &input_string);
        let result2 = day3::part2::final_function_v2(12, &input_string);

        let expected: i64 = 888911112111;
        assert_eq!(expected, result);
        assert_eq!(result, result2);
    }

    #[test]
    fn test_intermediate_increments() {
        let mut input = vec!(0, 2, 4, 6, 7, 10);
        let length = input.len() as i32;
        let maybe_result = day3::part2::increment_once(input, 10, length - 1);

        assert_ne!(maybe_result, None);
        let (i, result) = maybe_result.unwrap();

        assert_eq!(result[result.len() - 1], 9);
        assert_eq!(result[result.len() - 2], 8);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 4);
        assert_eq!(result[3], 6);
    }

    #[test]
    fn test_check_done() {
        let input = vec!(8, 9);
        let result = day3::part2::check_done(&input, 10);
        assert!(result);
    }

    #[test]
    fn test_check_done_in_loop() {
        let mut input = vec!(8, 9);
        let length = input.len() as i32;
        let result = day3::part2::increment_once(input, 10, length);
        assert_eq!(result, None);
    }

    #[test]
    fn test_max_increments() {
        let mut input = vec!(0, 2, 4, 6, 7, 10);
        let length = input.len() as i32;
        let max = 10;
        while true {
            let maybe_result = day3::part2::increment_once(input, max, length - 1);
            match maybe_result {
                None => {
                    break;
                }
                Some((i, result)) => {
                    input = result;
                    // perform_action(&input);
                }
            }
        }

        println!("done")
    }
}