/// https://adventofcode.com/2023/day/3

fn main() {
    let board = part1("src/bin/_part1.txt");
    let groups = board.valid_groups();
    // println!("Groups: {:?}", groups);
    println!("Sum: {}", board.sum());
}

struct Board {
    board: Vec<Vec<Square>>,
}

impl Board {
    fn width(&self) -> usize {
        self.board[0].len() - 1
    }
    fn height(&self) -> usize {
        self.board.len() - 1
    }

    fn valid_groups(&self) -> Vec<u32> {
        let mut groups: Vec<Vec<&Square>> = Vec::new();
        let mut current_group: Vec<&Square> = Vec::new();

        for x in 0..self.height() + 1 {
            for y in 0..self.width() + 1 {
                let square = &self.board[x][y];
                if (square.is_space() || square.is_symbol()) && !current_group.is_empty() {
                    groups.push(current_group);
                    current_group = Vec::new();
                }
                if square.is_number() && !self.is_invalid(square, None) {
                    current_group.push(square);
                }
            }
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        }

        if !current_group.is_empty() && !groups.contains(&current_group) {
            groups.push(current_group);
        }

        let mut merged_groups: Vec<u32> = Vec::new();

        for group in groups {
            let combined: String = group.iter().map(|x| x.value).collect();
            merged_groups.push(combined.parse::<u32>().unwrap())
        }

        merged_groups
    }

    fn sum(&self) -> u32 {
        let sum = self.valid_groups().iter().sum();
        sum
    }

    /// Returns a vector of non-space squares surrounding the source square,
    fn get_surrounding(&self, source: &Square) -> Vec<&Square> {
        let mut surrounding: Vec<&Square> = Vec::new();
        let x = self.locate(source).0;
        let y = self.locate(source).1;

        // north
        if x > 0 {
            let north = &self.board[x - 1][y];
            if !north.is_number() {
                surrounding.push(north);
            }
        }
        // south
        if x < self.height() {
            let south = &self.board[x + 1][y];
            if !south.is_number() {
                surrounding.push(south);
            }
        }
        // west
        if y > 0 {
            surrounding.push(&self.board[x][y - 1]);
        }
        // east
        if y < self.width() {
            surrounding.push(&self.board[x][y + 1]);
        }
        // north-east
        if x > 0 && y < self.width() {
            let north_east = &self.board[x - 1][y + 1];
            if !north_east.is_number() {
                surrounding.push(north_east);
            }
        }
        // north-west
        if x > 0 && y > 0 {
            let north_west = &self.board[x - 1][y - 1];
            if !north_west.is_number() {
                surrounding.push(north_west);
            }
        }
        // south-east
        if x < self.height() && y < self.width() {
            let south_east = &self.board[x + 1][y + 1];
            if !south_east.is_number() {
                surrounding.push(south_east);
            }
        }
        // south-west
        if x < self.height() && y > 0 {
            let south_west = &self.board[x + 1][y - 1];
            if !south_west.is_number() {
                surrounding.push(south_west);
            }
        }

        // filter out any squares that are is_space()
        let filtered: Vec<&Square> = surrounding
            .iter()
            .filter(|&x| !x.is_space())
            .copied()
            .collect();

        filtered
    }
    fn get_neighboring(&self, source: &Square, exclude: Option<Vec<&Square>>) -> Vec<&Square> {
        let mut neighboring: Vec<&Square> = Vec::new();
        let x = self.locate(source).0;
        let y = self.locate(source).1;

        // east
        if y < self.width() {
            neighboring.push(&self.board[x][y + 1]);
        }
        // west
        if y > 0 {
            neighboring.push(&self.board[x][y - 1]);
        }

        if let Some(exclude) = exclude {
            for exclude in exclude {
                let index = neighboring
                    .iter()
                    .position(|x| x.position == exclude.position)
                    .unwrap();
                neighboring.remove(index);
            }
        }

        neighboring
    }
    /// Returns the position of a square on the board.
    fn locate(&self, square: &Square) -> (usize, usize) {
        for (x, x_square) in self.board.iter().enumerate() {
            for (y, y_square) in x_square.iter().enumerate() {
                if std::ptr::eq(y_square, square) {
                    return (x, y);
                }
            }
        }
        panic!("Square not found.");
    }

