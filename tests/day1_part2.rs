// Integration tests for day1::part1.txt
// These tests import the library crate and test its public API

#[cfg(test)]
mod tests {
    // Import the library crate
    use advent_of_code_2025::days::day1;

    fn data_forward_once() -> Vec<String> {
        let data = vec!(String::from("R50"));
        data
    }

    #[test]
    fn test_forward_once() {
        let dial = day1::part2::run(Some(data_forward_once));
        assert_eq!(dial.clicks, 1);
    }

    fn data_backwards_once() -> Vec<String> {
        let data = vec!(String::from("L50"));
        data
    }

    #[test]
    fn test_backwards_once() {
        let dial = day1::part2::run(Some(data_backwards_once));
        assert_eq!(dial.clicks, 1);
    }

    fn data_forwards_many() -> Vec<String> {
        let data = vec!(
            String::from("R50"),
            String::from("R51"),
            String::from("R52"),
            String::from("R53"),
        );
        data
    }

    #[test]
    fn test_forwards_many() {
        let dial = day1::part2::run(Some(data_forwards_many));
        assert_eq!(dial.clicks, 2);
    }

    fn data_backwards_many() -> Vec<String> {
        let data = vec!(
            String::from("L50"),
            String::from("L51"),
            String::from("L52"),
            String::from("L53"),
        );
        data
    }

    #[test]
    fn test_backwards_many() {
        let dial = day1::part2::run(Some(data_backwards_many));
        assert_eq!(dial.clicks, 2);
    }

    fn data_forwards_thousand() -> Vec<String> {
        let data = vec!(
            String::from("R1000"),
        );
        data
    }

    #[test]
    fn test_forwards_thousand() {
        let dial = day1::part2::run(Some(data_forwards_thousand));
        assert_eq!(dial.clicks, 10);
    }

    fn data_backwards_thousand() -> Vec<String> {
        let data = vec!(
            String::from("R1000"),
        );
        data
    }

    #[test]
    fn test_backwards_thousand() {
        let dial = day1::part2::run(Some(data_backwards_thousand));
        assert_eq!(dial.clicks, 10);
    }
}