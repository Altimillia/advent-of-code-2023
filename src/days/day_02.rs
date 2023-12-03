use std::cmp;
use std::fmt::Display;
use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use crate::tools::parse_numbers;

pub fn part_one(input: String) -> impl Display {
    let sum = input
        .lines()
        .into_iter()
        .map(|f| Game::new(f).unwrap().1)
        .filter(|game| game.is_possible(14,12, 13))
        .map(|valid_game| valid_game.id)
        .sum::<i32>();
    sum
}

pub fn part_two(input: String) -> impl Display {
    let sum = input
        .lines()
        .into_iter()
        .map(|f| Game::new(f).unwrap().1.get_cube_power())
        .sum::<i32>();
    sum
}
struct Game {
    id: i32,
    sets: Vec<GameSet>
}

impl Game {
    pub fn new(input_line: &str) -> IResult<&str, Self> {
        let (input_line, _) = tag("Game ")(input_line)?;
        let (input_line, id) = take_until(":")(input_line)?;
        let (input_line, _) = tag(": ")(input_line)?;

        let game_sets:Vec<GameSet> = input_line.split(";").map(|split| GameSet::new(split)).collect();

        return Ok((input_line, Game { id: parse_numbers(id).unwrap().1, sets: game_sets }));
    }

    pub fn is_possible(&self, blue_count: i32, red_count: i32, green_count: i32) -> bool {
        for set in &self.sets {
            if set.blue > blue_count || set.green > green_count || set.red > red_count {
                return false;
            }
        }

        return true;
    }

    pub fn get_cube_power(&self) -> i32 {
        let mut min_blue = 0;
        let mut min_red = 0;
        let mut min_green = 0;

        for set in &self.sets {
            min_blue = cmp::max(set.blue, min_blue);
            min_red = cmp::max(set.red, min_red);
            min_green = cmp::max(set.green, min_green);
        }

        min_blue * min_red * min_green
    }
}

#[derive(Debug)]
struct GameSet {
    blue: i32,
    red: i32,
    green: i32
}

impl GameSet {
    fn new(input_line: &str) -> Self {
        let cube_counts:Vec<&str> = input_line.split(",").collect();
        let mut green_count = 0;
        let mut red_count = 0;
        let mut blue_count = 0;
        for cube_count in cube_counts {
            if cube_count.contains("green") {
                green_count = parse_numbers(cube_count.trim()).unwrap().1;
            }
            if cube_count.contains("blue") {
                blue_count = parse_numbers(cube_count.trim()).unwrap().1;
            }
            if cube_count.contains("red") {
                red_count = parse_numbers(cube_count.trim()).unwrap().1;
            }
        }

        return GameSet{
            blue: blue_count,
            red: red_count,
            green: green_count,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::{Game, GameSet};

    #[test]
    fn game_new_can_parse_counts() -> Result<(), String> {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"#;
        let game = Game::new(input);

        assert_eq!(game.unwrap().1.sets.iter().count(), 3);
        Ok(())
    }

    #[test]
    fn game_set_new_can_parse_set_counts() {
        let input = r#"3 blue, 4 red"#;
        let game_set = GameSet::new(input);

        assert_eq!(game_set.blue, 3);
        assert_eq!(game_set.red, 4);
        assert_eq!(game_set.green, 0);
    }

    #[test]
    fn game_validates_if_is_possible() -> Result<(), String> {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"#;
        let game = Game::new(input).unwrap().1;

        let possible = game.is_possible(14,12, 13);
        assert_eq!(possible, true);
        Ok(())
    }

    #[test]
    fn game_validates_if_impossible() -> Result<(), String> {
        let input = r#"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"#;
        let game = Game::new(input).unwrap().1;

        let possible = game.is_possible(14,12, 13);
        assert_eq!(possible, false);
        Ok(())
    }

    #[test]
    fn game_returns_power() -> Result<(), String> {
        let input = r#"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"#;
        let game = Game::new(input).unwrap().1;

        let power = game.get_cube_power();
        assert_eq!(power, 1560);
        Ok(())
    }
}