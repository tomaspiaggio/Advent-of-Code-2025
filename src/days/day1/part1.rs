use std::fmt::Formatter;
use std::{fmt, fs};

#[derive(Debug)]
enum Direction {
    LEFT, RIGHT
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::LEFT => write!(f, "LEFT"),
            Direction::RIGHT => write!(f, "RIGHT")
        }
    }
}

struct Dial {
    current_position: i32,
}

impl Dial {
    pub fn move_dial(&mut self, direction: Direction, distance: i32) {
        let new_position: i32 = match direction {
            Direction::LEFT => {
                let difference = self.current_position - distance;
                if difference < 0 {
                    (100 + (difference % 100)) % 100
                } else {
                    difference
                }
            },
            Direction::RIGHT => (self.current_position + distance) % 100
        };

        assert!(new_position < 100, "new position should be smaller than 100: {}. Direction: {}. Distance: {}. Old Position: {}", new_position, direction, distance, self.current_position);
        assert!(new_position >= 0, "new position shuold be bigger than 0: {}. Direction: {}. Distance: {}. Old Position: {}", new_position, direction, distance, self.current_position);

        self.current_position = new_position;
    }
}

#[warn(dead_code)]
fn data() -> Vec<String> {
    let file_path = "./data/day1/first_part.txt";
    let error_message = format!("File {} not found", file_path);
    let result = fs::read_to_string(file_path).expect(error_message.as_str());
    let lines = result.lines();
    println!("Reading input file of length {} and lines {}", result.len(), lines.clone().count());

    lines.map(|x| String::from(x)).collect::<Vec<String>>()
}

#[warn(dead_code)]
fn mock_data() -> Vec<String> {
    let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    data.split("\n")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}

pub fn run(data_maker: Option<fn() -> Vec<String>>) {
    let data = data_maker.map(|f| f())
        .unwrap_or_else(|| mock_data());
    let mut dial = Dial {
        current_position: 50
    };
    let mut sum: i32 = 0;
    for line in data {
        let raw_direction = line.chars().next().expect("x was empty");
        let error_message = format!("amount was not a number {}", raw_direction);
        let amount: i32 = line[1..].parse().expect(&error_message);
        let direction = parse_direction(&raw_direction);
        dial.move_dial(direction, amount);
        if dial.current_position == 0 {
            sum += 1;
        }
    }

    println!("current_position {}; actual_password {}", dial.current_position, sum)
}

fn parse_direction(raw_direction: &char) -> Direction {
    if *raw_direction == 'R' {
        return Direction::RIGHT
    }

    Direction::LEFT
}