use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};
use priority_queue::{PriorityQueue};
use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};


pub fn part_one(input: String) -> impl Display {
    using_module(&input, 1, 3)
}

pub fn part_two(input: String) -> impl Display {
    using_module(&input, 4, 10)
}

fn part_one_finder(input: String) -> u32 {
    let mut grid = &mut Grid::parse(input);
    path_find(Point::new(0, grid.total_size.y - 1), Point::new(grid.total_size.x - 1, 0), &mut grid) as u32
}

fn using_module(input: &str, min_move: usize, max_move: usize) -> u32 {
    let grid = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10))),
    )
        .unwrap();
    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, (dr, dc), l)| {
            let mut next = Vec::with_capacity(3);
            let mut e = |dir, l| {
                next.extend(
                    &grid
                        .move_in_direction(pos, dir)
                        .map(|t| ((t, dir, l), grid[t])),
                );
            };
            if l < max_move {
                e((dr, dc), l + 1);
            }
            if l >= min_move {
                e((-dc, -dr), 1);
                e((dc, dr), 1);
            } else if l == 0 {
                e((1, 0), 1);
                e((0, 1), 1);
            }
            next
        },
        |&(pos, _, l)| pos == (grid.rows - 1, grid.columns - 1) && l >= min_move,
    )
        .unwrap()
        .1
}

fn breadth_first(start_point: Point, end_point: Point, grid: &mut Grid) -> i32 {
    let mut frontier:PriorityQueue<Point, i32> = PriorityQueue::new();
    let mut closed:Vec<Point> = Vec::new();
    grid.map.get_mut(&start_point).unwrap().cost = 0;

    let mut cost_map = HashMap::new();
    frontier.push(start_point, 0);
    while frontier.len() > 0 {
        let popped = frontier.pop().unwrap();

        let current_pos = popped.0;
        //
        if current_pos == end_point {
            break;
        }

        let mut current_cost = -popped.1;
        println!("{}", current_cost);
        let current_copy;
        {
            let current = &mut grid.get_node(current_pos);
            current_copy = current.clone();
        }

        closed.push(current_copy.position);

        // if current_copy.prev.is_some() {
        //     let mut direction_marker = &current_copy.clone();
        //     let mut counter = 0;
        //     let direction = current_copy.position - current_copy.prev.unwrap();
        //     while direction_marker.prev.is_some() && counter <= 4 {
        //         if direction != direction_marker.position - direction_marker.prev.unwrap() {
        //             break;
        //         }
        //         direction_marker = grid.get_node(direction_marker.prev.unwrap());
        //         counter += 1;
        //     }
        //
        //     if counter > 2 {
        //         println!("{} Culled", current_copy.position + direction);
        //         neighbors.retain(|&f| f != current_copy.position + direction);
        //     }
        // }
        let directions = [SOUTH, NORTH, EAST, WEST];

        for direction in directions {
            for i in 1..3 {
                let next_node = Point::new(direction.x * 1, direction.y * i) + current_pos;
                if next_node.x >= grid.total_size.x || next_node.x < 0 || next_node.y >= grid.total_size.y || next_node.y < 0 {
                    continue;
                }

                let neighbor_copy;
                {
                    let neighbor = &mut grid.get_node(next_node);
                    neighbor_copy = neighbor.clone();
                }

                // if closed.contains(&neighbor_copy.position) {
                //     continue;
                // }

                current_cost += neighbor_copy.heat_loss;

                if current_cost < *cost_map.get(&neighbor_copy.position).unwrap_or(&i32::MAX) {
                    grid.update_node(current_cost, current_copy.position, neighbor_copy.position);
                    grid.map.get_mut(&neighbor_copy.position).unwrap().cost = current_cost;
                    cost_map.insert(neighbor_copy.position, current_cost);
                    frontier.push(neighbor_copy.position, -current_cost);
                }
            }
        }
    }

    let mut current_goal = grid.get_node(end_point);

    let mut counter = 0;
    let mut positions:Vec<Point> = Vec::new();
    positions.push(current_goal.position);
    while current_goal.prev.is_some() && counter < 200 {
        println!("{}", current_goal.position);
        current_goal = grid.get_node(current_goal.prev.unwrap());
        positions.push(current_goal.position.clone());
        counter = counter + current_goal.heat_loss;
    }


    grid.print_grid(&positions);
    0
}



