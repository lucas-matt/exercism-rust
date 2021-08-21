use crate::Error::{GameComplete, NotEnoughPinsLeft};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Box<dyn Frame>>
}

trait Frame {

    fn done(&self) -> bool;

    fn roll(&mut self, score:u16) -> Result<(), Error>;

    fn score(&self) -> u16;

}

struct Standard(Option<u16>, Option<u16>);

fn validate_roll(score: u16, total: u16, max: u16) -> Result<(), Error> {
    if score > 10 || (score + total) > max {
        return Err(NotEnoughPinsLeft)
    }
    Ok(())
}

impl Frame for Standard {

    fn done(&self) -> bool {
        match self {
            Standard(Some(_), Some(_)) => true,
            _ => false
        }
    }

    fn roll(&mut self, score: u16) -> Result<(), Error> {
        validate_roll(score, self.score(), 10)?;
        match self {
            Standard(None, None) => {
                self.0 = Some(score);
                if score == 10 {
                    self.1 = Some(0)
                }
                Ok(())
            },
            Standard(Some(_), None) => { self.1 = Some(score); Ok(()) },
            _ => Err(NotEnoughPinsLeft)
        }
    }

    fn score(&self) -> u16 {
        [self.0, self.1].iter()
            .filter(|o| o.is_some())
            .map(|opt| opt.unwrap())
            .sum()
    }

}

struct Final(Option<u16>, Option<u16>, Option<u16>);

impl Frame for Final {

    fn done(&self) -> bool {
        match self {
            Final(Some(10), Some(10), None) | Final(_, None, None)  => false,
            _ => true
        }
    }

    fn roll(&mut self, score: u16) -> Result<(), Error> {
        match self {
            Final(None, None, None) => { self.0 = Some(score); Ok(()) },
            Final(Some(_), None, None) => { self.1 = Some(score); Ok(()) },
            Final(Some(_), Some(_), None) => { self.2 = Some(score); Ok(()) },
            _ => Err(NotEnoughPinsLeft)
        }
    }

    fn score(&self) -> u16 {
        [self.0, self.1, self.2].iter()
            .filter(|o| o.is_some())
            .map(|opt| opt.unwrap())
            .sum()
    }

}


impl BowlingGame {

    pub fn new() -> Self {
        let mut frames:Vec<Box<dyn Frame>> = (0..9).map(|_| Box::new(Standard(None, None)) as Box<dyn Frame>).collect();
        frames.push(Box::new(Final(None, None, None)));
        BowlingGame {
            frames
        }
    }

    fn done(&self) -> bool {
        (&self.frames).into_iter().all(|frame| frame.done())
    }

    fn next(&mut self) -> Option<&mut Box<dyn Frame>> {
        (&mut self.frames).into_iter().filter(|frame| !frame.done()).next()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.next() {
            None => Err(GameComplete),
            Some(frame) => frame.roll(pins)
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.done() {
            return None;
        }
        Some((&self.frames).into_iter().map(|frame| frame.score()).sum())
    }

}
