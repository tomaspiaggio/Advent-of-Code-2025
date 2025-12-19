use std::{fmt, fs};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Direction {
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

pub struct Dial {
    pub current_position: i32,
    pub clicks: i32
}

impl Dial {
    pub fn move_dial(&mut self, direction: Direction, distance: i32) {
        let original_value = self.current_position;
        match direction {
            Direction::LEFT => {
                let difference = self.current_position - distance;
                if difference < 0 {
                    if self.current_position != 0 {
                        self.clicks += 1
                    }
                    self.current_position = (100 + (difference % 100)) % 100;
                    self.clicks += (difference / -100);
                } else {
                    self.current_position = difference;
                    if self.current_position == 0 {
                        self.clicks += 1
                    }
                }
            },
            Direction::RIGHT => {
                let sum = self.current_position + distance;
                self.current_position = sum % 100;
                self.clicks += sum / 100;
            }
        };

        assert!(self.current_position < 100, "new position should be smaller than 100: {}. Direction: {}. Distance: {}. Old Position: {}", self.current_position, direction, distance, original_value);
        assert!(self.current_position >= 0, "new position shuold be bigger than 0: {}. Direction: {}. Distance: {}. Old Position: {}", self.current_position, direction, distance, original_value);
    }
}

fn parse_direction(raw_direction: &char) -> Direction {
    if *raw_direction == 'R' {
        return Direction::RIGHT
    }

    Direction::LEFT
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

pub fn run(data_maker: Option<fn() -> Vec<String>>) -> Dial {
    let data = data_maker.map(|f| f())
        .unwrap_or_else(|| data());
    let mut dial = Dial {
        current_position: 50,
        clicks: 0
    };

    for line in data {
        let first = line.chars().next().expect("line was empty");
        let direction = parse_direction(&first);
        let distance: i32 = line[1..].parse::<i32>().expect(format!("line {} not a number", line).as_str());
        println!("Before: pos={}, clicks={} | Moving {} by {}", dial.current_position, dial.clicks, direction, distance);
        dial.move_dial(direction, distance);
        println!("After:  pos={}, clicks={}", dial.current_position, dial.clicks);
    }

    println!("Result clicks: {}", dial.clicks);

    dial
}