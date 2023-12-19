use std::collections::HashMap;
use std::fmt::Display;
use itertools::Itertools;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};
use crate::tools::parse_numbers;
use std::collections::VecDeque;

pub fn part_one(input: String) -> impl Display {
    dig_it_up(input, true, true)
}

pub fn part_two(input: String) -> impl Display {
    0
}


fn dig_it_up(input: String, run_fill: bool, print_grid: bool) -> usize {
    let instructions:Vec<Instruction> = input.lines().map(|l| Instruction::parse(l)).collect();
    let mut dig_map = DigMap { map: HashMap::new(), current_position: Point::new(0,0), vertices: vec![]};
    instructions.iter().for_each(|inst| dig_map.process(inst));

    if run_fill {
        dig_map.flood_fill_efficient();
    }

    if print_grid {
        dig_map.print_grid();
    }

    dig_map.map.len()

}

fn dig_it_up_advanced(input: String, run_fill: bool, print_grid: bool) -> usize {
    let instructions:Vec<Instruction> = input.lines().map(|l| Instruction::parse_advanced_instruction(l)).collect();
    let mut dig_map = DigMap { map: HashMap::new(), current_position: Point::new(0,0), vertices: vec![ Point::new(0,0)]};
    instructions.iter().for_each(|inst| dig_map.process_advanced(inst));
    if run_fill {
        dig_map.fill();
    }

    for vertex in dig_map.vertices {
        println!("{}", vertex);
    }


    0
}

struct DigMap {
    map: HashMap<Point, String>,
    current_position: Point,
    vertices: Vec<Point>
}
impl DigMap {
    fn process(&mut self, instruction: &Instruction) {
        for i in 1..=instruction.amount {
            self.current_position = self.current_position.clone() + instruction.direction;
            self.map.insert(self.current_position, instruction.color.to_string());
        }
        self.vertices.push(self.current_position);
    }

    fn process_advanced(&mut self, instruction: &Instruction) {
        self.vertices.push(*self.vertices.last().unwrap() + instruction.direction.scale(instruction.amount));
    }

    fn calculate_area(&mut self) {
        let mut queue:VecDeque<(i32, i32, i32, i32)> = VecDeque::new();

    }

    fn flood_fill_efficient(&mut self) {
        let mut queue:VecDeque<(i32, i32, i32, i32)> = VecDeque::new();

        let start_x = self.vertices.last().unwrap().x;
        let start_y = self.vertices.last().unwrap().y;

        queue.push_back((start_x, start_x, start_y, 1));
        queue.push_back((start_x, start_x, start_y - 1, -1));

        while let Some((mut x1,mut x2, y, dy)) = queue.pop_front() {
            let mut x = x1;
            if self.inside(Point::new(x, y)) {
                while self.inside(Point::new(x - 1, y)) {
                    self.map.insert(Point::new(x - 1, y), "#FFFFFF".to_string());
                    x = x - 1;
                }
                if x < x1 {
                    queue.push_back((x, x1 - 1, y - dy, -dy));
                }
            }
            while x1 <= x2 {
                while self.inside(Point::new(x1, y)) {
                    self.map.insert(Point::new(x - 1, y), "#FFFFFF".to_string());
                    x1 += 1;
                }
                if x1 > x {
                    queue.push_back((x, x1 - 1, y + dy, dy));
                }
                if x1 - 1 > x2 {
                    queue.push_back((x2 + 1, x1 - 1, y - dy, -dy));
                }
                x1 = x1 + 1;
                while x1 < x2 && !self.inside(Point::new(x1, y)) {
                    x1 = x1 + 1;
                }
                x = x1;
            }
        }

    }

