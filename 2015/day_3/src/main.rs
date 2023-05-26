use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut directions = Directions::new();
    directions.build_positions(&data);
    println!(
        "Houses with gifts delivered with santa: {}",
        directions.positions.len()
    );
    directions.build_positions_with_robot(&data);
    println!(
        "Houses with gifts delivered with santa and robot: {}",
        directions.positions.len()
    );
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Directions {
    positions: HashSet<Position>,
}

impl Directions {
    fn new() -> Self {
        Self {
            positions: HashSet::new(),
        }
    }

    fn build_positions(&mut self, direction: &str) {
        self.positions.clear();
        self.positions.insert(Position { x: 0, y: 0 });
        let mut x = 0;
        let mut y = 0;
        for character in direction.chars() {
            Directions::change_position_in_axis(&mut x, &mut y, &character);
            self.positions.insert(Position { x, y });
        }
    }

    fn change_position_in_axis(x_axis: &mut i32, y_axis: &mut i32, character: &char) {
        match character {
            '^' => *y_axis += 1,
            'v' => *y_axis -= 1,
            '>' => *x_axis += 1,
            '<' => *x_axis -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn build_positions_with_robot(&mut self, direction: &str) {
        self.positions.clear();
        self.positions.insert(Position { x: 0, y: 0 });
        let mut x_santa = 0;
        let mut y_santa = 0;
        let mut x_robot = 0;
        let mut y_robot = 0;
        let mut iter = direction.chars();
        loop {
            let santa_char = iter.next();
            let robot_char = iter.next();
            match santa_char {
                Some(character) => {
                    Directions::change_position_in_axis(&mut x_santa, &mut y_santa, &character);
                    self.positions.insert(Position {
                        x: x_santa,
                        y: y_santa,
                    });
                }
                None => break,
            }
            match robot_char {
                Some(character) => {
                    Directions::change_position_in_axis(&mut x_robot, &mut y_robot, &character);
                    self.positions.insert(Position {
                        x: x_robot,
                        y: y_robot,
                    });
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_equality() {
        let position = Position { x: 1, y: 1 };

        let expected = Position { x: 1, y: 1 };
        assert_eq!(expected, position);
    }
    #[test]
    fn test_position_inequality() {
        let position = Position { x: 1, y: 1 };

        let expected = Position { x: 1, y: 2 };
        assert_ne!(expected, position);
    }

    #[test]
    fn test_build_positions_ex1() {
        let mut directions = Directions::new();
        directions.build_positions(">");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 1, y: 0 });
        assert_eq!(expected, directions)
    }

    #[test]
    fn test_build_positions_ex2() {
        let mut directions = Directions::new();
        directions.build_positions("^>v<");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 1, y: 0 });
        expected.positions.insert(Position { x: 0, y: 1 });
        expected.positions.insert(Position { x: 1, y: 1 });
        assert_eq!(expected, directions)
    }
    #[test]
    fn test_build_positions_ex3() {
        let mut directions = Directions::new();
        directions.build_positions("^v^v^v^v^v");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 0, y: 1 });
        assert_eq!(expected, directions)
    }

    #[test]
    fn test_build_position_with_robot_1() {
        let mut directions = Directions::new();
        directions.build_positions_with_robot("^v");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 0, y: 1 });
        expected.positions.insert(Position { x: 0, y: -1 });

        assert_eq!(directions, expected);
    }
    #[test]
    fn test_build_position_with_robot_2() {
        let mut directions = Directions::new();
        directions.build_positions_with_robot("^>v<");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 1, y: 0 });
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 0, y: 1 });

        assert_eq!(directions, expected);
    }
    #[test]
    fn test_build_position_with_robot_3() {
        let mut directions = Directions::new();
        directions.build_positions_with_robot("^v^v^v^v^v");

        let mut expected = Directions::new();
        expected.positions.insert(Position { x: 0, y: -4 });
        expected.positions.insert(Position { x: 0, y: 1 });
        expected.positions.insert(Position { x: 0, y: 4 });
        expected.positions.insert(Position { x: 0, y: -3 });
        expected.positions.insert(Position { x: 0, y: 0 });
        expected.positions.insert(Position { x: 0, y: -5 });
        expected.positions.insert(Position { x: 0, y: -2 });
        expected.positions.insert(Position { x: 0, y: 5 });
        expected.positions.insert(Position { x: 0, y: 2 });
        expected.positions.insert(Position { x: 0, y: -1 });
        expected.positions.insert(Position { x: 0, y: 3 });
        expected.positions.insert(Position { x: 0, y: 5 });

        assert_eq!(directions, expected);
    }
}
