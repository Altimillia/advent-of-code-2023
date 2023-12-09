use std::fmt::Display;
use itertools::Itertools;
use nom::sequence::pair;
use crate::tools::parse_numbers_i64;

pub fn part_one(input: String) -> impl Display {
    input.lines().map(|line| History::parse(line))
        .map(|history| {
            return history.process_until_end()
                .predict_sequences_next_value()
                .get_prediction_value()
        }).sum::<i64>()
}

pub fn part_two(input: String) -> impl Display {
    input.lines().map(|line| History::parse(line))
        .map(|history| {
            return history.process_until_end()
                .predict_sequences_previous_value()
                .get_previous_predict_value()
        }).sum::<i64>()
}

struct History {
    sequences: Vec<Sequence>
}

impl History {
    fn parse(input_line: &str) -> Self {
        History { sequences: vec![Sequence::parse(input_line)]}
    }

    fn has_end_sequence(&self) -> bool {
        self.sequences.last().unwrap().is_end()
    }

    fn process_until_end(self) -> History {
        let next_sequence = self.sequences.last().unwrap().clone();
        let mut updated_sequences:Vec<Sequence> = Vec::new();
        updated_sequences.push(next_sequence);

        let mut is_end:bool = updated_sequences.last().unwrap().is_end();
        while !is_end {
            updated_sequences.push(updated_sequences.last().unwrap().next_sequence());
            is_end = updated_sequences.last().unwrap().is_end();
        }

        History { sequences: updated_sequences }
    }

    fn predict_sequences_next_value(self) -> History {
        let amount_of_sequences = self.sequences.len();

        let mut updated_sequences:Vec<Sequence> = Vec::new();

        for i in (0..self.sequences.len()).rev() {
            let mut previous_diff:i64 = 0;
            if i != amount_of_sequences - 1 {
                previous_diff = *updated_sequences.last().unwrap().values.last().unwrap();
            }
            let new_sequence = self.sequences[i].predict_next_value(previous_diff).clone();
            updated_sequences.push(new_sequence);
        }

        updated_sequences.reverse();

        // for updated_sequence in &updated_sequences {
        //     print!("Next Sequence: ");
        //     for value in &updated_sequence.values {
        //         print!("{} ", value);
        //     }
        //     println!(" ");
        //
        // }

        History { sequences: updated_sequences }
    }

    fn predict_sequences_previous_value(self) -> History {
        let amount_of_sequences = self.sequences.len();

        let mut updated_sequences:Vec<Sequence> = Vec::new();

        for i in (0..self.sequences.len()).rev() {
            let mut previous_diff:i64 = 0;
            if i != amount_of_sequences - 1 {
                previous_diff = *updated_sequences.last().unwrap().values.first().unwrap();
            }
            let new_sequence = self.sequences[i].predict_previous_value(previous_diff).clone();
            updated_sequences.push(new_sequence);
        }

        updated_sequences.reverse();

        // for updated_sequence in &updated_sequences {
        //     print!("Next Sequence: ");
        //     for value in &updated_sequence.values {
        //         print!("{} ", value);
        //     }
        //     println!(" ");
        //
        // }

        History { sequences: updated_sequences }
    }

    fn get_prediction_value(self) -> i64 {
        *self.sequences[0].values.last().unwrap()
    }

    fn get_previous_predict_value(self) -> i64 {
        self.sequences[0].values[0]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Sequence {
    values: Vec<i64>
}

impl Sequence {
    fn parse(input_line: &str) -> Self {
        let numbers:Vec<i64> = input_line.split_whitespace().map(|number| parse_numbers_i64(number).unwrap().1).collect();

        Sequence { values: numbers }
    }

    fn is_end(&self) -> bool {
        if self.values.iter().all(|value| *value == 0) {
            return true;
        }
        false
    }

    fn next_sequence(&self) -> Sequence {
        let values:Vec<i64> =  self.values.windows(2).map(|pair| pair[1] - pair[0]).collect();
        Sequence { values: values }
    }

    fn predict_next_value(&self, previous_diff: i64) -> Sequence {
        let mut updated = self.values.clone();
        updated.push(updated.last().unwrap() + previous_diff);
        Sequence { values: updated }
    }

    fn predict_previous_value(&self, previous_diff: i64) -> Sequence {
        let mut updated = self.values.clone();
        updated.insert(0, updated.first().unwrap() - previous_diff);
        Sequence { values: updated }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_09::{History, Sequence};

    #[test]
    fn can_get_next_diff_sequence() {
        let input = r#"0 3 6 9 12 15"#;

        let sequence = Sequence::parse(input);
        let next = sequence.next_sequence();

        assert_eq!(next.values.len(), 5);
        assert_eq!(next.values.iter().all(|x| *x == 3), true);
    }

    #[test]
    fn can_tell_if_sequence_is_end() {
        let input = r#"3 3 3 3 3"#;

        let sequence = Sequence::parse(input);

        assert_eq!(sequence.is_end(), false);

        assert_eq!(sequence.next_sequence().is_end(), true);
    }

    #[test]
    fn can_process_history_until_end() {
        let input = r#"0 3 6 9 12 15"#;

        let history = History::parse(input);

        let updated_history = history.process_until_end();
        assert_eq!(updated_history.sequences.len(), 3);
    }

    #[test]
    fn can_predict_next_sequence_value() {
        let input = r#"0 3 6 9 12 15"#;

        let history = History::parse(input);

        let updated_history = history.process_until_end().predict_sequences_next_value();

        assert_eq!(*updated_history.sequences.first().unwrap().values.last().unwrap(), 18);
    }

    #[test]
    fn sequence_can_add_prediction() {
        let input = r#"0 3 6 9 12 15"#;

        let seq = Sequence::parse(input);

        let updated_seq = seq.predict_next_value(3);

        assert_eq!(*updated_seq.values.last().unwrap(), 18);
    }

    #[test]
    fn sequence_can_add_previous_prediction() {
        let input = r#"10 13 16 21 30 45"#;
        let seq = Sequence::parse(input);

        let updated = seq.predict_previous_value(5);
        assert_eq!(updated.values[0], 5);
    }

    #[test]
    fn history_can_predict_previous_sequence_value() {
        let input = r#"10 13 16 21 30 45"#;

        let history = History::parse(input);

        let updated_history = history.process_until_end().predict_sequences_previous_value();

        assert_eq!(updated_history.sequences[0].values[0], 5);
    }


}