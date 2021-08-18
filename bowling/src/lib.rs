use crate::Frame::{Standard, Final};
use crate::Error::GameComplete;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<Frame>
}

enum Frame {
    Standard(Option<u16>, Option<u16>),
    Final(Option<u16>, Option<u16>, Option<u16>)
}

impl Frame {
    pub fn done(&self) -> bool {
        match self {
            Standard(Some(_), Some(_)) => true,
            Final(Some(10), Some(10), Some(_)) => true,
            Final(Some(10), Some(10), None) => false,
            Final(Some(_), Some(_), None) => true,
            _ => false
        }
    }
}

impl BowlingGame {

    pub fn new() -> Self {
        BowlingGame {
            rolls: vec!(Standard(None, None))
        }
    }

    fn done(&self) -> bool {
        (&self.rolls).into_iter().all(|frame| frame.done())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.done() {
            return Err(GameComplete)
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.done() {
            return None;
        }
        Some(0)
    }

}
