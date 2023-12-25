use std::num::ParseIntError;

/// https://adventofcode.com/2023/day/3

fn main() {
    let board = part1("src/bin/_part1.txt");
    let gear_ratio_groups = board.get_gear_ratio_groups();
    let sum = sum_gear_ratios(&gear_ratio_groups);
    println!("Sum of gear ratios: {}", sum);
}

struct Board {
    squares: Vec<Vec<Square>>,
    groups: Vec<Group>,
}

impl Board {
    fn width(&self) -> usize {
        self.squares[0].len() - 1
    }
    fn height(&self) -> usize {
        self.squares.len() - 1
    }

    /// Returns a vector of non-space squares surrounding the source square,
    fn get_surrounding(&self, source: &Square) -> Vec<&Square> {
        let mut surrounding: Vec<&Square> = Vec::new();
        let (x, y) = source.position;

        // north
        if x > 0 {
            let north = &self.squares[x - 1][y];
            if north.is_number() {
                surrounding.push(north);
            }
        }
        // east
        if y < self.width() {
            let east = &self.squares[x][y + 1];
            if east.is_number() {
                surrounding.push(east);
            }
        }
        // south
        if x < self.height() {
            let south = &self.squares[x + 1][y];
            if south.is_number() {
                surrounding.push(south);
            }
        }
        // west
        if y > 0 {
            let west = &self.squares[x][y - 1];
            if west.is_number() {
                surrounding.push(west);
            }
        }
        // north-east
        if x > 0 && y < self.width() {
            let north_east = &self.squares[x - 1][y + 1];
            if north_east.is_number() {
                surrounding.push(north_east);
            }
        }
        // north-west
        if x > 0 && y > 0 {
            let north_west = &self.squares[x - 1][y - 1];
            if north_west.is_number() {
                surrounding.push(north_west);
            }
        }
        // south-east
        if x < self.height() && y < self.width() {
            let south_east = &self.squares[x + 1][y + 1];
            if south_east.is_number() {
                surrounding.push(south_east);
            }
        }
        // south-west
        if x < self.height() && y > 0 {
            let south_west = &self.squares[x + 1][y - 1];
            if south_west.is_number() {
                surrounding.push(south_west);
            }
        }

        surrounding
    }

    fn get_gear_squares(&self) -> Vec<&Square> {
        let mut gear_squares: Vec<&Square> = Vec::new();

        for x in 0..self.height() + 1 {
            for y in 0..self.width() + 1 {
                let square = &self.squares[x][y];
                if square.is_ratio() {
                    gear_squares.push(square);
                }
            }
        }

        gear_squares
    }

