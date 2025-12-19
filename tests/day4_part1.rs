// Integration tests for day1::part1.txt
// These tests import the library crate and test its public API

#[cfg(test)]
mod tests {
    // Import the library crate
    use advent_of_code_2025::days::day4;

    #[test]
    fn test_max_increments() {
        assert_eq!(13, day4::part1::run())
    }
}