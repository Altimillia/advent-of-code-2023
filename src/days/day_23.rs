use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use itertools::max;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};

pub fn part_one(input: String) -> impl Display {
    find_longest_path(input, false)
}

fn find_longest_path(input: String, part_2: bool) -> i32 {
    let grid = Grid::parse(input);
    grid.print_grid(&vec![]);

    let max_path = iterative_path_finding(PathFit { path: HashSet::new() }, &grid, grid.start, part_2);

    max_path
}

fn find_a_star_path(input: String, part_2: bool) -> i32 {
    let grid = Grid::parse(input);
    grid.print_grid(&vec![]);

    let path_length = a_star_path_finding(&grid, grid.start, part_2);

    path_length
}


pub fn part_two(input: String) -> impl Display {
    find_longest_path(input, true)
}

fn try_number_4(grid: &Grid) -> i32 {
    let mut stack: Vec<Point> = Vec::new();
    stack.push(grid.start);
    let mut cache_map:HashMap<Point, i32> = HashMap::new();
    let mut dp = vec![vec![0; grid.total_size.y as usize]; grid.total_size.x as usize];
    cache_map.insert(grid.start, 1);
    let mut max_path = 0;
    dp[grid.start.x as usize][grid.start.y as usize] = 1;

    for x in 0..grid.total_size.x {
        for y in 0..grid.total_size.y {
            let current_point = Point::new(x, y);
            if !grid.is_movable(&current_point) {
                continue
            }
            println!("{}", current_point);
            let neighbors = grid.get_neighbors_part2(current_point);
            for neighbor in neighbors {
                dp[neighbor.x as usize][neighbor.y as usize] = dp[neighbor.x as usize][neighbor.y as usize].max(dp[current_point.x as usize][current_point.y as usize] + 1);
                max_path = max_path.max(dp[neighbor.x as usize][neighbor.y as usize]);
            }
        }
    }
    max_path + 1
}

fn iterative_path_finding(path_fit: PathFit, grid: &Grid, start_point: Point, part_2: bool) -> i32 {

    let mut graph:FxHashMap<Point, Vec<Point>> = FxHashMap::default();

    for (key, _) in &grid.nodes {
        let neighbors = match part_2 {
            true => grid.get_neighbors_part2(*key),
            false => grid.get_neighbors(*key)
        };
        graph.insert(*key, neighbors.clone());
    }

    let mut stack: Vec<(PathFit, Point)> = Vec::new();
    stack.push((path_fit, start_point));

    let mut max_path:i32 = 0;
    let mut counter:u128 = 0;
    let mut solution_counter = 0;

    while let Some((current_path_fit, current_point)) = stack.pop() {
        counter += 1;

        if counter % 100000 == 0 {
            let max = stack.iter().map(|(p,_)| p.path.len()).max();
            println!("Iterations: {} - Max: {} - Solutions: {} Max In progress: {} for {}", counter, max_path, solution_counter, max.unwrap_or(0), stack.len());
        }

        let neighbors = graph.get(&current_point).unwrap();

        if current_path_fit.contains(&grid.end) {
            solution_counter += 1;
            max_path = max_path.max(current_path_fit.path.len() as i32);
            continue;
        }

        for neighbor in neighbors {
            if current_path_fit.path.contains(&neighbor) {
                continue;
            }

            let mut branch = current_path_fit.clone();
            branch.path.insert(*neighbor);

            stack.push((branch, *neighbor));
        }
    }

    max_path
}

// Im not sure A-Star is usable. How do you build towards the consensus?
fn a_star_path_finding(grid: &Grid, start_point: Point, part_2: bool) -> i32 {
    let mut frontier: PriorityQueue<Point, i32> = PriorityQueue::new();
    let mut closed: Vec<Point> = Vec::new();
    let mut cost_so_far: HashMap<Point, i32> = HashMap::new();
    let mut came_from = HashMap::new();

    frontier.push(start_point, 0);

    came_from.insert(start_point, None);
    cost_so_far.insert(start_point, 0);

    while frontier.len() > 0 {
        let popped = frontier.pop().unwrap();

        let current_pos = popped.0;

        closed.push(current_pos);

        if current_pos == grid.end {
            break;
        }


        let neighbors = match part_2 {
            true => grid.get_neighbors_part2(current_pos),
            false => grid.get_neighbors(current_pos)
        };
        for neighbor in neighbors {

            let cost = cost_so_far.get(&current_pos).unwrap() + 1;

            if frontier.get(&neighbor).is_some() && cost > *cost_so_far.get(&neighbor).unwrap() {
                frontier.remove(&neighbor);
            }
            //
            // if closed.contains(&neighbor) && cost > *cost_so_far.get(&neighbor).unwrap()  {
            //     closed.retain(|&x| x != neighbor);
            // }

            if !frontier.get(&neighbor).is_some() && !closed.contains(&neighbor) {
                cost_so_far.insert(neighbor, cost);
                let priority = cost + heuristic(neighbor, grid.end);
                frontier.push(neighbor, priority);
                came_from.insert(neighbor, Some(current_pos));
            }
        }
    }
    let mut path = Vec::new();
    let mut current = grid.end;
    while let Some(&Some(prev)) = came_from.get(&current) {
        path.push(current);
        current = prev;
    }
    path.push(grid.start);
    path.reverse();

    grid.print_grid(&path);

    return path.len() as i32
}

