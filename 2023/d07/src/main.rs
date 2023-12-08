use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file1 = fs::File::open(filename)?;
    let file2 = fs::File::open(filename)?;
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    println!("Part one: {}", process_part_one(reader1));
    println!("Part two: {}", process_part_two(reader2));
    Ok(())
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum CardLabels {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl CardLabels {
    fn from_char(c: char) -> Option<CardLabels> {
        match c {
            'A' => Some(CardLabels::A),
            'K' => Some(CardLabels::K),
            'Q' => Some(CardLabels::Q),
            'J' => Some(CardLabels::J),
            'T' => Some(CardLabels::T),
            '2' => Some(CardLabels::N2),
            '3' => Some(CardLabels::N3),
            '4' => Some(CardLabels::N4),
            '5' => Some(CardLabels::N5),
            '6' => Some(CardLabels::N6),
            '7' => Some(CardLabels::N7),
            '8' => Some(CardLabels::N8),
            '9' => Some(CardLabels::N9),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type() && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() != other.hand_type() {
            return self.hand_type().partial_cmp(&other.hand_type());
        }
        let self_card_vals = self.cards.chars().filter_map(CardLabels::from_char);
        let other_card_vals = other.cards.chars().filter_map(CardLabels::from_char);

        self_card_vals.partial_cmp(other_card_vals)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn hand_type(&self) -> Option<HandType> {
        let cards: Vec<char> = self.cards.chars().collect();

        let mut card_label_counts: HashMap<char, u32> = HashMap::new();
        for &card in &cards {
            *card_label_counts.entry(card).or_insert(0) += 1;
        }
        let j_count = card_label_counts.remove(&'J').unwrap_or(0);
        let has_five_of_a_kind = card_label_counts.values().max().unwrap_or(&0) + j_count == 5;
        if has_five_of_a_kind {
            return Some(HandType::FiveOAK);
        };
        if card_label_counts.iter().any(|(_, &v)| v + j_count == 4) {
            return Some(HandType::FourOAK);
        };
        if card_label_counts.len() == 2 {
            return Some(HandType::FullHouse);
        };
        if card_label_counts.iter().any(|(_, &v)| v + j_count == 3) {
            return Some(HandType::ThreeOAK);
        };
        if card_label_counts.len() == 3 {
            return Some(HandType::TwoPair);
        }
        if card_label_counts.len() == 4 {
            return Some(HandType::OnePair);
        }
        Some(HandType::HighCard)
    }
}

fn parse_data<R: Read>(reader: BufReader<R>) -> Vec<Hand> {
    let lines = reader.lines();
    lines
        .flatten()
        .map(|item| {
            let parts: Vec<_> = item.split_whitespace().collect();
            let (cards, bid_str) = (parts[0], parts[1]);
            let bid = bid_str.parse::<u32>().expect("is digits");
            Hand {
                cards: cards.to_string(),
                bid,
            }
        })
        .collect()
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let mut hands = parse_data(reader);
    hands.sort();
    let score: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u32)
        .sum();
    score
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
    let mut hands = parse_data(reader);
    hands.sort();
    let score: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u32)
        .sum();
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    // #[test]
    // fn test_process_part_one() {
    //     let input_bytes = INPUT.as_bytes();
    //     assert_eq!(6440, process_part_one(BufReader::new(input_bytes)));
    // }

    #[test]
    fn test_card_values() {
        let hand1 = Hand {
            cards: "8A888".to_string(),
            bid: 100,
        };
        let hand2 = Hand {
            cards: "AABBJ".to_string(),
            bid: 200,
        };
        assert_eq!(hand1.hand_type().unwrap(), HandType::FourOAK);
        assert_eq!(hand2.hand_type().unwrap(), HandType::FullHouse);
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(5905, process_part_two(BufReader::new(input_bytes)));
    }
}
