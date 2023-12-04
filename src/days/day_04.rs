use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use itertools::Itertools;
use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use crate::domain::point::Point;
use crate::tools::{parse_numbers, usize_to_i32, usize_to_u32};

pub fn part_one(input: String) -> impl Display {
    input.lines().map(|line|
        Card::parse(line).unwrap().1
    ).map(|card| card.score_card()).sum::<i32>()
}

pub fn part_two(input: String) -> impl Display {
    card_counter(input)
}

fn card_counter(input: String) -> i32 {
    let mut original_card_map:HashMap<i32, Card> = HashMap::new();
    let mut card_instances:HashMap<i32, Vec<Card>> = HashMap::new();
    input.lines().for_each(|line|
        {
            let card = Card::parse(line).unwrap().1;
            original_card_map.insert(card.id.clone(), card.clone());
            card_instances.insert(card.id.clone(), vec![card.clone()]);

        });

    let id_count = card_instances.keys().len() as i32;
    for i in 1..id_count + 1 {
        let cards = card_instances.get(&i).unwrap().clone();
        for card in cards {
            for j in 1..card.get_number_of_winners() + 1 {
                let index = i+j;
                if(card_instances.contains_key(&index)) {
                    card_instances.get_mut(&index).unwrap().push(original_card_map.get(&index).unwrap().clone())
                }
            }
        }
    }

    let mut total_count:i32 = 0;
    for i in 1..id_count + 1 {
        let count = card_instances.get(&i).unwrap().iter().count() as i32;
        total_count += count;
    }

    total_count
}

#[derive(Clone)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers_i_have: Vec<i32>,
    id: i32
}
impl Card {

    fn parse(input_line: &str) -> IResult<&str, Self> {
        let (input_line, _) = tag("Card ")(input_line)?;
        let (input_line, id) = take_until(": ")(input_line)?;
        let (input_line, _) = tag(":")(input_line)?;
        let (input_line, winning_numbers) = take_until("| ")(input_line)?;
        let (numbers_i_have, _) = tag("|")(input_line)?;

        let my_numbers = numbers_i_have.split_whitespace().map(|item| parse_numbers(item).unwrap().1).collect();
        let winning = winning_numbers.split_whitespace().map(|item| parse_numbers(item).unwrap().1).collect();

        Ok((input_line, Card { numbers_i_have: my_numbers, winning_numbers: winning, id: parse_numbers(id.trim()).unwrap().1 }))
    }

    fn score_card(&self) -> i32 {
        let matched_numbers_count = self
            .numbers_i_have
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .map(|number| number)
            .count();

        if(matched_numbers_count == 0){
            return 0;
        }
        let base:i32 = 2;

        base.pow(usize_to_u32(matched_numbers_count).unwrap() - 1)
    }

    fn get_number_of_winners(&self) -> i32 {
        let matched_numbers_count = self
            .numbers_i_have
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .map(|number| number)
            .count();

        usize_to_i32(matched_numbers_count).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::Card;
    use super::card_counter;

    #[test]
    fn card_can_be_parsed() -> Result<(), String> {
        let input_line = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;

        let card = Card::parse(input_line).unwrap().1;

        assert_eq!(card.winning_numbers.iter().count(), 5);
        Ok(())
    }

    #[test]
    fn card_can_be_scored() -> Result<(), String> {
        let input_line = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;

        let card = Card::parse(input_line).unwrap().1;

        assert_eq!(card.score_card(), 8);
        Ok(())
    }

    #[test]
    fn non_winning_card_score_is_zero() -> Result<(), String> {
        let input_line = r#"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"#;

        let card = Card::parse(input_line).unwrap().1;

        assert_eq!(card.score_card(), 0);
        Ok(())
    }

    #[test]
    fn card_with_one_winner_returns_one() -> Result<(), String> {
        let input_line = r#"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"#;

        let card = Card::parse(input_line).unwrap().1;

        assert_eq!(card.score_card(), 1);
        Ok(())
    }

    #[test]
    fn cards_can_be_cloned() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let result = card_counter(input.to_string());

        assert_eq!(result, 30);
    }
}