fn heuristic(a:Point, b:Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

// Recurisve failed because it caused a stack overflow
fn recursive_path_finding(path_fit: PathFit, grid: &Grid, current_point: Point) -> Vec<PathFit> {
    let neighbors = grid.get_neighbors(current_point);
    let mut paths: Vec<PathFit> = Vec::new();

    if path_fit.contains(&grid.end) {
        return vec![path_fit];
    }

    for neighbor in neighbors {
        if path_fit.path.contains(&neighbor) {
            continue;
        }

        let mut branch = path_fit.clone();
        branch.path.insert(neighbor);

        paths.extend(recursive_path_finding(branch, grid, neighbor));
    }

    paths
}


#[derive(Debug, Clone)]
struct PathFit {
    path: HashSet<Point>,
}

impl PathFit {
    fn contains(&self, pos: &Point) -> bool {
        self.path.contains(pos)
    }
}

struct Grid {
    nodes: HashMap<Point, char>,
    total_size: Point,
    start: Point,
    end: Point,
}

impl Grid {
    fn is_movable(&self, pos: &Point) -> bool {
        if let Some(node) = self.nodes.get(pos) {
            return node != &'#'
        }

        false
    }
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut total_size: Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);
        y_index -= 1;

        for (y, line) in input.lines().enumerate() {
            let mut x_index = 0;
            for (x, node) in line.chars().enumerate() {
                map.insert(Point::new(x_index, y_index), node);
                x_index += 1;
            }
            y_index = y_index - 1;
        }

        Grid { nodes: map, total_size, start: Point::new(1, total_size.y - 1), end: Point::new(total_size.x - 2, 0) }
    }

    fn get_neighbors_part2(&self, pos: Point) -> Vec<Point> {
        let directions: Vec<Point> = vec![NORTH, SOUTH, EAST, WEST];

        let mut neighbor_points: Vec<Point> = Vec::new();

        directions.iter().for_each(|dir| {
            if let Some(node) = self.nodes.get(&(*dir + pos)) {
                if node != &'#' {
                    neighbor_points.push(*dir + pos);
                }
            }
        });

        return neighbor_points;
    }


    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let current_node = self.nodes.get(&pos).unwrap();
        let directions: Vec<Point> = match current_node {
            &'^' => vec![NORTH],
            &'v' => vec![SOUTH],
            &'<' => vec![WEST],
            &'>' => vec![EAST],
            &_ => vec![SOUTH, NORTH, EAST, WEST]
        };

        let mut neighbor_points: Vec<Point> = Vec::new();

        directions.iter().for_each(|dir| {
            if let Some(node) = self.nodes.get(&(*dir + pos)) {
                if node != &'#' {
                    neighbor_points.push(*dir + pos);
                }
            }
        });

        return neighbor_points;
    }

    fn print_grid(&self, movement: &Vec<Point>) {
        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                if movement.contains(&Point::new(x, y)) {
                    print!("O");
                } else {
                    //print!("{}", self.map.get(&Point::new(x, y)).unwrap().heat_loss);
                    print!("{}", self.nodes.get(&Point::new(x, y)).unwrap());
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_23::{a_star_path_finding, find_a_star_path, find_longest_path};

    #[test]
    fn can_find_paths() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

        let result = find_longest_path(input.to_string(), false);

        assert_eq!(result, 94);
    }

    #[test]
    fn can_find_paths_part_2() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

        let result = find_longest_path(input.to_string(), true);

        assert_eq!(result, 154);
    }

    #[test]
    fn can_find_single_longest_path() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

        let result = find_a_star_path(input.to_string(), false);

        assert_eq!(result, 94);
    }
}