    fn fill(&mut self) {
        let mut queue = VecDeque::new();
        println!("{}", self.current_position);
        queue.push_back(self.current_position.clone());
        queue.push_back(self.current_position.clone() + NORTH);
        queue.push_back(self.current_position.clone() + SOUTH);
        queue.push_back(self.current_position.clone() + EAST);
        queue.push_back(self.current_position.clone() + WEST);
        queue.push_back(self.current_position.clone() + NORTH + EAST);
        queue.push_back(self.current_position.clone() + SOUTH + WEST);
        queue.push_back(self.current_position.clone() + EAST + SOUTH);
        queue.push_back(self.current_position.clone() + WEST + NORTH);

        while let Some(next) = queue.pop_front() {

            if !&self.inside(next) {
                // println!("Inside {}", next);
                continue;
            }
            if self.map.contains_key(&next) {
                continue;
            }

            self.map.insert(next, "#FFFFFF".to_string());
            let neighbors = next.get_neighbors();
            for neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }
    }

    fn inside(&self, position: Point) -> bool {
        let mut is_inside = false;

        if self.map.contains_key(&position) {
            println!("found {}", position);
            return true;
        }

        let mut j = self.vertices.len() - 1;

        for i in 0..self.vertices.len() {
            let p1 = self.vertices[i];
            let p2 = self.vertices[j];

            if p1.y < position.y && p2.y >= position.y || p2.y < position.y && p1.y >= position.y {
                if p1.x + (position.y - p1.y) / (p2.y - p1.y) * (p2.x - p1.x) < position.x
                {
                    is_inside = !is_inside;
                }
            }

            j = i;
        }

        println!("{} {}", position, is_inside);

        return is_inside
    }

    fn print_grid(&self) {
        let min_x = self.vertices.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = self.vertices.iter().max_by_key(|p| p.x).unwrap().x;
        let min_y = self.vertices.iter().min_by_key(|p| p.y).unwrap().y;
        let max_y = self.vertices.iter().max_by_key(|p| p.y).unwrap().y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(val) = self.map.get(&Point::new(x,y)) {
                    print!("{}", ansi_hex_color::colored(val, "", "#"));
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

struct Instruction {
    direction: Point,
    amount: i32,
    color: String
}

impl Instruction {
    fn parse(input_line: &str) -> Self {
        let mut split = input_line.split_whitespace();
        let direction_str = split.nth(0).unwrap();
        let direction = match direction_str {
            "D" => { SOUTH }
            "U" => { NORTH }
            "L" => { WEST}
            "R" => { EAST }
            &_ => panic!("not parsable")
        };

        let amount = parse_numbers(split.nth(0).unwrap()).unwrap().1;
        let color = split.nth(0).unwrap().replace("(", "").replace(")", "");

        Instruction { direction, amount, color: color.to_string() }
    }

    fn parse_advanced_instruction(input_line: &str) -> Self {
        let mut split = input_line.split_whitespace();
        let hex_value = split.nth(2).unwrap().replace("(", "").replace(")", "");
        let color = hex_value.to_string();


        let distance:String = hex_value.replace("#", "").chars().take(5).collect();
        let amount = i32::from_str_radix(&distance, 16).unwrap();
        let direction_char = hex_value.replace("#", "").chars().skip(5).take(1).at_most_one().unwrap();
        let direction = match direction_char.unwrap() {
            '1' => { SOUTH }
            '3' => { NORTH }
            '2' => { WEST}
            '0' => { EAST }
            _ => panic!("not parsable")
        };

        Instruction { color, direction, amount }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_18::{dig_it_up, dig_it_up_advanced, Instruction};
    use crate::domain::point::SOUTH;

    #[test]
    fn can_dig_outline_tiles() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        let result = dig_it_up(input.to_string(), false, true);

        assert_eq!(result, 38);
    }
    #[test]
    fn can_empty_out_tiles() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        let result = dig_it_up(input.to_string(), true, true);

        assert_eq!(result, 62);
    }

    #[test]
    fn can_parse_advanced() {
        let input = r#"D 5 (#0dc571)"#;

        let instruction = Instruction::parse_advanced_instruction(input);

        assert_eq!(instruction.amount, 56407);
        assert_eq!(instruction.direction, SOUTH);
        assert_eq!(instruction.color, "#0dc571");
    }

    #[test]
    fn can_calculate_dig_area_advanced() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        let result = dig_it_up_advanced(input.to_string(), false, false);

        assert_eq!(result, 952408144115);
    }
}