fn path_find(start_point: Point, end_point: Point, grid: &mut Grid) -> i32 {
    let mut frontier:PriorityQueue<(Point, Point), i32> = PriorityQueue::new();
    let mut closed:Vec<Point> = Vec::new();
    grid.map.get_mut(&start_point).unwrap().cost = 0;
    let start = grid.get_node(start_point);
    frontier.push((start.position, Point::new(0,0)), 0);

    while frontier.len() > 0 {

        let popped = frontier.pop().unwrap();
        println!("{}", -popped.1);
        let current_pos = popped.0.0;
        let current_copy;
        {
            let current = &mut grid.get_node(current_pos);
            current_copy = current.clone();
        }

        if current_pos == end_point {
            println!("DID I FIND IT {}", -popped.1);
        }
        closed.push(current_copy.position);


        //
        // let mut neighbors = grid.get_neighbors(current_copy.position);
        // check previous to see if valid direction
        // If the previous two in the path are the same direction, then remvoe the neighbor thats in that direction.
        // if current_copy.prev.is_some() {
        //     let mut direction_marker = &current_copy.clone();
        //     let mut counter = 0;
        //     let direction = current_copy.position - current_copy.prev.unwrap();
        //     while direction_marker.prev.is_some() && counter <= 4 {
        //         if direction != direction_marker.position - direction_marker.prev.unwrap() {
        //             break;
        //         }
        //         direction_marker = grid.get_node(direction_marker.prev.unwrap());
        //         counter += 1;
        //     }
        //
        //     if counter > 2 {
        //         println!("{} Culled", current_copy.position + direction);
        //         neighbors.retain(|&f| f != current_copy.position + direction);
        //     }
        // }



        let directions = [SOUTH, NORTH, EAST, WEST];

        for direction in directions {
            if popped.0.1 == direction {
                continue;
            }
            let mut current_cost = -popped.1;
            for i in 1..=3 {
                let next_node = Point::new(direction.x * i, direction.y * i) + current_pos;
                if next_node.x >= grid.total_size.x || next_node.x < 0 || next_node.y >= grid.total_size.y || next_node.y < 0 {
                    continue;
                }

                let neighbor_copy;
                {
                    let neighbor = &mut grid.get_node(next_node);
                    neighbor_copy = neighbor.clone();
                }

                current_cost += neighbor_copy.heat_loss;

                // if frontier.get(&neighbor_copy.position).is_some() && current_cost < neighbor_copy.cost {
                //     //println!("{}", neighbor_copy.position);
                //     frontier.remove(&neighbor_copy.position);
                // }

                if closed.contains(&neighbor_copy.position) && current_cost < neighbor_copy.cost {
                    //println!("Closed contains {}", neighbor_copy.position);
                    closed.retain(|&x| x != neighbor_copy.position);
                }

                if current_cost < neighbor_copy.cost {
                    grid.update_node(current_cost, current_pos + Point::new(direction.x * (i-1), direction.y * (i-1)), neighbor_copy.position);
                    grid.map.get_mut(&neighbor_copy.position).unwrap().cost = current_cost;

                    let priority = current_cost;
                    //println!("Adding to Frontier {} {}", neighbor_copy.position, priority);
                    frontier.push((neighbor_copy.position, direction), -priority);
                }
            }
        }

    }

    println!("Finish it up");
    let mut current_goal = grid.get_node(end_point);

    let mut counter = 0;
    let mut positions:Vec<Point> = Vec::new();
    positions.push(current_goal.position);
    while current_goal.prev.is_some() && counter < 200 {
        println!("{}", current_goal.position);
        current_goal = grid.get_node(current_goal.prev.unwrap());
        positions.push(current_goal.position.clone());
        counter = counter + current_goal.heat_loss;
    }


    grid.print_grid(&positions);

    return 0;
}

fn heuristic(a:Point, b:Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}



struct Grid {
    map: HashMap<Point, GridNode>,
    total_size: Point
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, GridNode> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {

                map.insert(Point::new(x as i32, y_index as i32), GridNode { position: Point::new(x as i32, y_index as i32), heat_loss: node.to_digit(10).unwrap() as i32, cost: i32::MAX, prev: Option::None, closed: Cell::new(false) });
            }
            y_index = y_index - 1;

        }

        Grid { map, total_size }
    }

    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let directions = [SOUTH, NORTH, EAST, WEST];
        let mut neighbor_points:Vec<Point> = Vec::new();
        directions.iter().for_each(|dir| {
            if self.map.contains_key(&(*dir + pos)) {
                neighbor_points.push(*dir + pos);
            }
        });

        return neighbor_points;
    }

    fn print_grid(&self, movement: &Vec<Point>) {
        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                if movement.contains(&Point::new(x,y)) {
                    print!("#");
                }
                else {
                     //print!("{}", self.map.get(&Point::new(x, y)).unwrap().heat_loss);
                    print!(".")
                }
            }
            println!("");
        }
    }

    fn get_node(&self, pos: Point) -> &GridNode {
        return self.map.get(&pos).unwrap();
    }

    fn update_node(&mut self, cost:i32, prev: Point, pos: Point) {
        self.map.get_mut(&pos).unwrap().cost = cost;
        self.map.get_mut(&pos).unwrap().prev = Option::Some(prev);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct GridNode {
    position: Point,
    heat_loss: i32,
    cost: i32,
    prev: Option<Point>,
    closed: Cell<bool>
}


#[cfg(test)]
mod tests {
    use crate::days::day_17::part_one_finder;

    #[test]
    fn test_path_find() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

        let result = part_one_finder(input.to_string());

        assert_eq!(result, 102);
    }
}