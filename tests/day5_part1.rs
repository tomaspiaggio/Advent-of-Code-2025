// Integration tests for day1::part1.txt
// These tests import the library crate and test its public API

#[cfg(test)]
mod tests {
    use advent_of_code_2025::days::day5::part1;
    // Import the library crate
    use advent_of_code_2025::days::day5::part1::Cartesian;

    #[test]
    fn test_basic() {
        let mut cartesian = Cartesian {
            ranges: vec![]
        };

        cartesian.add_range_v2((11, 20));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (11, 20));

        cartesian.add_range_v2((0, 10));
        assert_eq!(cartesian.ranges.len(), 2);
        assert_eq!(cartesian.ranges[0], (0, 10));
        assert_eq!(cartesian.ranges[1], (11, 20));

        cartesian.add_range_v2((9, 12));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 20));

    }

    #[test]
    fn test_complex() {
        let mut cartesian = Cartesian {
            ranges: vec![]
        };

        cartesian.add_range_v2((0, 1));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 1));

        cartesian.add_range_v2((1, 2));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 2));

        cartesian.add_range_v2((0, 1));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 2));

        cartesian.add_range_v2((0, 3));
        cartesian.add_range_v2((1, 2));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 3));

        cartesian.add_range_v2((0, 3));
        cartesian.add_range_v2((4, 5));
        cartesian.add_range_v2((6, 7));
        assert_eq!(cartesian.ranges.len(), 3);
        assert_eq!(cartesian.ranges[0], (0, 3));
        assert_eq!(cartesian.ranges[1], (4, 5));
        assert_eq!(cartesian.ranges[2], (6, 7));

        cartesian.add_range_v2((0, 10));
        println!("{}", cartesian);
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 10));
    }

    #[test]
    fn test_query() {
        let mut cartesian = Cartesian {
            ranges: vec![]
        };

        cartesian.add_range_v2((0, 1));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 1));

        cartesian.add_range_v2((1, 2));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 2));

        cartesian.add_range_v2((0, 1));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 2));

        cartesian.add_range_v2((0, 3));
        cartesian.add_range_v2((1, 2));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (0, 3));

        cartesian.add_range_v2((0, 3));
        cartesian.add_range_v2((4, 5));
        cartesian.add_range_v2((6, 7));
        assert_eq!(cartesian.ranges.len(), 3);
        // THIS VALUE IS BEING INSERTED LAST (NOT SORTED)
        assert_eq!(cartesian.ranges[0], (0, 3));
        assert_eq!(cartesian.ranges[1], (4, 5));
        assert_eq!(cartesian.ranges[2], (6, 7));

        println!("{}", cartesian);

        assert_eq!(cartesian.is_spoiled(0), false);
        assert_eq!(cartesian.is_spoiled(1), false);
        assert_eq!(cartesian.is_spoiled(2), false);
        assert_eq!(cartesian.is_spoiled(3), false);
        assert_eq!(cartesian.is_spoiled(4), false);
        assert_eq!(cartesian.is_spoiled(5), false);
        assert_eq!(cartesian.is_spoiled(6), false);
        assert_eq!(cartesian.is_spoiled(7), false);
        assert_eq!(cartesian.is_spoiled(8), true);
        assert_eq!(cartesian.is_spoiled(9), true);
        assert_eq!(cartesian.is_spoiled(10), true);
        assert_eq!(cartesian.is_spoiled(-1), true);
    }

    #[test]
    fn test_same_value() {
        let mut cartesian = Cartesian {
            ranges: vec![]
        };

        cartesian.add_range_v2((1, 1));
        assert_eq!(cartesian.ranges.len(), 1);
        assert_eq!(cartesian.ranges[0], (1, 1));

        assert!(!cartesian.is_spoiled(1));
    }

    #[test]
    fn test_big_numbers() {
        let mut cartesian = Cartesian {
            ranges: vec![]
        };

        cartesian.add_range_v2((273755558074677, 273755558074677));
        cartesian.add_range_v2((473129501945828, 473593331787329));
        cartesian.add_range_v2((479189870831184, 479808015665254));
        cartesian.add_range_v2((25170668551825, 28139651159810));
        assert_eq!(cartesian.ranges.len(), 4);
    }
}