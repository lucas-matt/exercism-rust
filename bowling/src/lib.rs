#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current: Vec<u16>,
    frames: Vec<Vec<u16>>
}

trait Frame {
    fn is_complete(&self) -> bool;
    fn check_pins(&self, pins:u16) -> Result<(), Error>;
}

struct Standard {
    rolls: Vec<u16>
}

struct Final {
    rolls: Vec<u16>
}

impl Frame for Standard {
    fn is_complete(&self) -> bool {
        self.rolls.len() >= 2 || score(&self.rolls) >= 10
    }

    fn check_pins(&self, pins: u16) -> Result<(), Error> {
        check_pins(&self.rolls, pins, 10)
    }
}

impl Frame for Final {
    fn is_complete(&self) -> bool {
        match self.rolls.as_slice() {
            &[x, y] if x + y < 10 => true,
            &[_, _, _] => true,
            _ => false
        }
    }

    fn check_pins(&self, pins: u16) -> Result<(), Error> {
        check_pins(&self.rolls, pins, 30)?;
        match self.rolls.as_slice() {
            &[10, x] if x < 10 && x + pins > 10 => Err(Error::NotEnoughPinsLeft),
            _ => Ok(())
        }
    }
}

fn check_pins(rolls:&Vec<u16>, pins:u16, max:u16) -> Result<(), Error> {
    let score = rolls.iter().sum::<u16>();
    if pins > 10 || pins + score > max {
        return Err(Error::NotEnoughPinsLeft)
    }
    Ok(())
}

fn score(rolls: &Vec<u16>) -> u16 {
    rolls.iter().sum::<u16>()
}


impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current: vec!(),
            frames: vec!()
        }
    }

    fn is_complete(&self) -> bool {
        self.frames.len() == 10
    }

    fn validate(&self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }
        self.frame().check_pins(pins)
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.validate(pins)?;
        self.current.push(pins);
        if self.frame().is_complete() {
            self.frames.push(self.current.clone());
            self.current = vec!();
        }
        Ok(())
    }

    fn frame(&self) -> Box<dyn Frame> {
        match self.frames.len() + 1 {
            10 => Box::new(Final{rolls: self.current.clone()}),
            _ => Box::new(Standard{rolls: self.current.clone()})
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }
        let mut frames = self.frames.clone();
        let mut score = 0;
        while !frames.is_empty() {
            let frame = frames.remove(0);
            score += match frame.as_slice() {
                // strike
                &[10] => 10 + next(2, &frames),
                // spare
                &[x, y] if x + y == 10 => x + y + next(1, &frames),
                // open
                &[x, y] => x + y,
                // final
                &[x, y, z] => x + y + z,
                _ => 0
            };
        }
        Some(score)
    }
}

fn next(n: usize, frames: &Vec<Vec<u16>>) -> u16 {
    frames.iter()
        .flat_map(|v| v)
        .take(n)
        .sum()
}
