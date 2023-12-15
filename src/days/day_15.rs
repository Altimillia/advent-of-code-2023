use std::collections::BTreeMap;
use std::fmt::Display;
use crate::tools::parse_numbers;
use indexmap::{indexmap, IndexMap};

pub fn part_one(input: String) -> impl Display {
    let result = input.replace("\n", "").split(",").map(|line| trust_the_process(line)).sum::<u32>();

    result
}

pub fn part_two(input: String) -> impl Display {
    holiday_ascii_string_helper_manual_arrangement_procedure(input)
}

fn holiday_ascii_string_helper_manual_arrangement_procedure(input: String) -> u32 {
    let mut box_map:BTreeMap<u32, IndexMap<String, i32>> = BTreeMap::new();
    input.replace("\n", "").split(",").for_each(|line| process_instruction_line(line, &mut box_map));

    let mut running_total = 0;

    for (key, entry) in box_map {
        println!("{}", key);
        let mut slot_number = 1;
        for (label, focal) in entry {
            println!("{} {}", label, focal);
            running_total += (key + 1) * (slot_number) * focal as u32;
            slot_number += 1;
        }
    }

    running_total
}

fn process_instruction_line(instruction_line: &str, box_map: &mut BTreeMap<u32, IndexMap<String, i32>>) {
    if instruction_line.contains("=") {
        // Add Operation
        let mut split = instruction_line.split("=");
        let label = split.nth(0).unwrap().to_string();
        let focal_len = parse_numbers(split.nth(0).unwrap()).unwrap().1;
        let box_number = trust_the_process(&label);

        if let Some(lens_box) = box_map.get_mut(&box_number) {
            lens_box.insert(label.clone(), focal_len);
        }
        else {
            let mut focal_box:IndexMap<String, i32> = IndexMap::new();
            focal_box.insert(label.clone(), focal_len);
            box_map.insert(box_number, focal_box);
        }
    } else {
        // Remove Operation
        let mut split = instruction_line.split("-");
        let label = split.nth(0).unwrap();
        let box_number = trust_the_process(label);

        if let Some(lens_box) = box_map.get_mut(&box_number) {
            lens_box.shift_remove(label);
        }
    }
}

fn trust_the_process(input:&str) -> u32 {
    let mut current_value = 0;
    input.chars().for_each(|c| {
        let ascii = c as u32;
        current_value += ascii;
        current_value *= 17;
        current_value %= 256;
    });

    //println!("{}", current_value);
    current_value
}

struct Lens {
    label: String,
    focal: i32
}

#[cfg(test)]
mod tests {
    use crate::days::day_15::{holiday_ascii_string_helper_manual_arrangement_procedure, trust_the_process};

    #[test]
    fn can_process_hash() {
        let input = r#"HASH"#;

        assert_eq!(trust_the_process(input), 52);
    }

    #[test]
    fn can_process_initialization() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

        let result = input.split(",").map(|line| trust_the_process(line)).sum::<u32>();

        assert_eq!(result, 1320);
    }

    #[test]
    fn hashmap() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

        let result = holiday_ascii_string_helper_manual_arrangement_procedure(input.to_string());

        assert_eq!(result, 145);
    }
}