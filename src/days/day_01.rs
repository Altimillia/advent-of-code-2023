use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use itertools;
use itertools::Itertools;
use crate::tools::parse_numbers;


pub fn part_one(input: String) -> impl Display {
    let instructions:Vec<CalibrationInstruction> = input
        .lines()
        .into_iter()
        .map(|f| CalibrationInstruction::new(f))
        .collect();

    let sum = instructions
        .iter()
        .map(|instruction| instruction.value)
        .sum::<i32>();

    sum
}

pub fn part_two(input: String) -> impl Display {
    let instructions:Vec<CalibrationInstruction> = input
        .lines()
        .into_iter()
        .map(|f| CalibrationInstruction::parse_v2(f))
        .collect();

    let sum = instructions
        .iter()
        .map(|instruction| instruction.value)
        .sum::<i32>();

    sum
}


struct CalibrationInstruction {
    value: i32
}

impl CalibrationInstruction {
    pub fn new(instruction_line: &str) -> Self {
        let instruction_values:Vec<char> = instruction_line
            .chars()
            .into_iter()
            .filter(|character| character.is_numeric())
            .collect();

        let first_value = instruction_values.as_slice()[0].to_string();
        let last_value = instruction_values.as_slice().last().unwrap().to_string();
        let total_value_string = format!("{first_value}{last_value}");

        return CalibrationInstruction { value: parse_numbers(&total_value_string).unwrap().1 }
    }

    pub fn parse_v2(instruction_line: &str) -> Self {

        let mut string_to_number = HashMap::new();

        string_to_number.insert("one", 1);
        string_to_number.insert("two", 2);
        string_to_number.insert("three", 3);
        string_to_number.insert("four", 4);
        string_to_number.insert("five", 5);
        string_to_number.insert("six", 6);
        string_to_number.insert("seven", 7);
        string_to_number.insert("eight", 8);
        string_to_number.insert("nine", 9);

        let mut number_positions= BTreeMap::new();
        for i in 0..10 {
            let i_str = i.to_string();
            let indices = instruction_line.match_indices(&i_str);
            indices.for_each(|(index, _)| {
                let _ = number_positions.insert(index, i);
            });
        }

        for kvp in string_to_number {
            let indices = instruction_line.match_indices(&kvp.0);
            indices.for_each(|(index, _)| {
                let _ = number_positions.insert(index, kvp.1);
            });
        }

        let sorted: Vec<&i32> = number_positions.iter()
            .sorted_by(|(a_key, _), (b_key, _)| Ord::cmp(a_key, b_key))
            .map(|(_, value)| value).collect();

        let total_value_string;
        let first_value = sorted.first().unwrap().to_string();
        let last_value = sorted.last().unwrap().to_string();

        total_value_string = format!("{first_value}{last_value}");

        return CalibrationInstruction { value: parse_numbers(&total_value_string).unwrap().1 }
    }
}