    fn is_invalid(&self, square: &Square, exclude: Option<&Square>) -> bool {
        let mut surrounding = self.get_surrounding(square);

        if !square.is_number() {
            return false;
        }

        if let Some(exclude) = exclude {
            surrounding.retain(|&x| x != exclude);
        }

        for surround in &surrounding {
            if surround.is_symbol() {
                return false;
            }
        }

        for surround in &surrounding {
            if surround.is_number() && !self.is_invalid(surround, Some(square)) {
                return false;
            }
        }
        true
    }
}

#[derive(PartialEq)]
struct Square {
    value: char,
    position: (usize, usize),
}

impl Square {
    /// Returns true if the square is a blank.
    fn is_space(&self) -> bool {
        self.value == '.'
    }
    /// Returns true if the square is a special character.
    fn is_symbol(&self) -> bool {
        return !self.is_space() && !self.is_number();
    }
    /// Returns true if the square is a number.
    /// This is the only valid square type.
    /// All other types are invalid.
    fn is_number(&self) -> bool {
        let numbers: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        numbers.contains(&self.value)
    }
}

/// Converts a string of lines into a vector of vectors of squares.
fn lines_to_vec_vec(lines: String) -> Vec<Vec<Square>> {
    let mut lines_vec: Vec<Vec<Square>> = Vec::new();
    for (x, line) in lines.lines().enumerate() {
        let mut y_vec: Vec<Square> = Vec::new();
        for (y, character) in line.chars().enumerate() {
            let square = Square {
                value: character,
                position: (x, y),
            };
            y_vec.push(square)
        }
        lines_vec.push(y_vec);
    }
    lines_vec
}

/// Reads a file and returns a board.
fn part1(file_path: &str) -> Board {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");
    let board = Board {
        board: lines_to_vec_vec(input),
    };

    board
}

