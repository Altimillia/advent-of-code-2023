use std::fmt::Display;
use num::traits::real::Real;
use crate::tools::{parse_numbers_i64, usize_to_i64};

pub fn part_one(input: String) -> impl Display {
    parse_number_of_winning_races(input)
}

pub fn part_two(input: String) -> impl Display {
    parse_mega_race_winning(input)
}

fn parse_number_of_winning_races(input: String) -> i64 {
    let races = parse_races(input);
    determine_race_winning_margin_of_error(races)
}

fn parse_mega_race_winning(input: String) -> i64 {
    let races = vec![parse_mega_race(&input)];
    determine_race_winning_margin_of_error(races)
}


fn determine_race_winning_margin_of_error(races: Vec<Race>) -> i64 {
    let winning_races:Vec<i64> = races.iter().map(|race| {
        let winning_races = race
            .race_configurations
            .iter()
            .filter(|configuration|
                {
                    configuration.distance_achieved > race.record_distance
                }
            )
            .count();
        return usize_to_i64(winning_races).unwrap()
    }).collect();

    winning_races.iter().fold(1, |acc, num| {
        println!("{}", num);
        acc * num
    })
}
fn parse_races(input: String) -> Vec<Race> {
    let mut lines = input.lines();
    let times:Vec<i64> = parse_values("Time:", lines.nth(0).unwrap());
    let distance:Vec<i64> = parse_values("Distance:", lines.nth(0).unwrap());
    let mut races:Vec<Race> = Vec::new();

    for i in 0..times.iter().count() {
        races.push(Race::new(times[i], distance[i]));
    }

    races
}

fn parse_mega_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time_str :String = lines.nth(0)
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .collect();
    let time = parse_numbers_i64(&time_str).unwrap().1;
    let distance_str :String = lines.nth(0)
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .collect();
    let distance = parse_numbers_i64(&distance_str).unwrap().1;

    Race::new(time, distance)
}


fn parse_values(value_type: &str, input_line: &str) -> Vec<i64> {
    let input_line = input_line.replace(value_type, "");
    input_line.split_whitespace().map(|time_entry| parse_numbers_i64(time_entry).unwrap().1).collect()
}
struct Race {
    time: i64,
    record_distance: i64,
    race_configurations: Vec<RaceConfiguration>
}

impl Race {
    fn new(time:i64, record_distance: i64) -> Self {
        let min_velocity = (record_distance as f32 / time as f32).ceil() as i64;

        let mut race_configurations:Vec<RaceConfiguration> = Vec::new();
        for i in min_velocity..time {
            race_configurations.push(RaceConfiguration::new(i, time))
        }

        Race { time,race_configurations, record_distance}
    }
}

struct RaceConfiguration {
    hold_button_time: i64,
    distance_achieved: i64
}

impl RaceConfiguration {
    fn new(hold_button_time: i64, max_allowed_time: i64) -> Self {
        let travel_time = max_allowed_time - hold_button_time;

        return RaceConfiguration { hold_button_time, distance_achieved: travel_time * hold_button_time }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_06::{parse_mega_race_winning, parse_number_of_winning_races};

    #[test]
    fn can_parse_race() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let result = parse_number_of_winning_races(input.to_string());

        assert_eq!(result, 288);
    }

    #[test]
    fn can_parse_mega_race() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let result = parse_mega_race_winning(input.to_string());

        assert_eq!(result, 71503);
    }
}
