use regex::Regex;

fn main() {
    part1();
}

fn part1() {
    let input = include_str!("_part1.txt");
    let mut possible_games: Vec<Game> = Vec::new();
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    for line in input.lines() {
        let game = generate_game(line);
        if game.possible(&bag) {
            possible_games.push(game);
        }
    }
    for game in &possible_games {
        println!("Game {} is possible", game.id);
    }

    println!(
        "Sum of game ids: {}",
        possible_games.iter().map(|m| m.id).sum::<u32>()
    )
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn possible(&self, bag: &Bag) -> bool {
        for hand in &self.hands {
            if hand.red > bag.red || hand.green > bag.green || hand.blue > bag.blue {
                return false;
            }
        }
        return true;
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
        let input = include_str!("./_example1.txt");
        let mut possible_games: Vec<Game> = Vec::new();

        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        for line in input.lines() {
            let game = generate_game(line);
            if game.possible(&bag) {
                possible_games.push(game);
            }
        }
        assert_eq!(possible_games.len(), 3);
        let known_game_ids = vec![1, 2, 5];
        for game in &possible_games {
            assert!(known_game_ids.contains(&game.id));
        }
        for known_game_id in known_game_ids {
            assert!(&possible_games.iter().any(|g| g.id == known_game_id));
        }
    }
}
