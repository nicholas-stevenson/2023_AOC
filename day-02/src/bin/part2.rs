use regex::Regex;

fn main() {
    part2("src/bin/_part1.txt");
}

fn part2(file_path: &str) -> Vec<Game> {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");

    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let game = generate_game(line);
        games.push(game)
    }

    let mut games_sum: u32 = 0;
    for game in &games {
        let bag = game.minimum_viable();
        println!(
            "Game {}: red: {}, green: {}, blue: {}",
            game.id, bag.red, bag.green, bag.blue
        );
        println!("Power of dice: {}", bag.power_of_dice());
        games_sum += bag.power_of_dice()
    }

    println!("Games sum: {}", games_sum);

    games
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn power_of_dice(&self) -> u32 {
        let red = if self.red > 0 { self.red } else { 1 };
        let green = if self.green > 0 { self.green } else { 1 };
        let blue = if self.blue > 0 { self.blue } else { 1 };
        return red * green * blue;
    }
}
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}
impl Hand {}
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn minimum_viable(&self) -> Bag {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for hand in &self.hands {
            if hand.red > red {
                red = hand.red
            };
            if hand.green > green {
                green = hand.green
            };
            if hand.blue > blue {
                blue = hand.blue
            };
        }

        return Bag { red, green, blue };
    }
}

fn generate_game(game_str: &str) -> Game {
    let game_id_regex = Regex::new(r"Game (\d+):").unwrap();
    let hands_regex = Regex::new(r": (.*)").unwrap();
    let rgb_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let all_hands = hands_regex.captures(game_str).unwrap();
    let all_hands = all_hands[1].split(';').collect::<Vec<&str>>();
    let id = game_id_regex.captures(game_str).unwrap()[1]
        .parse::<u32>()
        .unwrap();

    let mut hands: Vec<Hand> = Vec::new();
    for hand_str in all_hands {
        let rgb_captures = rgb_regex.captures_iter(hand_str);
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for capture in rgb_captures {
            let count = capture[1].parse::<u32>().unwrap();
            let color = capture[2].to_string();
            match color.as_str() {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => panic!("Invalid color"),
            }
        }

        let hand = Hand { red, green, blue };
        hands.push(hand);
    }
    return Game { id, hands };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_correct_answer() {
        use super::*;
        let expected: Vec<u32> = vec![48, 12, 1560, 630, 36];
        let sum: u32 = expected.iter().sum();

        let games = part2("src/bin/_example1.txt");

        for (idx, game) in games.iter().enumerate() {
            let expected_value = expected.get(idx).unwrap();
            let bag = game.minimum_viable();
            assert_eq!(bag.power_of_dice(), *expected_value);
        }

        let mut games_sum = 0;
        for game in &games {
            let bag = game.minimum_viable();
            games_sum += bag.power_of_dice();
        }

        assert_eq!(games_sum, sum);
    }
}
