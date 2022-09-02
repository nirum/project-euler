use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use itertools::Itertools;
use std::fmt;


#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Score::HighCard => write!(f, "High Card"),
            Score::OnePair => write!(f, "One Pair"),
            Score::TwoPair => write!(f, "Two Pair"),
            Score::ThreeOfAKind => write!(f, "Three of a Kind"),
            Score::Straight => write!(f, "Straight"),
            Score::Flush => write!(f, "Flush"),
            Score::FullHouse => write!(f, "Full House"),
            Score::FourOfAKind => write!(f, "Four of a Kind"),
            Score::StraightFlush => write!(f, "Straight Flush"),
            Score::RoyalFlush => write!(f, "Royal Flush"),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Debug)]
struct Card {
    suit: Suit,
    value: i32,
}

type Hand = [Card; 5];


fn check_if_straight(values: Vec<i32>) -> bool {
    let current = values[0];
    for (index, &value) in values.iter().enumerate() {
        if value != (current + index as i32) {
            return false;
        }
    };
    return true;
}


fn score_hand(hand: Hand) -> (Score, i32) {

    // Check if flush.
    let suits = hand.clone().into_iter().map(|x| x.suit);
    let is_flush: bool = Itertools::unique(suits).count() == 1;

    // Check if straight.
    let mut values: Vec<i32> = hand.clone().into_iter().map(|x| x.value).collect();
    values.sort();
    let is_straight: bool = check_if_straight(values.clone());
    values.reverse();
    let high_card = *values.get(0).unwrap();

    // Check number of pairs (matches).
    let counts_map = hand.clone().into_iter().map(|x| x.value).counts();
    let mut counts: Vec<usize> = counts_map.clone().into_values().collect();
    counts.sort();
    counts.reverse();

    // Mapping from counts to values.
    let mut count_to_value: HashMap<usize, i32> = HashMap::new();
    for (&key, &val) in counts_map.iter() {
        if count_to_value.contains_key(&val) {
            // Pick whichever is larger.
            let existing = match count_to_value.get(&val) {
                Some(v) => v,
                None => &0
            };
            if key <= *existing {
                continue;
            }
        }
        count_to_value.insert(val, key);
    }

    if is_flush && is_straight {
        if hand.get(0).unwrap().value == 9 { // Royal Flush (starts with 10).
            return (Score::RoyalFlush, 0);
        } else {
            return (Score::StraightFlush, high_card);
        }
    }

    if counts[0] == 4 {
        return (Score::FourOfAKind, count_to_value[&4]);
    } else if counts[0] == 3 && counts[1] == 2 {
        return (Score::FullHouse, count_to_value[&3]);
    } else if is_flush {
        return (Score::Flush, high_card);
    } else if is_straight {
        return (Score::Straight, high_card);
    } else if counts[0] == 3 {
        return (Score::ThreeOfAKind, count_to_value[&3]);
    } else if counts[0] == 2 && counts[1] == 2 {
        return (Score::TwoPair, count_to_value[&2]);
    } else if counts[0] == 2 {
        return (Score::OnePair, count_to_value[&2]);
    } else {
        return (Score::HighCard, high_card);
    }
}


fn parse_card(card_str: String) -> Result<Card, String> {
    let char_vec: Vec<char> = card_str.chars().collect();
    let value: i32 = match char_vec.get(0) {
        Some(c) => match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return Err("Could not parse card value.".to_string()),
        },
        None => return Err("No card value found.".to_string()),
    };
    let suit: Suit = match char_vec.get(1) {
        Some(c) => match c {
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => return Err("Could not parse card suit.".to_string()),
        },
        None => return Err("No card suit found.".to_string()),
    };
    return Ok(Card { suit, value });
}


fn parse_line(line: String) -> (Hand, Hand) {
    // Parses a string line into a pair of Hands.
    let mut cards = Vec::new();
    for card_str in line.split(' ') {
        let card: Card = parse_card(card_str.to_string()).unwrap();
        cards.push(card)
    }

    let p2: Hand = [
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
    ];

    let p1: Hand = [
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
        cards.pop().unwrap(),
    ];

    return (p1, p2);
}

fn score_file(filepath: &str) -> std::io::Result<i32> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut p1_wins = 0;
    for line in reader.lines() {
        let (p1, p2): (Hand, Hand) = parse_line(line.unwrap());

        let s1 = score_hand(p1);
        let s2 = score_hand(p2);
        // println!("Hand 1: {} ({}), Hand 2: {} ({})", s1, v1, s2, v2);

        match s1.cmp(&s2) {
            Ordering::Less => {},
            Ordering::Greater => p1_wins += 1,
            Ordering::Equal => {},
        };
    }
    Ok(p1_wins)
}


pub fn main() -> f64 {
    let result = score_file("data/p054_poker.txt");
    match result {
        Ok(v) => return v as f64,
        Err(_) => return -1 as f64,
    }
}
