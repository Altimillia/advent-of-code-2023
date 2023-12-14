use std::fmt::Display;

pub fn part_one(input: String) -> impl Display {
    summarize(input, false)
}

fn summarize(input:String, smudging:bool) -> i32 {
    let patterns:Vec<MirrorPattern> = input.split_terminator("\n\n")
        .map(|block| MirrorPattern::parse(block)).collect();


    let mut running_total = 0;
    patterns.iter().for_each(|pattern| {
        running_total += pattern.get_summary_total(smudging);
    });

    running_total
}

pub fn part_two(input: String) -> impl Display {
    summarize(input, true)
}

struct MirrorPattern {
    vertical: Vec<Vec<char>>,
    horizontal: Vec<Vec<char>>
}

impl MirrorPattern {
    fn parse(input_block: &str) -> Self {
        let horizontal: Vec<Vec<char>> = input_block.lines().map(|line| line.chars().collect()).collect();
        let line_len = input_block.lines().peekable().nth(0).unwrap().len();

        let mut vertical: Vec<Vec<char>> = Vec::new();
        let lines_count = input_block.lines().count();
        for i in 0..line_len {
            let mut vert_slice:Vec<char> = Vec::new();
            for j in 0..lines_count {
                vert_slice.push(input_block
                    .lines()
                    .peekable()
                    .nth(j)
                    .unwrap()
                    .chars()
                    .peekable()
                    .nth(i)
                    .unwrap());
            }

            vertical.push(vert_slice);
        }


        MirrorPattern { horizontal, vertical }
    }

    fn get_summary_total(&self, smudging:bool) -> i32 {
        let mut running_total = 0;
        match self.find_vertical_reflect_point(smudging) {
            None => {}
            Some(value) => {
                running_total += value as i32
            }
        }

        match self.find_horizontal_reflect_point(smudging) {
            None => {}
            Some(value) => {
                running_total += (100 * value as i32)
            }
        }

        running_total
    }

    fn find_vertical_reflect_point(&self, smudging: bool) -> Option<usize> {
        // go through each point, and then match up the positions outward until they do not match or an end is reached

        let slices = &self.vertical.len();
        for i in 1..self.vertical.len()  {
            if MirrorPattern::check_position(self.vertical.clone(), i as i32, *slices, smudging) {
                return Some(i);
            }
        }

        None
    }

    fn find_horizontal_reflect_point(&self, smudging: bool) -> Option<usize> {
        let slices = &self.horizontal.len();
        for i in 1..self.horizontal.len() {
            if MirrorPattern::check_position(self.horizontal.clone(), i as i32, *slices, smudging) {
                return Some(i);
            }
        }

        None
    }

    fn check_position(slices: Vec<Vec<char>>, position: i32, number_of_slices: usize, smudge: bool) -> bool {
        let mut left_index = position as i32 - 1;
        let mut right_index = position;
        let mut is_mirror = left_index >= 0 && right_index <= number_of_slices as i32;
        let mut smudged = !smudge;

        // println!("Start Check");
        while left_index >= 0 && right_index < number_of_slices as i32 && is_mirror {
            let left:String = slices.get(left_index as usize).unwrap().into_iter().collect();
            let right:String = slices.get(right_index as usize).unwrap().into_iter().collect();

            if left == right
            {
                is_mirror = true;
            }
            else {
                if !smudged {
                    let unequal_positions = MirrorPattern::find_unequal_chars_positions(&left, &right);
                    if unequal_positions.len() == 1 {
                        smudged = true;
                        is_mirror = true;
                    }
                    else {
                        is_mirror = false;
                    }
                }
                else {
                    is_mirror = false;
                }
            }
            // println!("{} {} {}", left, right, is_mirror);

            left_index -= 1;
            right_index += 1;
        }

        is_mirror && smudged
    }

    fn find_unequal_chars_positions(str1: &str, str2: &str) -> Vec<(usize, char, char)> {
        let mut unequal_positions = Vec::new();

        for (pos, (char1, char2)) in str1.chars().zip(str2.chars()).enumerate() {
            if char1 != char2 {
                unequal_positions.push((pos, char1, char2));
            }
        }

        unequal_positions
    }

}

#[cfg(test)]
mod tests {
    use crate::days::day_13::{MirrorPattern, summarize};

    #[test]
    fn find_vertical_reflect_pattern() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

        let mirror_pattern = MirrorPattern::parse(input);

        assert_eq!(mirror_pattern.find_vertical_reflect_point(false).unwrap(), 5);
    }

    #[test]
    fn find_horizontal_reflect_pattern() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let mirror_pattern = MirrorPattern::parse(input);

        assert_eq!(mirror_pattern.find_horizontal_reflect_point(false).unwrap(), 4);
    }

    #[test]
    fn summarize_notes(){
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let summary = summarize(input.to_string(), false);

        assert_eq!(summary, 405);
    }

    #[test]
    fn summary_pattern() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let mirror_pattern = MirrorPattern::parse(input);

        assert_eq!(mirror_pattern.get_summary_total(false), 400);
    }

    #[test]
    fn horizontal_position_with_smudging() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

        let mirror_pattern = MirrorPattern::parse(input);

        assert_eq!(mirror_pattern.find_horizontal_reflect_point(true).unwrap(), 3);
    }

    #[test]
    fn more_horizontal_position_with_smudging() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let mirror_pattern = MirrorPattern::parse(input);

        assert_eq!(mirror_pattern.find_horizontal_reflect_point(true).unwrap(), 1);
    }

    #[test]
    fn full_summary_with_smudging() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let summary = summarize(input.to_string(), true);

        assert_eq!(summary, 400);
    }
}