#[cfg(test)]
mod test {
    #[test]
    fn test_board_width_height() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert_eq!(board.width(), 9);
        assert_eq!(board.height(), 9);
    }

    #[test]
    fn test_location_query() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        // random number between 0 and 10
        assert_eq!(board.locate(&board.board[0][0]), (0, 0));
        assert_eq!(board.locate(&board.board[9][9]), (9, 9));
        assert_eq!(board.locate(&board.board[0][9]), (0, 9));
        assert_eq!(board.locate(&board.board[5][5]), (5, 5));
        assert_eq!(board.locate(&board.board[3][7]), (3, 7));
    }

    #[test]
    fn test_space_squares() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert!(!board.board[0][0].is_space()); // is 4
        assert!(!board.board[3][6].is_space()); // is #

        assert!(board.board[0][3].is_space()); // is *
        assert!(board.board[5][2].is_space()); // is *
        assert!(board.board[7][1].is_space()); // is *
    }

    #[test]
    fn test_symbol_squares() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert!(!board.board[0][0].is_symbol()); // is 4
        assert!(!board.board[0][3].is_symbol()); // is *
        assert!(!board.board[5][2].is_symbol()); // is *
        assert!(!board.board[7][1].is_symbol()); // is *

        assert!(board.board[3][6].is_symbol()); // is #
        assert!(board.board[8][3].is_symbol()); // is #
        assert!(board.board[8][5].is_symbol()); // is #
    }

    #[test]
    fn test_known_square_states() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert!(!board.is_invalid(&board.board[0][0], None)); // number 4 and invalid
        assert!(!board.is_invalid(&board.board[9][9], None)); // is space
        assert!(!board.is_invalid(&board.board[0][9], None));
        assert!(!board.is_invalid(&board.board[5][5], None));
        assert!(!board.is_invalid(&board.board[3][7], None));
    }

    #[test]
    fn test_known_valid_squares() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert!(board.is_invalid(&board.board[0][5], None)); // 1
        assert!(board.is_invalid(&board.board[0][6], None)); // 1
        assert!(board.is_invalid(&board.board[0][7], None)); // 4

        assert!(board.is_invalid(&board.board[5][7], None)); // 5
        assert!(board.is_invalid(&board.board[5][8], None)); // 8
    }

    #[test]
    fn test_known_invalid_squares() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert!(!board.is_invalid(&board.board[0][0], None)); // 4
        assert!(!board.is_invalid(&board.board[0][1], None)); // 6
        assert!(!board.is_invalid(&board.board[0][2], None)); // 7

        assert!(!board.is_invalid(&board.board[6][2], None)); // 5
        assert!(!board.is_invalid(&board.board[6][3], None)); // 9
        assert!(!board.is_invalid(&board.board[6][4], None)); // 2

        assert!(!board.is_invalid(&board.board[9][5], None)); // 5
        assert!(!board.is_invalid(&board.board[9][6], None)); // 9
        assert!(!board.is_invalid(&board.board[9][7], None)); // 8
    }

    #[test]
    fn test_known_nearby() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        let sample_square = &board.board[0][0];
        let surrounding = board.get_surrounding(sample_square);
        assert_eq!(surrounding.len(), 1);

        let known_surrounding: Vec<&Square> = vec![&board.board[0][1]];
        for known in known_surrounding {
            assert!(surrounding.contains(&known));
        }
    }

    // From the example, there are a total of 10 groups of numbers where 2 are invalid
    // The invalid numbers from the example are 114 and 58
    // The known sum of all of these numbers is 4361
    // and with valid number groups being 467, 35, 633, 617, 592, 755, 664, 598

    #[test]
    fn test_known_groups_count() {
        use super::*;
        let board = part1("src/bin/_example1.txt");

        let groups = board.valid_groups();
        assert_eq!(groups.len(), 8);
    }

    #[test]
    fn test_known_valid_groups() {
        use super::*;
        let board = part1("src/bin/_example1.txt");

        let groups = board.valid_groups();
        let known_groups: Vec<u32> = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let mut missing_groups: Vec<u32> = Vec::new();
        for known in known_groups {
            if !groups.contains(&known) {
                missing_groups.push(known);
            }
        }
        if !missing_groups.is_empty() {
            panic!("Missing groups: {:?}", missing_groups);
        }
    }

    #[test]
    fn test_known_example_sum() {
        use super::*;
        let board = part1("src/bin/_example1.txt");
        assert_eq!(board.sum(), 4361);
    }

    #[test]
    fn test_2_known_example_sum() {
        // provided by redditor i_have_no_biscuits
        // https://www.reddit.com/r/adventofcode/comments/189q9wv/2023_day_3_another_sample_grid_to_use/
        // who provided another example grid to test your code against
        use super::*;
        let board = part1("src/bin/_example1_2.txt");
        let groups = board.valid_groups();
        assert_eq!(board.sum(), 925);
    }

    #[test]
    fn test_2_known_valid_groups() {
        use super::*;
        let board = part1("src/bin/_example1_2.txt");

        let groups = board.valid_groups();
        let known_groups: Vec<u32> = vec![
            12, 34, 777, 12, 78, 78, 9, 23, 90, 12, 2, 2, 12, 1, 1, 503, 56,
        ];
        let mut missing_groups: Vec<u32> = Vec::new();
        let mut extra_groups: Vec<u32> = Vec::new();
        for known in &known_groups {
            if !groups.contains(known) {
                missing_groups.push(known.to_owned());
            }
        }

        for group in groups {
            if !known_groups.contains(&group) {
                extra_groups.push(group.to_owned());
            }
        }

        if !missing_groups.is_empty() || !extra_groups.is_empty() {
            println!("Extra groups: {:?}", extra_groups);
            println!("Missing groups: {:?}", missing_groups);
            panic!();
        }
    }

    #[test]
    fn test_2_known_edgecases() {
        // In example 2, the number 5 is invalid but is surrounded by valid numbers
        // This test ensures that the number 5 is not counted as valid
        // The logic error is that we use the surrounding blocks to validate if a square is valid,
        // but we should not recursively check the surround, but only the neighboring

        use super::*;
        let board = part1("src/bin/_example2.txt");
        let groups = board.valid_groups();
        assert!(!groups.contains(&5));
    }

    #[test]
    fn test_2_bottom_right_corner_56() {
        use super::*;
        let board = part1("src/bin/_example2.txt");

        assert!(!board.is_invalid(&board.board[11][10], None)); // 5
        assert!(!board.is_invalid(&board.board[11][11], None)); // 6

        let groups = board.valid_groups();
        let known_group: u32 = 56;
        assert!(groups.contains(&known_group));
    }

    #[test]
    fn test_part1_known_invalid_answer() {
        use super::*;
        let board = part1("src/bin/_part1.txt");
        let sum = board.sum();
        let known_invalid = [538044];

        assert!(!known_invalid.contains(&sum))
    }

    #[test]
    fn part_1_solution() {
        // This test is to ensure that the solution is correct
        use super::*;
        let board = part1("src/bin/_part1.txt");
        let sum = board.sum();

        // for group in board.valid_groups() {
        //     println!("Group: {}", group);
        // }

        assert_eq!(sum, 560670, "Sum: {}  Expected: {}", sum, "560670");
    }
}
