// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use crate::Direction::{North, East, South, West};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction
}

impl Direction {

    fn shift(&self, rotation:i32) -> Direction {
        let directions = [North, East, South, West];
        let idx = directions.iter().position(|d| d == self).unwrap() as i32;
        directions[((idx + rotation + 4) % 4) as usize]
    }

    pub fn right(&self) -> Direction {
        self.shift(1)
    }

    pub fn left(&self) -> Direction {
        self.shift(-1)
    }

    pub fn movement(&self) -> (i32, i32) {
        match self {
            North => (0, 1), East => (1, 0), South => (0, -1), West => (-1, 0)
        }
    }

}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot {
            x, y, direction
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = self.direction.right();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = self.direction.left();
        self
    }

    pub fn advance(mut self) -> Self {
        let (x, y) = self.direction.movement();
        self.x += x;
        self.y += y;
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, dir| match dir {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => robot
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
