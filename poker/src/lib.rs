use crate::Rank::{ONE, THREE, FOUR, TWO, FIVE, EIGHT, QUEEN, KING, SIX, SEVEN, NINE, TEN, JACK};
use crate::Suit::{HEARTS, SPADES, DIAMONDS, CLUBS};

#[derive(Debug, Copy, Clone)]
enum Rank {
    ONE = 1,
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
    KING = 13
}

impl Rank {
    pub fn from(rank:&str) -> Option<Rank> {
        match rank {
            "1" => Some(ONE),
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

#[derive(Debug)]
struct Hand<'a> {
    cards: Vec<Card>,
    source: &'a str
}

impl<'a> Hand<'a> {
    fn score(&self) -> u32 {
        match &self.cards {
            cards => highest(cards)
        }
    }

}

fn highest(cards: &Vec<Card>) -> u32 {
    cards.iter()
        .map(|card| *(&card.rank) as u32)
        .sum()
}

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