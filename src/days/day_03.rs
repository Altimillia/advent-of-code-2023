use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use nom::{FindSubstring};
use crate::domain::point::Point;
use crate::tools::parse_numbers;

pub fn part_one(input: String) -> impl Display {
    let schematic = Schematic::process_schematic(input);
    schematic.get_part_number_sum()
}

pub fn part_two(input: String) -> impl Display {
    let schematic = Schematic::process_schematic(input);
    schematic.get_gears().iter().map(|gear| gear.get_gear_power()).sum::<i32>()
}

struct Gear {
    parts: Vec<i32>
}

impl Gear {
    pub fn get_gear_power(&self) -> i32 {
        let mut power = 1;
        self.parts.iter().for_each(|part| {
            power *= part;
        });
        power
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Symbol {
    indicator: char,
    position: Point
}

struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn process_schematic(input: String) -> Self {
        let lines = input.trim().lines();

        let mut parts: Vec<Part> = Vec::new();
        let mut symbol_positions: Vec<Symbol> = Vec::new();
        let mut y_index = 0;

        lines.for_each(|line| {
            let mut string_buffer = String::from("");

            for i in 0..line.chars().count() {
                let leading_char = line.chars().nth(i).unwrap();
                if leading_char.is_digit(10) {
                    string_buffer.push(leading_char);
                    continue;
                }
                if string_buffer.chars().count() > 0 {
                    parts.push(Part::new(string_buffer.clone(), i, y_index));

                    string_buffer.clear();
                }

                if leading_char != '.' {
                    symbol_positions.push(Symbol { position: Point::parse(i, y_index), indicator: leading_char });
                }
            }
            if string_buffer.chars().count() > 0 {
                parts.push(Part::new(string_buffer.clone(), line.chars().count(), y_index));

                string_buffer.clear();
            }

            y_index = y_index + 1;
        });


        Schematic { symbols: symbol_positions, parts }
    }


    pub fn get_valid_parts(self) -> Vec<Part> {
        let parts = self.parts.into_iter().filter(|part| {
            part.check_validity(self.symbols.clone())
        }).collect();
        return parts;
    }

    pub fn get_part_number_sum(self) -> i32 {
        self.get_valid_parts().iter()
            .map(|part| part.number)
            .sum::<i32>()
    }

    pub fn build_part_map(&self) -> HashMap<Point, i32> {
        let mut part_map: HashMap<Point, i32> = HashMap::new();

        for part in &self.parts {
            for position in &part.positions {
                part_map.insert(position.clone(), part.number);
            }
        }

        part_map
    }

    pub fn get_gears(&self) -> Vec<Gear> {
        let part_map = &self.build_part_map();
        let mut gears: Vec<Gear> = Vec::new();
        for symbol in &self.symbols {
            if(symbol.indicator != '*'){
                continue;
            }
            let mut gear_parts: HashSet<i32> = HashSet::new();
            for neighbor in symbol.position.get_neighbors() {
                match part_map.get(&neighbor){
                    None => {}
                    Some(value) => { gear_parts.insert(*value); }
                }
            }

            if(gear_parts.iter().count() == 2)
            {
                gears.push(Gear { parts: gear_parts.iter().map(|number| *number).collect() });
            }
        }

        gears
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Part {
    number: i32,
    positions: Vec<Point>,
}

impl Part {
    pub fn new(number_value: String, x_position_end: usize, y_position_start: usize) -> Self {
        let part_number = parse_numbers(&*number_value).unwrap().1;
        let string_size = number_value.chars().count();
        let mut positions: Vec<Point> = Vec::new();
        for i in 0..string_size {
            positions.push(Point::parse(x_position_end - i - 1, y_position_start));
        }

        Part { positions, number: part_number }
    }

    pub fn get_possible_symbol_positions(&self) -> Vec<Point> {
        let mut point_hash: HashSet<Point> = HashSet::new();
        for position in self.positions.iter() {
            for neighbor in position.get_neighbors() {
                if (!self.positions.contains(&neighbor)) {
                    point_hash.insert(neighbor);
                }
            }
        }

        return point_hash.into_iter().collect();
    }

    pub fn check_validity(&self, symbols: Vec<Symbol>) -> bool {
        let possible_positions = self.get_possible_symbol_positions();
        let symbol_positions:Vec<Point> = symbols.iter().map(|symbol| symbol.position).collect();
        for possible_position in possible_positions {
            if(symbol_positions.contains(&possible_position)){
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::point::Point;
    use super::{Schematic, Part};
    #[test]
    fn schematic_can_parse() {
        let input = r#"
467..114.6
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664*598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        assert_eq!(schematic.parts.iter().count(), 11);
    }

    #[test]
    fn schematic_can_get_valid_parts() {
        let input = r#"
467..114.6
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        let valid_parts = schematic.get_valid_parts();
        assert_eq!(valid_parts.iter().count(), 8);
    }

    #[test]
    fn schematic_can_get_valid_part_sum() {
        let input = r#"
467..114.6
...*.....*
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664*598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        let sum = schematic.get_part_number_sum();
        assert_eq!(sum, 4367);
    }

    #[test]
    fn schematic_can_build_part_map() {
        let input = r#"
467..114.6
...*.....*
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664*598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        let part_map = schematic.build_part_map();
        let part_at = part_map.get(&Point::new(0,0)).unwrap();
        assert_eq!(part_at.clone(), 467);
    }

    #[test]
    fn schematic_can_get_gears() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        let gears = schematic.get_gears();
        assert_eq!(gears.iter().count(), 2);
    }

    #[test]
    fn schematic_gear_power_is_calculated() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let schematic = Schematic::process_schematic(input.to_string());
        let gears = schematic.get_gears();
        assert_eq!(gears.iter().map(|gear| gear.get_gear_power()).sum::<i32>(), 467835);
    }
}
