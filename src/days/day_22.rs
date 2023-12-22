#![feature(iter_array_chunks)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::ops::Sub;
use itertools::chain;
use rustc_hash::FxHashSet;
use crate::domain::vector3::Vector3;

pub fn part_one(input: String) -> impl Display {

    process_falling_bricks(World { grid: HashMap::new(), bricks: Vec::new()}, get_bricks(input))
}

fn get_bricks(input: String) -> Vec<Brick> {
    let mut counter_id = 0;
    input.lines().map(|line|
        {
            counter_id += 1;
            Brick::parse(line, counter_id)
        }).collect()
}

fn process_falling_bricks(mut world: World, mut bricks: Vec<Brick>) -> i32 {
    bricks.sort_by(|a,b| a.position.z.cmp(&b.position.z));

    let mut falling_bricks = bricks.clone();
    falling_bricks.iter_mut().for_each(|brick| {
        world.fall_until_stop(brick);
    });

    // world.grid.iter().for_each(|(key, brick)| println!("{}", brick));
    // world.bricks.iter().for_each(|brick| println!("{}", brick));

    let valid_targets =  world.get_valid_disintegration_targets();
    for brick in &valid_targets {
        println!("{}", brick);
    }
    valid_targets.len() as i32
}

pub fn part_two(input: String) -> impl Display {
    process_bricks_and_chain_reaction(World { grid: HashMap::new(), bricks: Vec::new()}, get_bricks(input))
}

fn process_bricks_and_chain_reaction(mut world: World, mut bricks: Vec<Brick>) -> i32 {
    bricks.sort_by(|a,b| a.position.z.cmp(&b.position.z));

    let mut falling_bricks = bricks.clone();
    falling_bricks.iter_mut().for_each(|brick| {
        world.fall_until_stop(brick);
    });

    let mut total_chains = 0;
    for brick in &world.bricks {
        // let chain_reaction_count = get_chain_reaction_recursive(&world, *brick, &HashSet::new()).len() - 1;
        let chain_reaction_count = chain_reaction_try_2(&world, brick);
        println!("Brick {} - chains {}", brick.id, chain_reaction_count);
        total_chains += chain_reaction_count;
    }

    total_chains as i32
}

fn chain_reaction_try_2(world: &World, brick: &Brick) -> i32 {
    // Clone a mutuable copy of world
    // Get the bricks supported by this brick
    // Remove brick
    // Process all supported bricks s_b
    //   Check original world for which s_b brick supports
    //   Push back to queue
    //   try to move each s_b down
    // Continue until no other bricks are allowed to move
    // return back the total brick movement count

    let mut chain_world = world.clone();
    let mut brick_queue:VecDeque<Brick> = VecDeque::new();
    let mut counter = 0;

    let supported_bricks = chain_world.get_supported_bricks(*brick);
    for supported_brick in supported_bricks {
        brick_queue.push_back(supported_brick);
    }

    chain_world.remove_brick(*brick);


    while brick_queue.len() > 0 {

        let next_brick = brick_queue.pop_front().unwrap();

        let next_check_bricks = world.get_supported_bricks(next_brick);
        let result = chain_world.brick_fall(next_brick);
        for next_check_brick in next_check_bricks {
            brick_queue.push_back(next_check_brick);
        }
        if result {
            counter += 1;
        }
    }

    counter
}


fn get_chain_reaction_recursive(world: &World, brick: Brick, falling_bricks: &HashSet<Brick>) -> Vec<Brick> {
    let mut bricks_to_fall:HashSet<Brick> = falling_bricks.clone();
    bricks_to_fall.insert(brick);
    let supported_bricks = world.get_supported_bricks(brick);
    let mut next_bricks:Vec<Brick> = Vec::new();

    // Get the next set of bricks this brick disappearing would get.
    // Then chain up the line one step with ALL of those bricks

    for supported_brick in supported_bricks {
        let brick_supports:Vec<Brick> = world.get_number_of_supporting_bricks(supported_brick);

        let number_of_supports = brick_supports
            .iter()
            .filter(|b| !bricks_to_fall.contains(b))
            .count();

        if number_of_supports > 0 {
            //println!("Supported by {}", number_of_supports);
            continue;
        }

        next_bricks.push(supported_brick);
        bricks_to_fall.insert(supported_brick);
    }

    for next_brick in next_bricks {
        let unique_bricks = get_chain_reaction_recursive(&world, next_brick, &bricks_to_fall);
        for unique_brick in unique_bricks {
            bricks_to_fall.insert(unique_brick);
        }
    }


    bricks_to_fall.iter().map(|b| *b).collect()
}

fn check_overlap(brick_1: &Brick, brick_2: &Brick) -> bool {
    let (b1_min, b1_max) = brick_1.get_bounds();
    let (b2_min, b2_max) = brick_2.get_bounds();

    let overlap_x = b1_min.x <= b2_max.x && b1_max.x >= b2_min.x;
    let overlap_y = b1_min.y <= b2_max.y && b1_max.y >= b2_min.y;
    let overlap_z = b1_min.z <= b2_max.z && b1_max.z >= b2_min.z;

    return overlap_x && overlap_y && overlap_z;
}
#[derive(Debug, Clone)]
struct World {
    grid: HashMap<Vector3, Brick>,
    bricks: Vec<Brick>
}

