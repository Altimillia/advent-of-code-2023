use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use crate::domain::point::{NORTH, SOUTH, Point, WEST, EAST};

pub fn part_one(input: String) -> impl Display {
    let grid = Board::parse(input);
    let updated = grid.tilt_until_stopped(NORTH);
    updated.print();

    updated.get_board_load()
}

pub fn part_two(input: String) -> impl Display {
    let mut grid = Board::parse(input);
    // Get a sample after a set of iterations
    let updated = grid.spin_times(300);

    for i in 0..1000000000 - 300 {
        grid = updated.spin_cycle();
        // println!("{}", i);
        if grid.compare(&updated) {
            println!("{} true", i);
        }
    }
    updated.print();

    updated.get_board_load()
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

    fn compare(&self, other: &Board) -> bool {
        for y in 0..self.total_size.y {
            for x in 0..self.total_size.x {
                if self.grid.get(&Point::new(x,y)).unwrap() != other.grid.get(&Point::new(x,y)).unwrap() {
                    return false;
                }
            }
        }

        return true;
    }

    fn print(&self) {

        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                print!("{}", self.grid.get(&Point::new(x,y)).unwrap());
            }
            println!("");
        }
    }
    
    fn tilt(&self, direction: Point) -> (Self, bool) {
        let mut updated_grid:HashMap<Point, Entity> = HashMap::new();
        let mut movement_detected = false;


        let (y_direction, y_start, y_end) = if direction == SOUTH { (1, 0, self.total_size.y) } else { (-1, self.total_size.y - 1, -1)};
        let (x_direction, x_start, x_end) = if direction == WEST { (1, 0, self.total_size.x) } else { (-1, self.total_size.x - 1, -1)};
        // //
        let mut y_index = y_start;
        // // let (y_start, y_end) = if direction == SOUTH { ((0..self.total_size.y).start,(0..self.total_size.y).end) } else { ((0..self.total_size.y).end,(0..self.total_size.y).start) };
        // //
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

                        movement_detected = true;

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



        (Board { grid: updated_grid, total_size: self.total_size }, movement_detected)
    }


    fn tilt_until_stopped(&self, direction: Point) -> Self {

        let (mut tilting_board, _) = self.tilt(direction);

        tilting_board
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
        let mut updated = self.tilt_until_stopped(NORTH)
            .tilt_until_stopped(WEST)
            .tilt_until_stopped(SOUTH)
            .tilt_until_stopped(EAST);


        updated
    }

    fn spin_times(&self, amount:i32) -> Self {
        let mut end_result = self.spin_cycle();
        for i in 0..amount - 1 {
            end_result = end_result.spin_cycle();
        }

        return end_result
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

    #[test]
    fn board_can_spin_cycle_amount() {
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
        let spun = board.spin_times(3);

        spun.print();
        assert_eq!(spun.get_board_load(), 10);
    }

    #[test]
    fn boards_can_be_equal() {
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

        let board = Board::parse(input.to_string()).spin_times(3);

        let second_board = Board::parse(input.to_string()).spin_times(3);
        assert_eq!(board.compare(&second_board), true);
    }
}