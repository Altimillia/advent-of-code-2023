use std::fmt::Display;
use crate::tools::parse_numbers;

pub fn part_one(input: String) -> impl Display {
    let configuration_sequences:Vec<ConfigurationSequence> = input.lines().map(|line| ConfigurationSequence::parse(line)).collect();

    let total = configuration_sequences.iter().map(|seq| {
        return arrangements(seq.condition_records.clone(), seq.contiguous_groups.clone(), 0, seq.condition_records.len() as i32);
    }).sum::<i64>();

    total
}

pub fn part_two(input: String) -> impl Display {
    let configuration_sequences:Vec<ConfigurationSequence> = input.lines().map(|line| ConfigurationSequence::unfold(line)).collect();

    let total = configuration_sequences.iter().map(|seq| {
        return arrangements(seq.condition_records.clone(), seq.contiguous_groups.clone(), 0, seq.condition_records.len() as i32);
    }).sum::<i64>();

    total
}
fn sequence_arrangements(sequence: ConfigurationSequence) -> i64 {
    return arrangements(sequence.condition_records.clone(), sequence.contiguous_groups.clone(), 0, sequence.condition_records.len() as i32);
}
fn arrangements(record: Vec<char>, groups: Vec<i32>, offset:i32, length:i32) -> i64 {
    return match groups.len() {
        0 => no_section(record, offset, length),
        1 => single_section(record, groups[0], offset, length),
        _ => multiple_section(record, groups, offset, length)
    }
}

fn multiple_section(record: Vec<char>, groups: Vec<i32>, offset:i32, length:i32) -> i64 {
    let left:Vec<i32>  = groups.iter().take(groups.len() / 2).map(|x| *x).collect();
    let pivot = groups[groups.len() / 2];
    let right:Vec<i32> = groups.iter().skip(groups.len() / 2 + 1).map(|x| *x).collect();

    let mut  before_pivot = left.iter().sum::<i32>() + left.len() as i32 - 1;
    let slack = length - groups.iter().sum::<i32>() - groups.len() as i32 + 2;
    let mut result = 0;

    for i in 0..slack {
        let after_pivot = before_pivot + 1 + pivot;
        if record[offset as usize + before_pivot as usize] == '#' || (after_pivot < length && record[offset as usize + after_pivot as usize] == '#')
        {
            before_pivot += 1;
            continue;
        }

        let middle = single_section(record.clone(), pivot, offset + before_pivot + 1, pivot);
        if middle == 0 {
            before_pivot += 1;
            continue;
        }

        let left = arrangements(record.clone(), left.clone(), offset, before_pivot);
        if left == 0 {
            before_pivot += 1;
            continue;
        }

        before_pivot += 1;
        let right = arrangements(record.clone(), right.clone(), offset + after_pivot + 1, length - after_pivot - 1);

        result += left * right;
    }

    result
}

fn single_section(record: Vec<char>, section: i32, offset:i32, length:i32) -> i64 {
    let slack = length - section - 1 + 2;
    let mut result = 0;
    for i in 0..slack {
        let mut index = 0;
        let mut possible = true;

        while possible && index < i {
            if record[(offset + index) as usize] == '#' {
                possible = false;
            }
            index += 1;
        }

        while possible && index < i + section {
            if record[(offset + index) as usize] == '.' {
                possible = false;
            }
            index += 1;
        }

        while possible && index < length {
            if record[(offset + index) as usize] == '#' {
                possible = false;
            }
            index += 1;
        }

        if possible {
            result += 1;
        }
    }

    result
}

fn no_section(record: Vec<char>, offset:i32, length:i32) -> i64 {
    for i in 0..length {
        if record[(offset + i) as usize] == '#' {
            return 0;
        }
    }

    return 1
}

struct ConfigurationSequence {
    condition_records: Vec<char>,
    contiguous_groups: Vec<i32>
}

impl ConfigurationSequence {
    fn parse(input_line: &str) -> Self {
        let mut split = input_line.split_whitespace();
        let first_half = split.nth(0).unwrap();
        let second_half = split.nth(0).unwrap();

        let characters:Vec<char> = first_half.chars().collect();
        let contiguous_groups:Vec<i32> = second_half.split(',').map(|section| parse_numbers(section).unwrap().1).collect();

        ConfigurationSequence { condition_records: characters, contiguous_groups }
    }

    fn unfold(input_line:&str) -> Self {
        let mut split = input_line.split_whitespace();
        let first_half = split.nth(0).unwrap();
        let second_half = split.nth(0).unwrap();

        let mut unfolded_records:Vec<&str> = Vec::new();
        let mut unfolded_groups:Vec<&str> = Vec::new();
        for i in 0..5 {
            unfolded_records.push(first_half);
            unfolded_groups.push(second_half);
        }

        let unfolded:Vec<char> = unfolded_records.join("?").chars().collect();
        let groups:Vec<i32> = unfolded_groups
            .join(",")
            .split(',')
            .map(|section| parse_numbers(section).unwrap().1)
            .collect();

        ConfigurationSequence { condition_records: unfolded, contiguous_groups: groups }
    }

}

#[cfg(test)]
mod tests {
    use crate::days::day_12::{ConfigurationSequence, sequence_arrangements};

    #[test]
    fn can_get_configuration_amount() {
        let input_line = r#"?###???????? 3,2,1"#;

        let sequence = ConfigurationSequence::parse(input_line);
        let amount = sequence_arrangements(sequence);

        assert_eq!(amount, 10);

    }

    #[test]
    fn can_unfold_record() {
        let input_line = r#"???.### 1,1,3"#;

        let sequence = ConfigurationSequence::unfold(input_line);
        let s:String = sequence.condition_records.iter().collect();
        assert_eq!(s, "???.###????.###????.###????.###????.###");
    }

    #[test]
    fn can_calculate_unfolded_configuration_amount() {
        let input_line = r#"?###???????? 3,2,1"#;

        let sequence = ConfigurationSequence::unfold(input_line);
        let amount = sequence_arrangements(sequence);

        assert_eq!(amount, 506250);
    }
}