use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use crate::domain::point::{EAST, NORTH, Point, SOUTH, WEST};

pub fn part_one(input: String) -> impl Display {

    process_light_beams_part_1(input)
}

pub fn part_two(input: String) -> impl Display {
    process_light_beams_part_2(input)
}


fn process_light_beams_part_1(input: String) -> usize {
    let grid = Grid::parse(input);
    test_configuration(&grid, LightBeam::new(Point::new(-1, grid.total_size.y - 1), Point::new(1,0)))
}

fn process_light_beams_part_2(input: String) -> usize {
    let grid = Grid::parse(input);
    let mut possible_configurations:Vec<LightBeam> = Vec::new();

    for x in 0..grid.total_size.x {
        possible_configurations.push(LightBeam::new(Point::new(x, grid.total_size.y), SOUTH));
        possible_configurations.push(LightBeam::new(Point::new(x, -1), NORTH));
    }

    for y in 0..grid.total_size.y {
        possible_configurations.push(LightBeam::new(Point::new(-1, y), EAST));
        possible_configurations.push(LightBeam::new(Point::new(grid.total_size.x, y), WEST));
    }
    let mut configuration_results:Vec<usize> = Vec::new();

    for possible_configuration in possible_configurations {
        configuration_results.push(test_configuration(&grid, possible_configuration.clone()));
    }

    *configuration_results.iter().max().unwrap()
}


fn test_configuration(grid: &Grid, initial_beam: LightBeam) -> usize {
    let mut light_beams:Vec<LightBeam> = Vec::new();
    light_beams.push(initial_beam.clone());

    let mut energized_positions:HashSet<LightBeam> = HashSet::new();
    let mut energized_tiles:HashSet<Point> = HashSet::new();

    let mut updated_light_beams:Vec<LightBeam> = light_beams;

    for i in 0..5000 {
        updated_light_beams = tick(&grid, updated_light_beams, &energized_positions);
        for light_beam in &updated_light_beams {
            energized_positions.insert(light_beam.clone());
            energized_tiles.insert(light_beam.position.clone());
        }
        if updated_light_beams.len() == 0 {
            break;
        }
    }

    energized_tiles.len()
}

fn tick(grid: &Grid, light_beams: Vec<LightBeam>, energized: &HashSet<LightBeam>) -> Vec<LightBeam> {
    let mut updated_light_beams:Vec<LightBeam> = Vec::new();

    for light_beam in light_beams {
        let updated_position = light_beam.position + light_beam.velocity;

        if let Some(tile) = grid.nodes.get(&updated_position) {
            match &tile {
                '/' => {
                    if light_beam.velocity == EAST {
                        updated_light_beams.push(LightBeam::new(updated_position, NORTH));
                    }
                    if light_beam.velocity == NORTH {
                        updated_light_beams.push(LightBeam::new(updated_position, EAST));
                    }
                    if light_beam.velocity == WEST {
                        updated_light_beams.push(LightBeam::new(updated_position, SOUTH));
                    }
                    if light_beam.velocity == SOUTH {
                        updated_light_beams.push(LightBeam::new(updated_position, WEST));
                    }
                },
                '\\' => {
                    if light_beam.velocity == WEST {
                        updated_light_beams.push(LightBeam::new(updated_position, NORTH));
                    }
                    if light_beam.velocity == NORTH {
                        updated_light_beams.push(LightBeam::new(updated_position, WEST));
                    }
                    if light_beam.velocity == EAST {
                        updated_light_beams.push(LightBeam::new(updated_position, SOUTH));
                    }
                    if light_beam.velocity == SOUTH {
                        updated_light_beams.push(LightBeam::new(updated_position, EAST));
                    }
                },
                '|' => {
                    if light_beam.velocity == EAST || light_beam.velocity == WEST {
                        updated_light_beams.push(LightBeam::new(updated_position, NORTH));
                        updated_light_beams.push(LightBeam::new(updated_position, SOUTH));
                    }
                    else {
                        updated_light_beams.push(LightBeam::new(updated_position, light_beam.velocity));
                    }
                },
                '-' => {
                    if light_beam.velocity == NORTH || light_beam.velocity == SOUTH {
                        updated_light_beams.push(LightBeam::new(updated_position, EAST));
                        updated_light_beams.push(LightBeam::new(updated_position, WEST));
                    }
                    else {
                        updated_light_beams.push(LightBeam::new(updated_position, light_beam.velocity));
                    }
                }
                _ => {
                    updated_light_beams.push(LightBeam::new(updated_position, light_beam.velocity));
                }
            }
        }
    }

    let mut final_beams:Vec<LightBeam> = Vec::new();
    for updated_light_beam in updated_light_beams {
        if !energized.contains(&updated_light_beam) {
            final_beams.push(updated_light_beam);
        }
    }

    final_beams
}


struct Grid {
    nodes: HashMap<Point, char>,
    total_size: Point
}

impl Grid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, char> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {

                map.insert(Point::new(x as i32, y_index as i32), node);
            }
            y_index = y_index - 1;

        }

        Grid { nodes: map, total_size }
    }

    fn print_grid_with_energy(&self, energized: &HashSet<Point>) {
        for y in (0..self.total_size.y).rev() {
            for x in 0..self.total_size.x {
                if energized.contains(&Point::new(x,y)) {
                    print!("#");
                }
                else {
                    print!("{}", self.nodes.get(&Point::new(x, y)).unwrap());
                }
            }
            println!("");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct LightBeam {
    position: Point,
    velocity: Point
}
impl LightBeam {
    fn new(position:Point, velocity: Point) -> Self {
        LightBeam { position, velocity }
    }
}

