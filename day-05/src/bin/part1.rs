// https://adventofcode.com/2023/day/5

const EXAMPLE: &str = "src/bin/example.txt";
const INPUT: &str = "src/bin/input.txt";

fn main() {
    let result = part1(INPUT);
    println!("Part 1: {}", result)
}

#[derive(Debug, PartialEq)]
enum MapsCascade {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MapsCascade {
    fn type_from_string(type_string: &str) -> MapsCascade {
        match type_string {
            "seed-to-soil" => MapsCascade::SeedToSoil,
            "soil-to-fertilizer" => MapsCascade::SoilToFertilizer,
            "fertilizer-to-water" => MapsCascade::FertilizerToWater,
            "water-to-light" => MapsCascade::WaterToLight,
            "light-to-temperature" => MapsCascade::LightToTemperature,
            "temperature-to-humidity" => MapsCascade::TemperatureToHumidity,
            "humidity-to-location" => MapsCascade::HumidityToLocation,
            _ => panic!("Unknown map name: {}", type_string),
        }
    }

    fn calculate_cascade(seed: u64, maps: &Vec<Map>) -> u64 {
        let seed_to_soil_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SeedToSoil)
            .unwrap();
        let soil_to_fertilizer_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SoilToFertilizer)
            .unwrap();
        let fertilizer_to_water_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::FertilizerToWater)
            .unwrap();
        let water_to_light_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::WaterToLight)
            .unwrap();
        let light_to_temperature_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::LightToTemperature)
            .unwrap();
        let temperature_to_humidity_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::TemperatureToHumidity)
            .unwrap();
        let humidity_to_location_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::HumidityToLocation)
            .unwrap();

        let soil = seed_to_soil_map.compute(seed);
        let fertilizer = soil_to_fertilizer_map.compute(soil);
        let water = fertilizer_to_water_map.compute(fertilizer);
        let light = water_to_light_map.compute(water);
        let temperature = light_to_temperature_map.compute(light);
        let humidity = temperature_to_humidity_map.compute(temperature);
        let location = humidity_to_location_map.compute(humidity);

        location
    }
}

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u64>,
}

impl Seeds {
    fn new(seeds: Vec<u64>) -> Seeds {
        Seeds { seeds: seeds }
    }
    fn from_line(line: &str) -> Seeds {
        let line_split = line.split_whitespace();
        // Get position 1 and beyond and parse it as a u64
        let seeds = line_split
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        Seeds { seeds: seeds }
    }
}

#[derive(Debug)]
struct Map {
    name: MapsCascade,
    map: Vec<Vec<u64>>,
}

impl Map {
    fn new(name: MapsCascade, map: Vec<Vec<u64>>) -> Map {
        Map { name, map }
    }
    fn from_lines(lines: Vec<&str>) -> Map {
        let name = MapsCascade::type_from_string(lines[0].split_whitespace().next().unwrap());
        let mut map: Vec<Vec<u64>> = Vec::new();

        for line in lines {
            if line.ends_with("map:") {
                continue;
            } else {
                let line_split = line.split_whitespace();
                let row: Vec<u64> = line_split.map(|x| x.parse::<u64>().unwrap()).collect();
                map.push(row);
            }
        }

        Map { name, map }
    }

    fn compute(&self, input: u64) -> u64 {
        if self.map.is_empty() {
            panic!("Map is empty, unable to compute input: {}", input)
        }

        for map in &self.map {
            let destination_range_start = map[0];
            let source_range_start = map[1];
            let range_length = map[2];

            if input >= source_range_start && input < source_range_start + range_length {
                let offset = input - source_range_start;
                return destination_range_start + offset;
            }
        }

        return input;
    }
}

fn part1(file_path: &str) -> u64 {
    let input = std::fs::read_to_string(file_path).expect("Failed to read file.");
    let (seeds, maps) = extract_data(input);

    let mut locations: Vec<u64> = Vec::new();

    for seed in seeds.seeds {
        let location = MapsCascade::calculate_cascade(seed, &maps);
        locations.push(location);
    }

    return locations.iter().cloned().min().unwrap();
}

fn extract_data(input: String) -> (Seeds, Vec<Map>) {
    let mut seeds: Seeds = Seeds::new(Vec::new());
    let mut maps: Vec<Map> = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();

        if line.starts_with("seeds:") {
            seeds = Seeds::from_line(line);
        } else if line.ends_with("map:") {
            let mut map_lines: Vec<&str> = Vec::new();
            map_lines.push(line);

            for line in lines.by_ref() {
                let line = line.trim();
                if line.is_empty() {
                    break;
                }
                map_lines.push(line);
            }
            maps.push(Map::from_lines(map_lines));
            // maps.push(Map.from_lines(map_lines));
        }
    }
    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Seed 79 should equal Soil 81
    #[test]
    fn test_seed_to_soil_79_eq_81() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        let seeds_to_soil_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SeedToSoil)
            .unwrap();

        assert_eq!(seeds_to_soil_map.compute(79), 81)
    }

    /// Seed 14 should equal Soil 14
    #[test]
    fn test_seed_to_soil_14_eq_14() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        let seeds_to_soil_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SeedToSoil)
            .unwrap();

        assert_eq!(seeds_to_soil_map.compute(14), 14)
    }

    /// Seed 55 should equal Soil 57
    #[test]
    fn test_seed_to_soil_55_eq_57() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        let seeds_to_soil_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SeedToSoil)
            .unwrap();

        assert_eq!(seeds_to_soil_map.compute(55), 57)
    }

    /// Seed 13 should equal Soil 13
    #[test]
    fn test_seed_to_soil_13_eq_13() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        let seeds_to_soil_map = maps
            .iter()
            .find(|x| x.name == MapsCascade::SeedToSoil)
            .unwrap();

        assert_eq!(seeds_to_soil_map.compute(13), 13)
    }

    #[test]
    fn test_seed_79_to_location_82() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        assert_eq!(MapsCascade::calculate_cascade(79, &maps), 82)
    }

    #[test]
    fn test_seed_14_to_location_43() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        assert_eq!(MapsCascade::calculate_cascade(14, &maps), 43)
    }

    #[test]
    fn test_seed_55_to_location_86() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        assert_eq!(MapsCascade::calculate_cascade(55, &maps), 86)
    }

    #[test]
    fn test_seed_13_to_location_35() {
        let input = std::fs::read_to_string(EXAMPLE).expect("Failed to read file.");
        let (_, maps) = extract_data(input);
        assert_eq!(MapsCascade::calculate_cascade(13, &maps), 35)
    }

    #[test]
    fn test_known_correct_answer() {
        let result = part1(INPUT);
        assert_eq!(result, 165788812)
    }
}
