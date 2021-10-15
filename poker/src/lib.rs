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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn score(&self) -> u32 {
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

    use crate::Card;

    // score boost 10^7
    pub fn three_of_a_kind(cards: &Vec<Card>) -> Option<u32> {
        let mut cards = cards.clone();
        cards.sort_by_key(|card| card.rank);
        let three = cards.windows(3)
            .filter(|&window| {
                match window {
                    &[a, b, c] if a.rank == b.rank && b.rank == c.rank => true,
                    _ => false
                }
            })
            .last();
        three
            .map(|cards| cards.iter().map(|card| card.rank as u32).sum::<u32>())
            .map(|score| score * 10_u32.pow(7))
    }

    // score boost 10^6
    pub fn two_pair(cards: &Vec<Card>) -> Option<u32> {
        if let (Some(pairs), remainder) = find_pairs(cards) {
            if pairs.len() == 2  {
                let score:u32 = pairs.iter()
                    .map(|pair| pair.0.rank as u32 * 10_u32.pow(6))
                    .chain(remainder.iter().map(|card| card.rank as u32))
                    .sum();
                return Some(score)
            }
        }
        return None
    }

    // score boost 10^5
    pub fn pair(cards: &Vec<Card>) -> Option<u32> {
        if let (Some(pairs), _) = find_pairs(cards) {
            if let Some(first) = pairs.first() {
                let rank = first.0.rank as u32;
                return Some(rank * 10_u32.pow(5));
            }
        }
        return None
    }

    // score boost 10^0 through 10^4
    pub fn highest(cards: &Vec<Card>) -> u32 {
        let mut ranks:Vec<u32> = cards.iter()
            .map(|card| *(&card.rank) as u32)
            .collect();
        ranks.sort();
        ranks.into_iter().enumerate()
            .map(|(i, rank)| rank * 10_u32.pow(i as u32))
            .sum()
    }

    /** 
     * Returns a list of pairs, and a list of unpaired
     */
    fn find_pairs(cards: &Vec<Card>) -> (Option<Vec<(Card, Card)>>, Vec<Card>) {
        let mut pairs:Vec<(Card, Card)> = Vec::new();
        for i in 0..cards.len() {
            for j in i+1..cards.len() {
                if cards[i].rank == cards[j].rank {
                    pairs.push((cards[i], cards[j]))
                }
            }
        }
        if pairs.is_empty() {
            return (None, cards.clone())
        }
        let paired: Vec<Card> = pairs.iter()
                                        .flat_map(|(x, y)| vec!(*x, *y))
                                        .collect();
        let remainder = cards.iter().filter(|&card| !paired.contains(card)).cloned().collect();
        return (Some(pairs), remainder)
    }

}