impl World {
    fn valid_placement(&self, brick: &Brick) -> bool {
        if brick.position.z <= 0 {
            return false;
        }
        let positions = brick.get_all_positions();
        for position in positions {
            if self.grid.contains_key(&position)
            {
                return false;
            }
        }

        return true;
    }

    fn fall_until_stop(&mut self, brick: &Brick) -> bool {
        let mut current_brick = brick.clone();
        let mut falling_brick = current_brick.clone();
        if !self.valid_placement(&current_brick) {
            panic!("This should be valid to place: {}", current_brick);
        }
        loop {
            falling_brick = current_brick.move_brick(Vector3::new(0,0,-1));
            if !self.valid_placement(&falling_brick) {
                self.add_brick(current_brick);
                break;
            }
            else {
                current_brick = falling_brick;
            }
        }

        return current_brick.position != brick.position
    }

    fn brick_fall(&mut self, brick: Brick) -> bool {
        self.remove_brick(brick);
        self.fall_until_stop(&brick)
    }

    fn remove_brick(&mut self, brick: Brick) {
        for position in brick.get_all_positions() {
            self.grid.remove(&position);
        }
        self.bricks.retain(|f| f.id != brick.id);
    }

    fn add_brick(&mut self, brick: Brick) {
        for position in brick.get_all_positions() {
            self.grid.insert(position, brick);
        }
        self.bricks.push(brick);
    }

    fn get_valid_disintegration_targets(&self) -> Vec<Brick> {
        let mut targets:Vec<Brick> = Vec::new();
        for brick in &self.bricks {
            let mut unique_bricks:FxHashSet<Brick> = FxHashSet::default();
            let brick_positions = brick.get_all_positions();
            for brick_position in brick_positions {
                if let Some(other_brick) = &self.grid.get(&(brick_position + Vector3::new(0,0,1))) {
                    if other_brick.id != brick.id {
                        unique_bricks.insert(**other_brick);
                    }
                }
            }

            let mut can_disintegrate = true;
            // Now check each of those bricks, if all of them have 2+ supports we are good to disintegrate
            for unique_brick in unique_bricks {
                if self.get_number_of_supporting_bricks(unique_brick).len() == 1 {
                    can_disintegrate = false;
                }
            }

            if can_disintegrate {
                targets.push(brick.clone());
            }
        }

        targets
    }

    fn get_supported_bricks(&self, brick: Brick) -> Vec<Brick> {
        let mut unique_bricks:FxHashSet<Brick> = FxHashSet::default();
        let brick_positions = brick.get_all_positions();
        for brick_position in brick_positions {
            if let Some(other_brick) = &self.grid.get(&(brick_position + Vector3::new(0,0,1))) {
                if other_brick.id != brick.id {
                    unique_bricks.insert(**other_brick);
                }
            }
        }

        unique_bricks.iter().map(|b| *b).collect()
    }

    fn get_number_of_supporting_bricks(&self, brick: Brick) -> Vec<Brick> {
        let mut unique_bricks:FxHashSet<Brick> = FxHashSet::default();
        let brick_positions = brick.get_all_positions();

        for brick_position in brick_positions {
            if let Some(other_brick) = &self.grid.get(&(brick_position + Vector3::new(0,0,-1))) {
                if other_brick.id != brick.id {
                    unique_bricks.insert(**other_brick);
                }
            }
        }

        unique_bricks.iter().map(|b| *b).collect()
    }


}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Brick {
    position: Vector3,
    size: Vector3,
    id: i32
}

impl Brick {
    fn parse(input_line:&str, id: i32) -> Self {
        let mut split = input_line.split("~");
        let [position1, position2] = split.map(|s| Vector3::parse(s)).next_chunk().unwrap();

        Brick { position: position1, size: position2 - position1, id }
    }

    fn get_bounds(&self) -> (Vector3, Vector3) {
        return (self.position, self.position + self.size)
    }

    fn move_brick(&self, velocity: Vector3) -> Self {
        return Brick { position: self.position + velocity, size: self.size, id: self.id }
    }

    fn get_all_positions(&self) -> Vec<Vector3> {
        let mut positions:Vec<Vector3> = Vec::new();
        for x in 0..=self.size.x {
            for y in 0..=self.size.y {
                for z in 0..=self.size.z {
                    positions.push(self.position + Vector3::new(x,y,z));
                }
            }
        }

        positions
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {} Position: {} Size: {}", self.id, self.position, self.size)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day_22::{get_bricks, process_bricks_and_chain_reaction, process_falling_bricks, World};

    #[test]
    fn can_get_disintegration_targets() {
        let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

        let result = process_falling_bricks(World { grid: HashMap::new(), bricks: Vec::new()}, get_bricks(input.to_string()));

        assert_eq!(result, 5);
    }

    #[test]
    fn can_get_chain_reaction_amount() {
        let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

        let result = process_bricks_and_chain_reaction(World { grid: HashMap::new(), bricks: Vec::new()}, get_bricks(input.to_string()));

        assert_eq!(result, 7);
    }
}