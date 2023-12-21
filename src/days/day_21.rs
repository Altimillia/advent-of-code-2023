use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::mem;
use itertools::Itertools;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};
use rustc_hash::FxHashSet;

pub fn part_one(input: String) -> impl Display {
    let grid = Grid::parse(input);
    get_possible_end_spots(&grid, 64)
}

fn get_possible_end_spots(grid: &Grid, steps:i32) -> usize {
    let possible_paths = path_finding_with_culling(&grid, steps);

    let end_nodes:Vec<&Point> = possible_paths.iter().map(|path| path.path.last().unwrap()).collect::<HashSet<_>>().into_iter().collect();

    end_nodes.len()
}

pub fn part_two(input: String) -> impl Display {
    let grid = Grid::parse(input);
    quadratic(&grid)
}

fn quadratic(grid: &Grid) -> u64 {
    let [mut paths, mut next_paths] =
        [
            FxHashSet::<Point>::default(),
            FxHashSet::<Point>::default()
        ];
    let mut step_set = vec![];
    paths.insert(grid.start);
    let mut steps_taken = 0;

    while step_set.len() < 3 {
        println!("Steps Taken {}", steps_taken);
        steps_taken += 1;
        for node in &paths {
            let neighbors = grid.get_neighbors(*node);
            for neighbor in neighbors {
                next_paths.insert(neighbor);
            }

        }

        mem::swap(&mut paths, &mut next_paths);

        if steps_taken % 131 == 65 {
            step_set.push(paths.len());
        }
    }

    let n = 26501365 / 131;
    let a = step_set[0];
    let b = step_set[1];
    let c = step_set[2];

    let result = a + (b - a) * n + (a + c - 2 * b) * (n * (n - 1) / 2);

    result as u64
}

fn get_end_point_count(paths: &Vec<PathFit>) -> u64 {
    let end_nodes:Vec<&Point> = paths.iter().map(|path| path.path.last().unwrap()).collect::<HashSet<_>>().into_iter().collect();

    end_nodes.len() as u64
}

fn path_finding_with_culling(grid: &Grid, steps: i32) -> Vec<PathFit> {
    let mut paths = vec![PathFit { path: vec![grid.start]}];
    let mut steps_left = steps;

    while steps_left > 0 {
        let mut next_paths:Vec<PathFit> = vec![];
        steps_left -= 1;
        for path in paths {
            let current_node = path.get_last_step();
            let neighbors = grid.get_neighbors(current_node);

            for neighbor in neighbors {
                let mut branch = path.clone();
                branch.path.push(neighbor);
                if !next_paths.iter().any(|next| next.get_last_step() == branch.get_last_step()) {
                    next_paths.push(branch);
                }
            }

        }

        paths = next_paths;
    }

    paths

}

fn recursive_path_finding(path_fit: PathFit, grid: &Grid, current_point: Point, steps_left: i32) -> Vec<PathFit> {
    let neighbors = grid.get_neighbors(current_point);
    let mut paths:Vec<PathFit> = Vec::new();

    if steps_left == 0 {
        return vec![path_fit]
    }

    for neighbor in neighbors {
        let steps_after_move = steps_left - 1;
        let mut branch = path_fit.clone();
        branch.path.push(neighbor);

        paths.extend(recursive_path_finding(branch, grid, neighbor, steps_after_move));
    }

    paths
}

#[derive(Debug, Clone)]
struct PathFit {
    path: Vec<Point>
}

impl PathFit {
    fn get_last_step(&self) -> Point {
        *self.path.last().unwrap()
    }
}

struct Grid {
    nodes: HashMap<Point, char>,
    total_size: Point,
    start: Point
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut total_size: Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);
        let mut start = Point::new(0,0);
        // y_index = (total_size.y - 1) / 2;
        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            //let mut x_index = -((total_size.x - 1) / 2);
            let mut x_index = 0;
            for (x, node) in line.chars().enumerate() {
                if node == 'S' {
                    start = Point::new(x_index, y_index as i32);
                    println!("{}", start);
                }
                map.insert(Point::new(x_index, y_index as i32), node);
                x_index += 1;
            }
            y_index = y_index - 1;
        }

        Grid { nodes: map, total_size, start }
    }

    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let directions = [SOUTH, NORTH, EAST, WEST];
        let mut neighbor_points:Vec<Point> = Vec::new();
        let mut size = Point::new(self.total_size.x, self.total_size.y);
        directions.iter().for_each(|dir| {
            let p = *dir + pos;

            let scaled_x = p.x.rem_euclid(size.x);
            let scaled_y = p.y.rem_euclid(size.y);
            let scaled_p = Point::new(scaled_x, scaled_y);

            if let Some(node) = self.nodes.get(&scaled_p) {
                if node != &'#' {
                    neighbor_points.push(*dir + pos);
                }
            }
        });

        return neighbor_points;
    }
}
#[cfg(test)]
mod tests {
    use crate::days::day_21::{get_possible_end_spots, Grid};
    use crate::domain::point::Point;

    #[test]
    fn test_smaller_example() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

        let grid = Grid::parse(input.to_string());
        let end_result = get_possible_end_spots(&grid, 6);

        assert_eq!(end_result, 16);
    }

    #[test]
    fn can_get_neighbors_outside_of_range() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

        let grid = Grid::parse(input.to_string());

        let neighbors = grid.get_neighbors(Point::new(-11,0));

        assert_eq!(neighbors.len(), 4);
    }

    #[test]
    fn test_smaller_example_100() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

        let grid = Grid::parse(input.to_string());
        let end_result = get_possible_end_spots(&grid, 100);

        assert_eq!(end_result, 6536);
    }

    #[test]
    fn test_smaller_example_1000() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

        let grid = Grid::parse(input.to_string());
        let end_result = get_possible_end_spots(&grid, 36);

        assert_eq!(end_result, 216);
    }

}