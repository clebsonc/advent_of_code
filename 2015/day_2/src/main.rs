struct Dimensions {}

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
        let extra_paper: u32 = *[self.length, self.width, self.height].iter().min().unwrap();
        dbg!(surface_area, extra_paper);

        surface_area + extra_paper
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
    type Error = &'static str;

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
            Err("The given Input does not have the [length x width x height].")
        }
    }
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
    fn example1() {
        let input = Dimension::try_from("2x3x4").unwrap();
        let expected = 58;
        let result = input.compute_required_paper();
        assert_eq!(expected, result);
    }

    // #[test]
    // fn example2() {
    //     let input = String::from("1x1x10");
    //     let expected = 43;
    //     result = compute_necessary_paper(input);
    //     assert_eq!(expected, result);
    // }
}
