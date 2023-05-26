// https://adventofcode.com/2015/day/1
#[derive(Debug)]
struct Dimension {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimension {
    fn compute_required_paper(&self) -> u32 {
        let surface_area = 2 * self.length * self.width
            + 2 * self.width * self.height
            + 2 * self.height * self.length;
        let mut sorted_dimensions = [self.length, self.width, self.height];
        sorted_dimensions.sort();
        let extra_paper = sorted_dimensions[0] * sorted_dimensions[1];
        surface_area + extra_paper
    }

    fn compute_required_ribbon(&self) -> u32 {
        let mut shortest_side = [self.length, self.width, self.height];
        shortest_side.sort();
        let ribbon_for_present =
            shortest_side[0] + shortest_side[0] + shortest_side[1] + shortest_side[1];
        let ribbon_for_bow = self.width * self.height * self.length;
        ribbon_for_bow + ribbon_for_present
    }
}

impl std::cmp::PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        if self.length == other.length && self.width == other.width && self.height == other.height {
            return true;
        }
        false
    }
}

impl std::convert::TryFrom<&str> for Dimension {
    // type Error = &'static str;
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lowered = value.trim().to_lowercase();
        let splits: Vec<&str> = lowered.split('x').collect();

        if splits.len() == 3 {
            Ok(Self {
                length: match splits[0].parse::<u32>() {
                    Ok(val) => val,
                    _ => panic!("Error parsing the length."),
                },
                width: match splits[1].parse::<u32>() {
                    Ok(val) => val,
                    _ => panic!("Error parsing the width."),
                },
                height: match splits[2].parse::<u32>() {
                    Ok(val) => val,
                    _ => panic!("Error parsing the height."),
                },
            })
        } else {
            Err(format!(
                "The given Input {} does not have the [length x width x height]: ",
                value
            ))
        }
    }
}

#[derive(Debug)]
struct Order {
    dimensions: Vec<Dimension>,
}

impl Order {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            dimensions: Vec::new(),
        }
    }

    fn compute_total_paper(&self) -> u32 {
        let mut required_paper = 0;
        for dim in self.dimensions.iter() {
            required_paper += dim.compute_required_paper();
        }
        required_paper
    }

    fn compute_total_ribbon(&self) -> u32 {
        let mut required_ribbon = 0;
        for dim in self.dimensions.iter() {
            required_ribbon += dim.compute_required_ribbon();
        }
        required_ribbon
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let dimensions: Vec<Dimension> = data
        .split('\n')
        .filter(|x| x.chars().count() >= 5)
        .map(|x| Dimension::try_from(x.trim()).unwrap())
        .collect();
    let order = Order { dimensions };
    dbg!(order.dimensions.len());
    println!("Total Order: {}", order.compute_total_paper());
    println!("Total Ribbon: {}", order.compute_total_ribbon());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        let input = "2x3x4";
        let expected = Dimension {
            length: 2,
            width: 3,
            height: 4,
        };
        let result = Dimension::try_from(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_try_from_panics_with_invalid_argument() {
        let input = "2x3xc";
        _ = Dimension::try_from(input);
    }

    #[test]
    fn test_try_from_returns_err_less_arguments() {
        let input = "2x3";
        let result = Dimension::try_from(input);
        assert!(result.is_err())
    }

    #[test]
    fn test_example_input_1() {
        let input = Dimension::try_from("2x3x4").unwrap();
        let expected = 58;
        let result = input.compute_required_paper();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example_input_2() {
        let input = Dimension::try_from("1x1x10").unwrap();
        let expected = 43;
        let result = input.compute_required_paper();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_compute_total_order_dimension() {
        let dim1 = Dimension::try_from("2x3x4").unwrap();
        let dim2 = Dimension::try_from("1x1x10").unwrap();
        let mut ord = Order::new();
        ord.dimensions.push(dim1);
        ord.dimensions.push(dim2);
        let result = ord.compute_total_paper();
        let expected = 58 + 43;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_ribbon_order_dimension() {
        for sample in [("2x3x4", 34), ("1x1x10", 14)] {
            let dim = Dimension::try_from(sample.0).unwrap();
            let required_ribbon: u32 = dim.compute_required_ribbon();
            let expected = sample.1;
            assert_eq!(required_ribbon, expected);
        }
    }
    #[test]
    fn test_compute_total_ribbon_order_dimension() {
        let dim1 = Dimension::try_from("2x3x4").unwrap();
        let dim2 = Dimension::try_from("1x1x10").unwrap();
        let mut ord = Order::new();
        ord.dimensions.push(dim1);
        ord.dimensions.push(dim2);
        let result = ord.compute_total_ribbon();
        let expected = 34 + 14;
        assert_eq!(result, expected);
    }
}
