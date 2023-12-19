use core::panic;
use std::{str::FromStr, collections::HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn determine_hand_type(cards: &[Card; 5]) -> Self {
        let mut freq: HashMap<&Card, u32> = HashMap::with_capacity(5);

        for card in cards {
            freq.entry(card).and_modify(|e| { *e += 1 }).or_insert(1);
        }

        match freq.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let mut values = freq.values();
                let val1 = values.next().unwrap();
                let val2 = values.next().unwrap();

                match (val1, val2) {
                    (1 | 4, 4 | 1) => HandType::FourOfAKind,
                    (3 | 2, 2 | 3) => HandType::FullHouse,
                    _ => panic!("Invalid case: val1 = {}, val2 = {}", val1, val2),
                }
            },
            3 => {
                let mut values = freq.values();
                let val1 = values.next().unwrap();
                let val2 = values.next().unwrap();
                let val3 = values.next().unwrap();

                match (val1, val2, val3) {
                    (3, _, _) | (_, 3, _) | (_, _, 3) => HandType::ThreeOfAKind,
                    (1, 2, 2) | (2, 1, 2) | (2, 2, 1) => HandType::TwoPair,
                    _ => panic!("Invalid case: val1 = {}, val2 = {}, val3 = {}", val1, val2, val3),
                }
            },
            4 => {
                let mut values = freq.values();
                let val1 = values.next().unwrap();
                let val2 = values.next().unwrap();
                let val3 = values.next().unwrap();
                let val4 = values.next().unwrap();

                match (val1, val2, val3, val4) {
                    (2, _, _, _) |
                    (_, 2, _, _) |
                    (_, _, 2, _) |
                    (_, _, _, 2) => HandType::OnePair,
                    _ => panic!("Invalid case: val1 = {}, val2 = {}, val3 = {}, val4 = {}", val1, val2, val3, val4),
                }
            },
            5 => HandType::HighCard,
            _ => panic!("Invalid number of cards ({})", cards.len()),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    r#type: HandType,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(format!("Invalid card character ({})", value)),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_unparsed, bid) = s.split_once(' ').unwrap();

        if cards_unparsed.len() != 5 {
            return Err(format!("Length of hand isn’t 5 but it’s {}", cards_unparsed.len()))
        }

        let bid: u32 = bid.parse().unwrap();

        let mut cards = [Card::Two; 5];

        for (i, c) in cards_unparsed.chars().enumerate() {
            cards[i] = c.try_into().unwrap();
        }

        let r#type = HandType::determine_hand_type(&cards);

        Ok(Hand { cards, bid, r#type })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();

    hands.sort_by(|hand1, hand2| {
        if hand1.r#type == hand2.r#type {
            for (card1, card2) in hand1.cards.iter().zip(hand2.cards.iter()) {
                if card1 == card2 {
                    continue;
                }

                return (*card1 as u8).cmp(&(*card2 as u8))
            }
        }

        (hand1.r#type as u8).cmp(&(hand2.r#type as u8))
    });

    (1..=hands.len()).zip(hands.iter()).map(|(rank, hand)| {
        (rank as u32) * hand.bid
    }).sum()
}

#[test]
fn test_part1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(6440, part1(input));
}
