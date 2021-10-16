use crate::Rank::{ACE, THREE, FOUR, TWO, FIVE, EIGHT, QUEEN, KING, SIX, SEVEN, NINE, TEN, JACK};
use crate::Suit::{HEARTS, SPADES, DIAMONDS, CLUBS};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands:Vec<Hand> = hands.iter().map(|&str| parse(str))
        .collect::<Option<Vec<Hand>>>()
        .unwrap();
    let highest = hands.iter()
        .map(|hand| hand.score())
        .max()
        .unwrap_or(0);
    hands.iter()
        .filter(|hand| hand.score() == highest)
        .map(|hand| hand.source)
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14,
}

impl Rank {
    pub fn from(rank:&str) -> Option<Rank> {
        match rank {
            "A" => Some(ACE),
            "2" => Some(TWO),
            "3" => Some(THREE),
            "4" => Some(FOUR),
            "5" => Some(FIVE),
            "6" => Some(SIX),
            "7" => Some(SEVEN),
            "8" => Some(EIGHT),
            "9" => Some(NINE),
            "10" => Some(TEN),
            "J" => Some(JACK),
            "Q" => Some(QUEEN),
            "K" => Some(KING),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Suit {
    HEARTS,
    SPADES,
    DIAMONDS,
    CLUBS
}

impl Suit {
    pub fn from(suit:&str) -> Option<Suit> {
        match suit {
            "H" => Some(HEARTS),
            "S" => Some(SPADES),
            "D" => Some(DIAMONDS),
            "C" => Some(CLUBS),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

#[derive(Debug)]
struct Hand<'a> {
    cards: Vec<Card>,
    source: &'a str
}

impl<'a> Hand<'a> {
    fn score(&self) -> u64 {
        if let Some(score) = score::flush(&self.cards) {
            return score;
        }
        if let Some(score) = score::straight(&self.cards) {
            return score;
        }
        if let Some(score) = score::four_of_a_kind(&self.cards) {
            return score;
        }
        if let Some(score) = score::three_of_a_kind(&self.cards) {
            return score;
        }
        if let Some(score) = score::two_pair(&self.cards) {
            return score;
        }
        if let Some(score) = score::pair(&self.cards) {
            return score;
        }        
        return score::highest(&self.cards)
    }

}

fn parse(input:&str) -> Option<Hand> {
    let cards = input.split_ascii_whitespace().into_iter()
        .map(|pair| {
            let rank = Rank::from(&pair[0..pair.len()-1]);
            let suit = Suit::from(&pair[pair.len()-1..=pair.len()-1]);
            match (rank, suit) {
                (Some(rank), Some(suit)) => Some(
                    Card {
                        rank, suit
                    }
                ),
                _ => None
            }
        })
        .collect::<Option<Vec<Card>>>()?;
    Some(Hand {
        cards,
        source: input
    })
}

mod score {

    use crate::{Card, Rank, Suit};
    use std::collections::HashSet;

    // score boost 11
    pub fn flush(cards: &Vec<Card>) -> Option<u64> {
        let is_flush = cards.iter()
            .map(|card| card.suit)
            .collect::<HashSet<Suit>>().len() == 1;
        if !is_flush {
            return None;
        }
        let score = cards.iter()
            .map(|card| card.rank as u64 * 10_u64.pow(11))
            .sum();
        Some(score)
    }

    // score boost 10
    pub fn straight(cards: &Vec<Card>) -> Option<u64> {
        let mut cards:Vec<Rank> = cards.iter()
            .map(|card| card.rank)
            .collect();
        cards.sort();
        match cards.as_slice() {
            // handle low ace
            &[Rank::TWO, Rank::THREE, Rank::FOUR, Rank::FIVE, Rank::ACE] => Some(10_u64.pow(10)),
            _ => {
                let is_str = cards.windows(2)
                    .all(|window| match window {
                        &[x, y] if x as u32 == y as u32 - 1 => true,
                        _ => false
                    });
                if is_str {
                    Some(cards.into_iter()
                        .map(|rank| rank as u64)
                        .sum::<u64>() * 10_u64.pow(10))
                } else {
                    None
                }
            }
        }
    }

    // score boost 10^9
    pub fn four_of_a_kind(cards: &Vec<Card>) -> Option<u64> {
        group_score(cards, 4, 1, 9)
    }

    // score boost 10^7
    pub fn three_of_a_kind(cards: &Vec<Card>) -> Option<u64> {
        group_score(cards, 3, 1, 7)
    }

    // score boost 10^6
    pub fn two_pair(cards: &Vec<Card>) -> Option<u64> {
        group_score(cards, 2, 2, 6)
    }

    // score boost 10^5
    pub fn pair(cards: &Vec<Card>) -> Option<u64> {
        group_score(cards, 2, 1, 5)
    }

    // score boost 10^0 through 10^4
    pub fn highest(cards: &Vec<Card>) -> u64 {
        let mut ranks:Vec<u64> = cards.iter()
            .map(|card| *(&card.rank) as u64)
            .collect();
        ranks.sort();
        ranks.into_iter().enumerate()
            .map(|(i, rank)| rank * 10_u64.pow(i as u32))
            .sum()
    }

    fn group_score(cards: &Vec<Card>, size:usize, count:usize, boost:u32) -> Option<u64> {
        let mut cards:Vec<Rank> = cards.iter()
            .map(|card| card.rank)
            .collect();
        cards.sort();
        let in_group:Vec<Rank> = cards.windows(size)
            .filter(|&window| window.iter()
                .collect::<HashSet<&Rank>>()
                .len() == 1
            )
            .map(|window| window.first().unwrap())
            .cloned()
            .collect();
        let remainder:Vec<&Rank> = cards.iter()
            .filter(|&rank| !in_group.contains(&rank))
            .collect();
        if in_group.len() != count {
            return None
        }
        let score = in_group
            .iter()
            .map(|rank| *rank as u64 * 10_u64.pow(boost))
            .chain(remainder.iter().map(|&rank| *rank as u64))
            .sum();
        Some(score)
    }

}


mod scores {

    trait Score {}

    struct Highest {

    }

    struct Pair {

    }

    struct TwoPair {

    }

    struct ThreeOfAKind {

    }

    struct FourOfAKind {

    }

    struct Straight {

    }

    struct Flush {

    }

    struct FullHouse {

    }

}