    /// Loops through all gear squares and returns vector of gear ratio groups.
    /// Any gear ratio square requires that at least two groups of numbers are adjacent to it.
    fn get_gear_ratio_groups(&self) -> Vec<Gear> {
        let mut gear_ratio_groups: Vec<Vec<&Group>> = Vec::new();
        let gear_squares = self.get_gear_squares();

        for gear_square in gear_squares {
            let mut gear_ratio_group: Vec<&Group> = Vec::new();

            let surrounding = self.get_surrounding(gear_square);
            for surround in surrounding {
                for group in &self.groups {
                    if group.squares.contains(&surround) && !gear_ratio_group.contains(&group) {
                        gear_ratio_group.push(group);
                    }
                }
            }

            if gear_ratio_group.len() > 1 {
                gear_ratio_groups.push(gear_ratio_group);
            }
        }

        let mut gears: Vec<Gear> = Vec::new();
        for group in gear_ratio_groups {
            let components: Vec<u32> = group.iter().map(|x| x.combined().unwrap()).collect();
            gears.push(Gear { components });
        }
        gears
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Group {
    squares: Vec<Square>,
}

impl Group {
    fn combined(&self) -> Result<u32, ParseIntError> {
        let combined: String = self.squares.iter().map(|x| x.value).collect();
        combined.parse::<u32>()
    }
}

fn sum_gear_ratios(groups: &Vec<Gear>) -> u32 {
    let mut sum = 0;
    for group in groups {
        sum += group.ratio();
    }
    sum
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Square {
    value: char,
    position: (usize, usize),
}

impl Square {
    fn is_space(&self) -> bool {
        self.value == '.'
    }
    fn is_ratio(&self) -> bool {
        self.value == '*'
    }
    fn is_symbol(&self) -> bool {
        return !self.is_space() && !self.is_number() && !self.is_ratio();
    }
    fn is_number(&self) -> bool {
        let numbers: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        numbers.contains(&self.value)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Gear {
    components: Vec<u32>,
}
impl Gear {
    fn ratio(&self) -> u32 {
        self.components.iter().product::<u32>()
    }
}

/// Converts a string of lines into a vector of vectors of squares.
fn lines_to_squares_groups(lines: &String) -> (Vec<Vec<Square>>, Vec<Group>) {
    let mut squares_vec: Vec<Vec<Square>> = Vec::new();
    for (x, line) in lines.lines().enumerate() {
        let mut y_vec: Vec<Square> = Vec::new();
        for (y, character) in line.chars().enumerate() {
            let square = Square {
                value: character,
                position: (x, y),
            };
            y_vec.push(square)
        }
        squares_vec.push(y_vec);
    }

    let mut groups_vec: Vec<Vec<Square>> = Vec::new();
    let mut current_group_squares: Vec<Square> = Vec::new();

    for x in 0..squares_vec.len() {
        for y in 0..squares_vec[x].len() {
            let square = &squares_vec[x][y];

            if (square.is_space() || square.is_symbol() || square.is_ratio())
                && !current_group_squares.is_empty()
            {
                groups_vec.push(current_group_squares);
                current_group_squares = Vec::new();
            }
            if square.is_number() {
                current_group_squares.push(*square);
            }
        }
        if !current_group_squares.is_empty() {
            groups_vec.push(current_group_squares);
            current_group_squares = Vec::new();
        }
    }

    let groups_vec: Vec<Group> = groups_vec
        .iter()
        .map(|x| Group {
            squares: x.to_vec(),
        })
        .collect();

    (squares_vec, groups_vec)
}

/// Reads a file and returns a board.
fn part1(file_path: &str) -> Board {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");
    let (squares, groups) = lines_to_squares_groups(&input);
    let board = Board { squares, groups };

    board
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1_known_gears() {
        let board = part1("src/bin/_example2.txt");
        let gear_ratio_groups = board.get_gear_ratio_groups();
        let known_ratios: Vec<u32> = vec![16345, 451490];

        for gear in gear_ratio_groups {
            println!("{:?}", gear.ratio());
            assert!(known_ratios.contains(&gear.ratio()));
        }
    }

    #[test]
    fn test_example1_known_gear_ratio_sum() {
        let board = part1("src/bin/_example2.txt");
        let gear_ratio_groups = board.get_gear_ratio_groups();

        let sum = sum_gear_ratios(&gear_ratio_groups);

        assert_eq!(sum, 467835);
    }

    #[test]
    fn test_part1_known_gear_ratio_sum() {
        let board = part1("src/bin/_part1.txt");
        let gear_ratio_groups = board.get_gear_ratio_groups();

        let sum = sum_gear_ratios(&gear_ratio_groups);

        assert_eq!(sum, 91622824);
    }

    #[test]
    fn test_example1_known_gear_squares() {
        let board = part1("src/bin/_example2.txt");

        // test known valid gear number groups
        for group in board.groups {
            println!("{:?}", group.combined().unwrap());
        }
    }
    #[test]
    fn test_example1_known_groups() {
        let board = part1("src/bin/_example2.txt");
        let known_groups: Vec<u32> = vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598];

        assert_eq!(board.groups.len(), known_groups.len());
    }
}
