use std::cmp::Ordering;
use std::fmt::{Display, Formatter, write};
use std::ops::Index;
use itertools::Itertools;
use crate::tools::{parse_numbers_u64, usize_to_u32, usize_to_u64};

const CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

pub fn part_one(input: String) -> impl Display {
    let hands:Vec<Hand> = input.lines().map(|line| Hand::parse(line)).sorted().collect();
    //bidder(hands)
    0
}

pub fn part_two(input: String) -> impl Display {
    let hands:Vec<Hand> = input.lines().map(|line| Hand::parse_v2(line)).sorted().collect();
    bidder(hands)
}

fn bidder(hands:Vec<Hand>) -> u64 {
    let mut running_total:u64 = 0;
    for i in 0..hands.iter().count() {
        let hand = hands.get(i).unwrap();
        running_total += hand.score * usize_to_u64(i + 1).unwrap();
        let str:String = hand.cards.iter().collect();
    }

    running_total
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

impl Display for HandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "Five of a kind"),
            HandType::FourOfAKind => write!(f, "Four of a kind"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::ThreeOfAKind => write!(f, "Three of a kind"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::OnePair => write!(f, "One Pair"),
            HandType::HighCard => write!(f, "High Card"),
        }
    }
}

struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    score: u64
}

impl Hand {
    fn parse(input_line: &str) -> Self {
        let mut split = input_line.split_whitespace();
        let hand_str:Vec<char> = split.nth(0).unwrap().chars().collect();
        let hand_type = Hand::determine_hand_type(hand_str.clone());
        let score = parse_numbers_u64(split.nth(0).unwrap()).unwrap().1;

        Hand { cards: hand_str, hand_type, score }
    }

    fn parse_v2(input_line: &str) -> Self {
        let mut split = input_line.split_whitespace();
        let hand_str:Vec<char> = split.nth(0).unwrap().chars().collect();
        let hand_type = Hand::determine_hand_type_v2(hand_str.clone());
        let score = parse_numbers_u64(split.nth(0).unwrap()).unwrap().1;

        Hand { cards: hand_str, hand_type, score }
    }

    fn determine_hand_type_v2(hand: Vec<char>) -> HandType {
        let mut card_counter:Vec<usize> = Vec::new();
        let mut jokers = hand.iter().filter(|c| *c == &'J').count();
        for card in CARDS {
            if card == 'J'
            {
                continue;
            }
            card_counter.push(hand.iter().filter(|x| *x == &card).count());
        }

        let mut sorted:Vec<&usize> = card_counter.iter().sorted_by(|a, b| b.cmp(a)).map(|a| a).collect();
        let mut updated_card_counts:Vec<usize> = Vec::new();

        for card_count in sorted {
            if(jokers > 0){
                updated_card_counts.push(card_count + jokers);
                jokers = 0;
            }
            else {
                updated_card_counts.push(*card_count);
            }
        }

        if(updated_card_counts.contains(&(5usize))){
            return HandType::FiveOfAKind;
        }
        if(updated_card_counts.contains(&(4usize))){
            return HandType::FourOfAKind;
        }
        if updated_card_counts.iter().filter(|count| *count >= &(3usize)).count() == 2 {
            return HandType::FullHouse;
        }
        if updated_card_counts.contains(&(3usize)) && updated_card_counts.contains(&(2usize)) {
            return HandType::FullHouse;
        }
        if updated_card_counts.contains(&(3usize)) {
            return HandType::ThreeOfAKind;
        }
        if updated_card_counts.iter().filter(|x| *x == &2usize).count() == 2 {
            return HandType::TwoPair;
        }
        if updated_card_counts.contains(&2usize){
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }
    fn determine_hand_type(hand: Vec<char>) -> HandType {
        let mut card_counter:Vec<usize> = Vec::new();
        for card in CARDS {
            card_counter.push(hand.iter().filter(|x| *x == &card).count());
        }

        if(card_counter.contains(&(5usize))){
            return HandType::FiveOfAKind;
        }
        if(card_counter.contains(&(4usize))){
            return HandType::FourOfAKind;
        }
        if(card_counter.contains(&(3usize)) && card_counter.contains(&(2usize))){
            return HandType::FullHouse;
        }
        if card_counter.contains(&(3usize)) {
            return HandType::ThreeOfAKind;
        }
        if card_counter.iter().filter(|x| *x == &2usize).count() == 2 {
            return HandType::TwoPair;
        }
        if card_counter.contains(&2usize){
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if(self.hand_type != other.hand_type){
            return self.hand_type.cmp(&other.hand_type);
        }
        for i in 0..5 {
            if(self.cards[i] != other.cards[i]){
                return CARDS.iter().position(|&c| c == other.cards[i]).unwrap().cmp(&CARDS.iter().position(|&c| c == self.cards[i]).unwrap())
            }
        }
        println!("help");
        self.score.cmp(&other.score)
    }

}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::days::day_07::{bidder, Hand, HandType};

    #[test]
    fn hand_can_be_parsed_from_string() {
        let input = r#"32T3K 765"#;

        let hand = Hand::parse(input);

        assert_eq!(hand.hand_type, HandType::OnePair);
        assert_eq!(hand.score, 765);
    }

    #[test]
    fn two_pair_hand_can_be_parsed_from_string() {
        let input = r#"KTJJT 220"#;

        let hand = Hand::parse(input);

        assert_eq!(hand.hand_type, HandType::TwoPair);
        assert_eq!(hand.score, 220);
    }

    #[test]
    fn hands_can_be_sorted(){
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let hands:Vec<Hand> = input.lines().map(|line| Hand::parse(line)).sorted().collect();

        assert_eq!(hands[0].cards, vec!['3','2','T','3','K']);
        assert_eq!(hands[4].cards, vec!['Q','Q','Q','J','A']);

    }

    #[test]
    fn running_total_can_be_processed() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let hands:Vec<Hand> = input.lines().map(|line| Hand::parse(line)).sorted().collect();
        let total = bidder(hands);

        assert_eq!(total, 6440);
    }

    #[test]
    fn jokers_can_be_processed() {
        let input = r#"T55J5 684"#;

        let hand = Hand::parse_v2(input);

        assert_eq!(hand.hand_type, HandType::FourOfAKind);
        assert_eq!(hand.score, 684);
    }

    #[test]
    fn joker_hands_can_be_bidded() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let hands:Vec<Hand> = input.lines().map(|line| Hand::parse_v2(line)).sorted().collect();
        let total = bidder(hands);

        assert_eq!(total, 5905);
    }

    #[test]
    fn joker_hands_can_be_sorted() {
        let hand_1 = Hand::parse_v2(r#"KTJJT 220"#);
        let hand_2 = Hand::parse_v2(r#"QQQJA 483"#);

        let ordering = hand_1.cmp(&hand_2);
        assert_eq!(ordering.is_gt(), true);
    }

    #[test]
    fn all_joker_hand_is_processed() {
        let hand_1 = Hand::parse_v2(r#"JJJJJ 287"#);

        assert_eq!(hand_1.hand_type, HandType::FiveOfAKind);
    }

    #[test]
    fn full_houses_are_parsed() {
        let hand_1 = Hand::parse_v2(r#"JKKQQ 100"#);

        assert_eq!(hand_1.hand_type, HandType::FullHouse);
    }

    #[test]
    fn jokers_are_not_reused() {
        let hand_1 = Hand::parse_v2(r#"JKKTQ 100"#);

        assert_eq!(hand_1.hand_type, HandType::ThreeOfAKind);
    }
}