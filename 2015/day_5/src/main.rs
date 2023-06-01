use std::fs::read_to_string;

fn main() {
    use better_special_string::verify_string_is_nice as better_nice_string;
    use special_string::verify_string_is_nice;
    let data = read_to_string("input.txt").unwrap();
    let words: Vec<&str> = data.split('\n').filter(|x| x.len() > 10).collect();
    let mut total_nice_string = 0;
    let mut total_better_nice_string = 0;

    for w in words {
        if verify_string_is_nice(w) {
            total_nice_string += 1;
        }

        if better_nice_string(w) {
            total_better_nice_string += 1;
        }
    }
    println!("Total nice strings: {}", total_nice_string);
    println!("Total nice strings: {}", total_better_nice_string);
}

mod better_special_string {
    pub fn verify_string_is_nice(input: &str) -> bool {
        let has_middle = has_middle_letter_in_pair(input);
        let has_double_pair = has_double_non_overlapping_pairs(input);
        if has_double_pair && has_middle {
            return true;
        }
        false
    }

    fn has_middle_letter_in_pair(word: &str) -> bool {
        let characters: Vec<char> = word.trim().chars().collect();

        if characters.is_empty() || characters.len() < 3 {
            return false;
        }

        let mut first = 0;
        let mut last = 2;

        while last < characters.len() {
            if characters[first] == characters[last] {
                return true;
            }
            first += 1;
            last += 1;
        }

        false
    }

    fn has_double_non_overlapping_pairs(word: &str) -> bool {
        let characters: Vec<char> = word.trim().chars().collect();
        if characters.len() <= 3 {
            return false;
        }
        let mut i = 0;
        let mut j = 1;
        while j < characters.len() {
            let mut k = j + 1;
            let mut l = j + 2;
            while l < characters.len() {
                if characters[k] == characters[i] && characters[l] == characters[j] {
                    return true;
                }
                k += 1;
                l += 1;
            }
            i += 1;
            j += 1;
        }
        false
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_double_non_overlapping_pairs() {
            let input = ["xyxy", "aabcdefgaa", "qjhvhtzxzqqjkmpb", "xxxx"];
            for word in input {
                let result: bool = has_double_non_overlapping_pairs(word);
                assert!(result);
            }
            let input = ["xyxz", "aabcdefga", "qjhvhtzxzqqkmpb", "xxxc", "aaa"];
            for word in input {
                let result: bool = has_double_non_overlapping_pairs(word);
                assert!(!result);
            }
        }

        #[test]
        fn test_letter_between_pair() {
            let input = ["xyx", "abcdefeghi", "aaa", "xxxx"];
            for word in input {
                let result: bool = has_middle_letter_in_pair(word);
                assert!(result);
            }
            let input = ["xx", "abcdeeghi", "aac", "xxcd"];
            for word in input {
                let result: bool = has_middle_letter_in_pair(word);
                assert!(!result);
            }
        }
    }
}

mod special_string {
    fn verify_not_allowed_subchars(input: &str) -> bool {
        let not_allowed = vec!["ab", "cd", "pq", "xy"];
        for substring in not_allowed.iter() {
            let found = input.find(substring);
            if found.is_some() {
                return false;
            }
        }
        true
    }

    fn contains_three_vowels(input: &str) -> bool {
        let mut counter = 0;
        for c in input.chars() {
            match c {
                'a' => counter += 1,
                'e' => counter += 1,
                'i' => counter += 1,
                'o' => counter += 1,
                'u' => counter += 1,
                _ => (),
            }
        }
        if counter < 3 {
            return false;
        }
        true
    }

    fn has_double_letters(input: &str) -> bool {
        let characters: Vec<char> = input.trim().chars().collect();
        let vector_size = characters.len() - 1;
        for index in 0..vector_size {
            if characters[index] == characters[index + 1] {
                return true;
            }
        }
        false
    }

    pub fn verify_string_is_nice(input: &str) -> bool {
        let no_subchars = verify_not_allowed_subchars(input);
        let has_allowed_vowels = contains_three_vowels(input);
        let has_double_char = has_double_letters(input);
        if no_subchars && has_allowed_vowels && has_double_char {
            return true;
        }
        false
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_substring_not_allowed() {
            let input = "ugknbfddgicrmopn";
            let result = verify_not_allowed_subchars(input);
            assert!(result);
            let input = "ugknbfcddgicrmopn";
            let result = verify_not_allowed_subchars(input);
            assert!(!result);
        }

        #[test]
        fn test_allowed_vowels() {
            let input = ["aaa", "aeiou", "xazegov", "aeiouaeiouaeiou"];
            for word in input {
                let result = contains_three_vowels(word);
                assert!(result);
            }
            let input = ["aa", "aic", "xov", "beetlll"];
            for word in input {
                let result = contains_three_vowels(word);
                assert!(!result);
            }
        }

        #[test]
        fn test_double_letters() {
            let input = ["abcdde", "aabbccdd"];
            for word in input {
                let result = has_double_letters(word);
                assert!(result);
            }
            let input = ["abcde", "abcd"];
            for word in input {
                let result = has_double_letters(word);
                assert!(!result);
            }
        }
    }
}

#[cfg(test)]
mod main_tests {
    use super::special_string::verify_string_is_nice;

    #[test]
    fn test_is_nice() {
        let input = ["ugknbfddgicrmopn", "aaa"];
        for word in input {
            let result: bool = verify_string_is_nice(word);
            assert!(result);
        }
        let input = ["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"];
        for word in input {
            let result: bool = verify_string_is_nice(word);
            assert!(!result);
        }
    }
}
