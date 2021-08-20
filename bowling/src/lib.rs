use crate::Error::GameComplete;

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

    fn roll(&mut self, score:u16);

    fn score(&self) -> u16;

}

struct Standard(Option<u16>, Option<u16>);

impl Frame for Standard {

    fn done(&self) -> bool {
        match self {
            Standard(Some(_), Some(_)) => true,
            _ => false
        }
    }

    fn roll(&mut self, score: u16) {
        match self {
            Standard(None, None) => self.0 = Some(score),
            Standard(Some(_), None) => self.1 = Some(score),
            _ => panic!("Too many rolls")
        }
    }

    fn score(&self) -> u16 {
        [self.0, self.1].iter().filter(|o| o.is_some()).map()
    }

}

struct Final(Option<u16>, Option<u16>, Option<u16>);

impl Frame for Final {

    fn done(&self) -> bool {
        match self {
            Final(Some(10), Some(10), None) => false,
            Final(_, None, _) => false,
            _ => true
        }
    }

    fn roll(&mut self, score: u16) {
        match self {
            Final(None, None, None) => self.0 = Some(score),
            Final(Some(_), None, None) => self.1 = Some(score),
            Final(Some(10), Some(10), None) => self.2 = Some(score),
            _ => panic!("Too many rolls")
        }
    }
}


impl BowlingGame {

    pub fn new() -> Self {
        let mut frames:Vec<Box<dyn Frame>> = (0..10).map(|_| Box::new(Standard(None, None)) as Box<dyn Frame>).collect();
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
            Some(frame) => {
                frame.roll(pins);
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.done() {
            return None;
        }
        Some(0)
    }

}
