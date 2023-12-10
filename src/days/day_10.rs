use std::collections::HashMap;
use std::fmt::Display;
use nom::character::complete::char;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};

pub fn part_one(input: String) -> impl Display {
    let grid = PipeGrid::parse(input);

    walk_node_path(grid)
}

pub fn part_two(input: String) -> impl Display {
    get_inner_area(PipeGrid::parse(input))
}

fn get_inner_area(pipe_grid: PipeGrid) -> usize {
    let mut inside:Vec<Point> = Vec::new();
    let corners = pipe_grid.get_corners_of_loop();
    let path = pipe_grid.get_walk_path();
    for map_entry in pipe_grid.grid {
        if !path.contains(&map_entry.0) && tile_is_inside(map_entry.0, &corners) {
            inside.push(map_entry.0.clone());
        }
    }

    inside.len()
}

fn tile_is_inside(position: Point, vertices: &Vec<Point>) -> bool {
    let mut is_inside = false;

    let mut j = vertices.len() - 1;

    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[j];

        if p1.y < position.y && p2.y >= position.y || p2.y < position.y && p1.y >= position.y {
            if p1.x + (position.y - p1.y) / (p2.y - p1.y) * (p2.x - p1.x) < position.x
            {
                is_inside = !is_inside;
            }
        }

        j = i;
    }

    return is_inside
}

fn walk_node_path(pipe_grid: PipeGrid) -> usize {
    let start_positions = pipe_grid.get_start_connected_nodes();
    let mut walk_map:Vec<Point> = Vec::new();
    let start_node = pipe_grid.get_start_node();
    walk_map.push(start_node);
    let mut current_node = start_positions[0];
    walk_map.push(current_node);

    while current_node != start_node {
        let pipe = pipe_grid.grid.get(&current_node).unwrap();
        let next_nodes:Vec<Point> = pipe.get_connected_positions().iter().filter(|connected| !walk_map.contains(connected)).map(|c| *c).collect();
        if(next_nodes.len() == 0) {
            break;
        }
        current_node = next_nodes[0];
        walk_map.push(current_node);

    }

    walk_map.len() / 2
}

struct PipeGrid {
    grid: HashMap<Point, Node>,
    size: Point
}

impl PipeGrid {
    fn parse(input: String) -> Self {
        let mut map:HashMap<Point, Node> = HashMap::new();
        let mut y_index = (input.lines().count() as i32) - 1;

        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);
        for (y, line) in input.lines().enumerate() {
            let mut x_index = 0;
            for (x, node) in line.chars().enumerate() {
                map.insert(Point::new(x_index,y_index), Node {pos: Point::new(x_index, y_index), icon: node, start: node == 'S'});
                x_index = x_index + 1;
            }

            y_index = y_index - 1;
        }

        PipeGrid { grid: map, size: total_size }
    }

    fn get_start_node(&self) -> Point {
        for map_entry in &self.grid {
            if (map_entry.1.start == true) {
                return map_entry.1.pos;
            }
        }

        Point::new(0,0)
    }

    fn get_start_connected_nodes(&self) -> Vec<Point> {
        let mut connected_nodes:Vec<Point> = Vec::new();
        for map_entry in &self.grid {
            if(map_entry.1.start == true){
                let start_position:Point = map_entry.1.pos.clone();
                start_position.get_cardinal_neighbors().iter().for_each(|point| {
                    if(!self.grid.contains_key(point)) {
                        return;
                    }
                    let connected = self.grid.get(point).unwrap();
                    if *&connected.get_connected_positions().iter().filter(|p| p.x == start_position.x && p.y == start_position.y).count() > 0 {
                        connected_nodes.push(Point::new(point.x, point.y));
                    }
                });
            }
        }

        connected_nodes
    }

    fn get_walk_path(&self) -> Vec<Point> {
        let start_positions = self.get_start_connected_nodes();
        let mut walk_map:Vec<Point> = Vec::new();
        let start_node = self.get_start_node();
        walk_map.push(start_node);
        let mut current_node = start_positions[0];
        walk_map.push(current_node);

        while current_node != start_node {
            let pipe = self.grid.get(&current_node).unwrap();
            let next_nodes:Vec<Point> = pipe.get_connected_positions().iter().filter(|connected| !walk_map.contains(connected)).map(|c| *c).collect();
            if(next_nodes.len() == 0) {
                break;
            }
            current_node = next_nodes[0];
            walk_map.push(current_node);

        }

        walk_map
    }

    fn get_corners_of_loop(&self) -> Vec<Point> {
        let start_positions = self.get_start_connected_nodes();
        let mut walk_map:Vec<Point> = Vec::new();
        let mut corner_list:Vec<Point> = Vec::new();
        let start_node = self.get_start_node();
        corner_list.push(start_node);
        walk_map.push(start_node);
        let mut current_node = start_positions[1];
        walk_map.push(current_node);

        while current_node != start_node {
            let pipe = self.grid.get(&current_node).unwrap();
            if pipe.is_corner() {
                corner_list.push(current_node);
            }
            let next_nodes:Vec<Point> = pipe.get_connected_positions().iter().filter(|connected| !walk_map.contains(connected)).map(|c| *c).collect();
            if(next_nodes.len() == 0) {
                break;
            }
            current_node = next_nodes[0];
            walk_map.push(current_node);

        }

        corner_list
    }
}

struct Node {
    pos: Point,
    icon: char,
    start: bool
}
impl Node {
    fn get_connected_positions(&self) -> Vec<Point> {
        return match self.icon {
            '|' => vec![self.pos + NORTH, self.pos + SOUTH],
            '-' => vec![self.pos + EAST, self.pos + WEST],
            '7' => vec![self.pos + SOUTH, self.pos + WEST],
            'L' => vec![self.pos + EAST, self.pos + NORTH],
            'J' => vec![self.pos + NORTH, self.pos + WEST],
            'F' => vec![self.pos + SOUTH, self.pos + EAST],
            '.' => vec![],
            _ => vec![],
        }
    }

    fn is_corner(&self) -> bool {
        return match self.icon {
            '|' => false,
            '-' => false,
            _ => true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_10::{get_inner_area, Node, PipeGrid, walk_node_path};
    use crate::domain::point::Point;

    #[test]
    fn can_get_connected_neighbors() {
        let node = Node { pos: Point::new(1, 1), icon: '-', start: false };
        let connected = node.get_connected_positions();

        assert_eq!(connected[0], Point::new(2,1));
        assert_eq!(connected[1], Point::new(0,1 ));
        assert_eq!(connected.len(), 2);
    }

    #[test]
    fn can_get_node_path() {
        let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

        let pipe_grid = PipeGrid::parse(input.to_string());

        assert_eq!(walk_node_path(pipe_grid), 8usize);
    }

    #[test]
    fn get_inner_area_of_grid() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        let pipe_grid = PipeGrid::parse(input.to_string());

        let inner_tiles = get_inner_area(pipe_grid);

        assert_eq!(inner_tiles, 4usize);
    }

    #[test]
    fn get_inner_area_complicated() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

        let pipe_grid = PipeGrid::parse(input.to_string());

        let inner_tiles = get_inner_area(pipe_grid);

        assert_eq!(inner_tiles, 10usize);
    }
}