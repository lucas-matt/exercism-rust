use crate::Error::GameComplete;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<Box<dyn Frame>>
}

trait Frame {

    fn done(&self) -> bool;

}

struct Standard(Option<i32>, Option<i32>);

impl Frame for Standard {

    fn done(&self) -> bool {
        todo!()
    }

}

struct Final(Option<i32>, Option<i32>, Option<i32>);

impl Frame for Final {

    fn done(&self) -> bool {
        todo!()
    }

}


impl BowlingGame {

    pub fn new() -> Self {
        BowlingGame {
            rolls: vec!(Box::new(Standard(None, None)))
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
