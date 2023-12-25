use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    let sum = part2("src/bin/input1.txt");
    println!("{}", sum);
}

fn part2(file_path: &str) -> u32 {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");
    let mut sums: Vec<u32> = Vec::new();

    for line in input.lines() {
        sums.push(parse_line(line));
    }

    return sums.iter().sum();
}

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("zero", 0);
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m.insert("0", 0);
        m.insert("1", 1);
        m.insert("2", 2);
        m.insert("3", 3);
        m.insert("4", 4);
        m.insert("5", 5);
        m.insert("6", 6);
        m.insert("7", 7);
        m.insert("8", 8);
        m.insert("9", 9);
        m
    };
}

struct Anchor {
    value: String,
    target: String,
}

impl Anchor {
    fn new(value: String, target: String) -> Anchor {
        Anchor { value, target }
    }

    fn is_value(&self) -> bool {
        for number in NUMBERS.keys() {
            if number == &self.value {
                return true;
            }
        }
        return false;
    }

    fn consume(&self) -> u32 {
        return NUMBERS
            .get_key_value(&self.value.as_str())
            .unwrap()
            .1
            .to_owned();
    }
}

fn parse_line(line: &str) -> u32 {
    let mut values: Vec<u32> = Vec::new();
    let mut anchors: Vec<Anchor> = Vec::new();

    for char in line.chars() {
        for anchor in anchors.iter_mut() {
            anchor.value.push(char);
        }

        for key in NUMBERS.keys() {
            if key.starts_with(char) {
                // The charater is the start of an existing number (eg "one")
                anchors.push(Anchor::new(char.to_string(), key.to_string()));
            }

            let mut to_remove: Vec<usize> = Vec::new();

            for (idx, anchor) in anchors.iter().enumerate() {
                if anchor.value == anchor.target {
                    values.push(NUMBERS.get(&anchor.value.as_str()).unwrap().to_owned());
                    // remove the anchor from the list
                    to_remove.push(idx)
                } else if !anchor.target.starts_with(&anchor.value) {
                    // remove the anchor from the list
                    to_remove.push(idx)
                }
            }

            for i in to_remove.iter().rev() {
                anchors.remove(*i);
            }
        }
    }

    let first = values[0];
    let last = values[values.len() - 1];

    let combined = format!("{}{}", first, last);

    return combined.parse::<u32>().unwrap();
}

#[cfg(test)]
mod test_part2 {
    #[test]
    fn example_input() {
        use super::*;
        let result = part2("src/bin/example2.txt");
        assert_eq!(result, 281);
    }

    #[test]
    fn edge_cases_oneeight() {
        use super::*;
        let result = parse_line("oneight");
        assert_eq!(result, 18);
    }

    #[test]
    fn edge_cases_two1nine() {
        use super::*;
        let result = parse_line("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn edge_cases_eighthree() {
        use super::*;
        let result = parse_line("eighthree");
        assert_eq!(result, 83);
    }

    #[test]
    fn edge_cases_sevenine() {
        use super::*;
        let result = parse_line("sevenine");
        assert_eq!(result, 79);
    }

    #[test]
    fn edge_cases_threetwoonez1gtrd() {
        use super::*;
        let result = parse_line("threetwoonez1gtrd");
        assert_eq!(result, 31);
    }

    #[test]
    fn edge_cases_769twotwo6rv9() {
        use super::*;
        let result = parse_line("769twotwo6rv9");
        assert_eq!(result, 79);
    }

    #[test]
    fn test_anchor_is_value() {
        use super::*;
        let anchor = Anchor::new("one".to_string(), "one".to_string());
        assert_eq!(anchor.is_value(), true);
        assert_eq!(anchor.consume(), 1);
    }

    #[test]
    fn test_anchor_is_not_value() {
        use super::*;
        let anchor = Anchor::new("on".to_string(), "one".to_string());
        assert_eq!(anchor.is_value(), false);
    }

    #[test]
    fn test_correct_answer() {
        use super::*;
        let correct_answer = 54249;
        let answer = part2("src/bin/input1.txt");
        assert_eq!(answer, correct_answer);
    }
}
