use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position : (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let next_direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot {
            direction: next_direction,
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        let next_direction = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Robot {
            direction: next_direction,
            ..self
        }
    }

    pub fn advance(self) -> Self {
        let change = match self.direction {
            North => (0,1),
            East => (1,0),
            South => (0,-1),
            West => (-1,0)
        };
        Robot {
            position: add(self.position, change),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, x| match x {
            'R' => acc.turn_right(),
            'A' => acc.advance(),
            'L' => acc.turn_left(),
            _ => panic!("Invalid expression, {:?}", x),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0+b.0, a.1+b.1)
}