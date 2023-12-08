use std::collections::HashMap;
use std::fmt::Display;
use itertools::Itertools;
use nom::bytes::complete::{tag, take, take_until};
use nom::IResult;
use num::integer::gcd;
use crate::days::day_08::Direction::{Left, Right};

pub fn part_one(input: String) -> impl Display {
    find_steps_to_end(input)
}

pub fn part_two(input: String) -> impl Display {
    find_steps_to_end_part_2(input)
}

fn find_steps_to_end(input: String) -> usize {
    let mut lines = input.lines();
    let mut instructions = parse_instructions(lines.nth(0).unwrap());
    lines.nth(0).unwrap();

    let mut node_map:HashMap<String, (String, String)> = HashMap::new();
    node_map = lines.map(|line| Node::parse(line).unwrap().1).map(|x| (x.id, (x.left.clone(), x.right.clone()))).collect();

    let instruction_size = instructions.iter().count();
    let mut current_node_key = "AAA";
    let mut number_of_steps = 0usize;
    while current_node_key != "ZZZ" {
        let node = node_map.get(current_node_key);
        let instruction = &instructions[number_of_steps % instruction_size];
        match instruction {
            Left => {
                current_node_key = &*node_map.get(current_node_key).unwrap().0;
            }
            Right => {
                current_node_key = &*node_map.get(current_node_key).unwrap().1;
            }
        }
        number_of_steps += 1;
    }

    number_of_steps
}

fn find_steps_to_end_part_2(input: String) -> usize {
    let mut lines = input.lines();
    let mut instructions = parse_instructions(lines.nth(0).unwrap());
    lines.nth(0).unwrap();

    let mut node_map:HashMap<String, (String, String)> = HashMap::new();
    node_map = lines.map(|line| Node::parse(line).unwrap().1).map(|x| (x.id, (x.left.clone(), x.right.clone()))).collect();

    let instruction_size = instructions.iter().count();
    let mut current_nodes:Vec<&str> = node_map.keys().into_iter().filter(|key| key.ends_with("A")).map(|key| &**key).collect();

    let mut step_collector:HashMap<usize, usize> = HashMap::new();

    let mut number_of_steps = 0usize;
    loop {
        let mut next_set: Vec<&str> = Vec::new();
        for current_node in current_nodes {
            let instruction = &instructions[number_of_steps % instruction_size];
            match instruction {
                Left => {
                    next_set.push(&*node_map.get(current_node).unwrap().0);
                }
                Right => {
                    next_set.push(&*node_map.get(current_node).unwrap().1);
                }
            }
        }

        current_nodes = next_set;
        number_of_steps += 1;

        for i in 0..current_nodes.iter().count() {
            if current_nodes[i].ends_with("Z") {
                if step_collector.get(&i).is_none() {
                    step_collector.insert(i, number_of_steps / instruction_size);
                }
            }
        }

        if step_collector.iter().count() == current_nodes.iter().count() {
            break;
        }

    }

    let mut number_of_iterations:Vec<usize> = Vec::new();
    for (_index, steps) in step_collector {
        number_of_iterations.push(steps);
    }

    let lcm = find_lcm_of_set(number_of_iterations);

    lcm * instruction_size
}


fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn find_lcm_of_set(numbers: Vec<usize>) -> usize {
    numbers.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}


fn parse_instructions(input_line: &str) -> Vec<Direction> {
    input_line.chars().map(|char| {
        match char {
            'L' => Left,
            'R' => Right,
            _ => panic!("ah")
        }
    }).collect()
}

enum Direction {
    Left,
    Right
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Node {
    id: String,
    left: String,
    right: String
}

impl Node {
    fn parse(input_line: &str) -> IResult<&str,Self> {
        let (input_line, id) = take_until(" ")(input_line)?;
        let (input_line, _) = tag(" = (")(input_line)?;
        let (input_line, left) = take(3usize)(input_line)?;
        let (input_line, _) = tag(", ")(input_line)?;
        let (input_line, right) = take(3usize)(input_line)?;

        //println!("{} {} {}", id, left, right);

        Ok((input_line, Node { id: id.to_string(), left: left.to_string(), right: right.to_string()}))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_08::find_steps_to_end_part_2;

    #[test]
    fn can_path_find_with_ghost_logic() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let result = find_steps_to_end_part_2(input.to_string());

        assert_eq!(result, 6);
    }
}