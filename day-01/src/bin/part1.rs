fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let mut values: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut left = 0;
        let mut right = 0;

        for char in line.chars() {
            if char.is_numeric() {
                left = char.to_digit(10).unwrap();
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                right = char.to_digit(10).unwrap();
                break;
            }
        }

        let result = format!("{}{}", left, right).parse::<u32>().unwrap();
        values.push(result);
    }
    return values.iter().sum();
}

#[cfg(test)]
fn test1() -> u32 {
    let input = include_str!("./example1.txt");
    return part1(&input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let result = test1();
        assert_eq!(result, 142);
    }
}

#[cfg(test)]
mod tests_result {
    #[test]
    fn part1_true_result() {
        use super::*;
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, 53194);
    }
}
