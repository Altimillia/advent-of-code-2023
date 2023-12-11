use std::collections::HashMap;
use std::fmt::Display;
use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let mut space_image = SpaceImage::parse(input);;
    space_image = space_image.expand_empty(1);
    get_combined_distances(space_image.get_galaxy_positions())
}

pub fn part_two(input: String) -> impl Display {
    let mut space_image = SpaceImage::parse(input);;
    let galaxies = space_image.get_expanded_galaxies(1000000);
    get_combined_distances(galaxies)
}

fn get_combined_distances(positions: Vec<Point>) -> i64 {
    let mut running_total = 0;
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            running_total += positions[i].manhattan_distance(positions[j]) as i64;
        }
    }

    running_total
}

struct SpaceImage {
    grid: HashMap<Point, char>,
    size: Point
}

impl SpaceImage {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, char> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        for (y, line) in input.lines().enumerate() {
            let mut x_index = 0;
            for (x, node) in line.chars().enumerate() {
                map.insert(Point::new(x_index, y as i32), node);
                x_index = x_index + 1;
            }

        }

        SpaceImage { grid: map, size: total_size }
    }

    fn expand_empty(&self, amount: i32) -> Self {
        let mut empty_rows:Vec<i32> = Vec::new();
        let mut empty_columns:Vec<i32> = Vec::new();
        let mut galaxies = self.get_galaxy_positions();

        for y in 0..self.size.y {
            let mut empty_row:bool = true;
            for x in 0..self.size.x {
                if(self.grid.get(&Point::new(x,y)).unwrap() == &'#') {
                    empty_row = false;
                }
            }

            if empty_row  {
                empty_rows.push(y);
            }
        }

        for x in 0..self.size.x {
            let mut empty_column:bool = true;
            for y in 0..self.size.y {
                if self.grid.get(&Point::new(x,y)).unwrap() == &'#' {
                    empty_column = false;
                }
            }

            if empty_column  {
                empty_columns.push(x);
            }
        }



        let mut space_image = SpaceImage { grid: self.grid.clone(), size: self.size };

            for row_index in 0..empty_rows.len() {
                space_image = space_image.add_empty_row(empty_rows[row_index] + (row_index as i32 * amount));
            }

            for col_index in 0..empty_columns.len() {
                for i in 0..amount {
                    space_image = space_image.add_empty_column(empty_columns[col_index] + (col_index as i32 * amount));
                }
            }

        space_image
    }

    fn get_expanded_galaxies(&self, amount: i32) -> Vec<Point> {

        let mut expand_amount = amount - 1;
        let mut empty_rows:Vec<i32> = Vec::new();
        let mut empty_columns:Vec<i32> = Vec::new();
        let mut galaxies = self.get_galaxy_positions();

        for y in 0..self.size.y {
            let mut empty_row:bool = true;
            for x in 0..self.size.x {
                if(self.grid.get(&Point::new(x,y)).unwrap() == &'#') {
                    empty_row = false;
                }
            }

            if empty_row  {
                empty_rows.push(y);
            }
        }

        for x in 0..self.size.x {
            let mut empty_column:bool = true;
            for y in 0..self.size.y {
                if self.grid.get(&Point::new(x,y)).unwrap() == &'#' {
                    empty_column = false;
                }
            }

            if empty_column  {
                empty_columns.push(x);
            }
        }

        let mut new_galaxy_positions:Vec<Point> = Vec::new();

        for galaxy in galaxies {
            let mut new_position:Point = galaxy.clone();
            for row_index in 0..empty_rows.len() {
                if galaxy.y > empty_rows[row_index] {
                    new_position = Point::new(new_position.x, new_position.y + expand_amount)
                }
            }

            for col_index in 0..empty_columns.len() {
                if galaxy.x > empty_columns[col_index] {
                    new_position = Point::new(new_position.x + expand_amount, new_position.y)
                }
            }

            new_galaxy_positions.push(new_position);
        }

        new_galaxy_positions
    }

    fn add_empty_row(&self, index: i32) -> Self {

        let mut new_grid:HashMap<Point, char> = HashMap::new();
        for i in 0..self.size.x {
            new_grid.insert(Point::new(i, index), '.');
        }

        for (point, char) in &self.grid {
            if point.y >= index  {
                new_grid.insert(Point::new(point.x, point.y + 1), *char);
            }
            else {
                new_grid.insert(Point::new(point.x, point.y), *char);
            }
        }

        SpaceImage { grid: new_grid, size: Point::new(self.size.x, self.size.y + 1)}
    }
    fn add_empty_column(&self, index: i32) -> Self {

        let mut new_grid:HashMap<Point, char> = HashMap::new();
        for i in 0..self.size.y {
            new_grid.insert(Point::new(index, i), '.');
        }

        for (point, char) in &self.grid {
            if point.x >= index  {
                new_grid.insert(Point::new(point.x + 1, point.y), *char);
            }
            else {
                new_grid.insert(Point::new(point.x, point.y), *char);
            }
        }

        SpaceImage { grid: new_grid, size: Point::new(self.size.x + 1, self.size.y)}
    }

    fn get_galaxy_positions(&self) -> Vec<Point> {
        let mut positions:Vec<Point> = Vec::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if self.grid.get(&Point::new(x, y)).unwrap() == &'#' {
                    positions.push(Point::new(x, y));
                }
            }
        }

        positions
    }

    fn print_out(&self) {
        for y in 0..self.size.y {

            for x in 0..self.size.x {
                print!("{}", self.grid.get(&Point::new(x, y)).unwrap())
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_11::{get_combined_distances, SpaceImage};

    #[test]
    fn image_can_expand_row_and_column() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        space_image.print_out();
        let new_space_image = space_image.add_empty_column(3);
        println!("");
        new_space_image.print_out();

        assert_eq!(new_space_image.size.x, space_image.size.x + 1);
    }

    #[test]
    fn image_can_expand_empty() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        space_image.print_out();
        let new_space_image = space_image.expand_empty(1);
        println!("");
        new_space_image.print_out();

        assert_eq!(new_space_image.size.y, space_image.size.y + 2);
        assert_eq!(new_space_image.size.x, space_image.size.x + 3);
    }

    #[test]
    fn can_measure_galaxy_distances() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        let new_space_image = space_image.expand_empty(1);
        let total = get_combined_distances(new_space_image.get_galaxy_positions());

        assert_eq!(total, 374);
    }

    #[test]
    fn can_measure_galaxy_distances_with_new_method() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        let galaxies = space_image.get_expanded_galaxies(2);
        let total = get_combined_distances(galaxies);

        assert_eq!(total, 374);
    }

    #[test]
    fn can_measure_galaxy_distances_with_10_scale() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        let galaxies = space_image.get_expanded_galaxies(10);
        let total = get_combined_distances(galaxies);

        assert_eq!(total, 1030);
    }

    #[test]
    fn can_measure_galaxy_distances_with_100_scale() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let space_image = SpaceImage::parse(input.to_string());

        let galaxies = space_image.get_expanded_galaxies(100);
        let total = get_combined_distances(galaxies);

        assert_eq!(total, 8410);
    }
}