// Integration tests for day1::part1.txt
// These tests import the library crate and test its public API

#[cfg(test)]
mod tests {
    // Import the library crate
    use advent_of_code_2025::days::day2;

    fn data_101() -> Vec<(i64, i64)> {
        let data = vec!(
            (101, 102),
        );
        data
    }

    #[test]
    fn test_101() {
        let result = day2::part2::run(Some(data_101));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mock_data() {
        let result = day2::part2::run(None);
        assert_eq!(result, 4174379265);
    }
}