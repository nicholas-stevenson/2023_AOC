const EXAMPLE1: &str = "src/bin/_example1.txt";
const PART1: &str = "src/bin/_part1.txt";

fn main() {
    let result = part1(PART1);
    println!("Part 1: {}", result)
}

fn part1(file_path: &str) -> u32 {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");
    let cards: Vec<Card> = input.lines().map(|x| line_to_card(x)).collect();
    return sum_cards(cards);
}

fn sum_cards(cards: Vec<Card>) -> u32 {
    let mut sum: u32 = 0;
    for card in cards.iter() {
        sum += card.score();
    }
    return sum;
}

fn line_to_card(line: &str) -> Card {
    let line_split: Vec<&str> = line.split(":").collect();
    let card_idx: Vec<&str> = line_split[0].split_whitespace().collect();
    let index = card_idx[card_idx.len() - 1].parse::<usize>().unwrap();

    let hand: Vec<&str> = line_split[1].split("|").collect();
    let winning: Vec<&str> = hand[0].split_whitespace().collect();
    let holding: Vec<&str> = hand[1].split_whitespace().collect();

    let winning: Vec<u32> = winning.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let holding: Vec<u32> = holding.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    println!("{:?}", line_split);

    return Card {
        index,
        winning,
        holding,
    };
}
struct Card {
    index: usize,
    winning: Vec<u32>,
    holding: Vec<u32>,
}

impl Card {
    fn new(index: usize, winning: Vec<u32>, holding: Vec<u32>) -> Self {
        Self {
            index,
            winning,
            holding,
        }
    }

    fn score(&self) -> u32 {
        let mut count: usize = 0;
        for h in self.holding.iter() {
            if self.winning.contains(&h) {
                count += 1;
            }
        }

        if count == 0 {
            return 0;
        } else if count == 1 {
            return 1;
        } else {
            return 2u32.pow(count as u32 - 1);
        }
    }
}

// test block
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = std::fs::read_to_string(EXAMPLE1).expect("Failed to read file.");
        for line in file.lines() {
            println!("{}", line);
        }
    }

    #[test]
    fn test_card1() {
        let file = std::fs::read_to_string(EXAMPLE1).expect("Failed to read file.");
        let line = file.lines().nth(0).unwrap();
        let card = line_to_card(line);
        let winning = vec![41, 48, 83, 86, 17];
        let holding = vec![83, 86, 6, 31, 17, 9, 48, 53];
        assert_eq!(card.index, 1);
        assert_eq!(card.winning, winning);
        assert_eq!(card.holding, holding);
        assert_eq!(card.score(), 8);
    }

    #[test]
    fn test_all_card() {
        let file = std::fs::read_to_string(EXAMPLE1).expect("Failed to read file.");
        let cards: Vec<Card> = file.lines().map(|x| line_to_card(x)).collect();
        let card_values: Vec<u32> = vec![8, 2, 2, 1, 0, 0];

        for (i, card) in cards.iter().enumerate() {
            assert_eq!(card.score(), card_values[i]);
        }
    }

    #[test]
    fn test_full_hand() {
        let file = std::fs::read_to_string(EXAMPLE1).expect("Failed to read file.");
        let cards: Vec<Card> = file.lines().map(|x| line_to_card(x)).collect();
        let hand = sum_cards(cards);

        assert_eq!(hand, 13);
    }
}
