// https://adventofcode.com/2015/day/1
use std::fs::read_to_string;

fn find_floor(direction: &str) -> i32 {
    let mut up: i32 = 0;
    let mut down: i32 = 0;

    for character in direction.chars() {
        match character {
            '(' => up += 1,
            ')' => down -= 1,
            _ => (),
        }
    }
    up + down
}

fn find_first_time_getting_into_basement_at_position(direction: &str) -> i32 {
    let mut position = 0;
    let mut basement_signal = 0;
    for character in direction.chars() {
        position += 1;
        basement_signal = if character == '(' {
            basement_signal + 1
        } else {
            basement_signal - 1
        };
        if basement_signal == -1 {
            return position;
        }
    }
    position
}

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let data = file.trim();
    println!("Floor Direction: {}", find_floor(data));
    println!(
        "First Time in Basement: {}",
        find_first_time_getting_into_basement_at_position(data)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn floor_0() {
        let input = String::from("(())");
        let result: i32 = find_floor(&input);
        assert_eq!(0, result);

        let input = String::from("()()");
        let result: i32 = find_floor(&input);
        assert_eq!(0, result);
    }

    #[test]
    fn floor_3() {
        let input = String::from("(((");
        let result: i32 = find_floor(&input);
        assert_eq!(3, result);

        let input = String::from("(()(()(");
        let result: i32 = find_floor(&input);
        assert_eq!(3, result);

        let input = String::from("))(((((");
        let result: i32 = find_floor(&input);
        assert_eq!(3, result);
    }

    #[test]
    fn floor_negative_1() {
        let input = String::from("())");
        let result: i32 = find_floor(&input);
        assert_eq!(-1, result);

        let input = String::from("))(");
        let result: i32 = find_floor(&input);
        assert_eq!(-1, result);
    }

    #[test]
    fn floor_negative_3() {
        let input = String::from(")))");
        let result: i32 = find_floor(&input);
        assert_eq!(-3, result);

        let input = String::from(")())())");
        let result: i32 = find_floor(&input);
        assert_eq!(-3, result);
    }

    #[test]
    fn find_position() {
        let input = String::from(")");
        let result: i32 = find_first_time_getting_into_basement_at_position(&input);
        assert_eq!(1, result);

        let input = String::from("()())");
        let result: i32 = find_first_time_getting_into_basement_at_position(&input);
        assert_eq!(5, result);
    }
}
