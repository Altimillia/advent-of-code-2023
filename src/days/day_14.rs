use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use crate::domain::point::{NORTH, SOUTH, Point, WEST, EAST};

pub fn part_one(input: String) -> impl Display {
    let mut grid = Board::parse(input);
    let update = grid.tilt(NORTH);
    update.print();
    update.get_board_load()
}

pub fn part_two(input: String) -> impl Display {
    let mut grid = Board::parse(input);
    let mut seen = vec![grid.grid.clone()];

    loop {
        grid = grid.spin_cycle();
        if let Some(position) = seen.iter().position(|x| x == &grid.grid) {
            let cycle_len = seen.len() - position;
            let remaining = position + (1_000_000_000 - position) % cycle_len;

            println!("{}", weight_grid(&seen[remaining]));
            return weight_grid(&seen[remaining]);
        }

        seen.push(grid.grid.clone());
    }
}

fn weight_grid(grid: &HashMap<Point, Entity>) -> i32 {
    let mut running_total = 0;
    grid.iter().for_each(|(pos, entity)| {
        if matches!(entity, Entity::Sphere) {
            running_total += pos.y + 1;
        }
    });

    running_total
}


#[derive(Copy, Clone, PartialEq, Eq)]
enum Entity {
    Sphere,
    Cube,
    Empty
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entity::Sphere => write!(f, "O"),
            Entity::Cube => write!(f, "#"),
            Entity::Empty => write!(f, "."),
        }
    }
}


struct Board {
    grid: HashMap<Point, Entity>,
    total_size: Point
}

impl Board {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, Entity> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {
                let entity = match node {
                    '#' => Entity::Cube,
                    'O' => Entity::Sphere,
                    _ => Entity::Empty
                };

                map.insert(Point::new(x as i32, y_index as i32), entity);
            }
            y_index = y_index - 1;

        }

        Board { grid: map, total_size }
    }

    fn print(&self) {

        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                print!("{}", self.grid.get(&Point::new(x,y)).unwrap());
            }
            println!("");
        }
    }

    
    fn tilt(&self, direction: Point) -> Self {
        let mut updated_grid:HashMap<Point, Entity> = HashMap::new();


        let (y_direction, y_start, y_end) = if direction == SOUTH { (1, 0, self.total_size.y) } else { (-1, self.total_size.y - 1, -1)};
        let (x_direction, x_start, x_end) = if direction == WEST { (1, 0, self.total_size.x) } else { (-1, self.total_size.x - 1, -1)};

        let mut y_index = y_start;
        while y_index != y_end {
            let mut x_index = x_start;
            while x_index != x_end {
                let current_point = Point::new(x_index, y_index);
                let current_entity = self.grid.get(&current_point).unwrap();

                if !matches!(current_entity, Entity::Sphere) {
                    // No Movement
                    updated_grid.insert(current_point, *current_entity);
                    x_index += x_direction;
                    continue;
                }

                let mut destination = current_point + direction;
                updated_grid.insert(current_point, Entity::Empty);
                if updated_grid.contains_key(&destination) && matches!(updated_grid.get(&destination).unwrap(), Entity::Empty) {
                    while updated_grid.contains_key(&destination) && matches!(updated_grid.get(&destination).unwrap(), Entity::Empty) {
                        updated_grid.remove(&destination);
                        updated_grid.insert(destination, *current_entity);
                        updated_grid.insert(destination - direction, Entity::Empty);
                        destination = destination + direction;
                    }
                }
                else {
                    updated_grid.insert(current_point, *current_entity);
                }

                x_index += x_direction;
            }

            y_index += y_direction;
        }

        Board { grid: updated_grid, total_size: self.total_size }
    }


    fn get_board_load(&self) -> i32 {
        let mut running_total = 0;
        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                let entity = self.grid.get(&Point::new(x,y)).unwrap();
                if matches!(entity, Entity::Sphere) {
                    running_total += y + 1
                }
            }
        }

        running_total
    }

    fn spin_cycle(&self) -> Self {
        let mut updated = self.tilt(NORTH)
            .tilt(WEST)
            .tilt(SOUTH)
            .tilt(EAST);


        updated
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_14::Board;
    use crate::domain::point::NORTH;

    #[test]
    fn can_get_board_load() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let board = Board::parse(input.to_string());
        let tilted = board.tilt_until_stopped(NORTH);

        assert_eq!(tilted.get_board_load(), 136);
    }

    #[test]
    fn board_can_spin_cycle() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let board = Board::parse(input.to_string());
        let spun = board.spin_cycle();

        spun.print();
        assert_eq!(spun.get_board_load(), 10);
    }


}