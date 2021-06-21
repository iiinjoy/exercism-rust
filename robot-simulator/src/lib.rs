#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot { d, ..self }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Robot { d, ..self }
    }

    pub fn advance(self) -> Self {
        use Direction::*;
        match self.d {
            North => Robot {
                y: self.y + 1,
                ..self
            },
            South => Robot {
                y: self.y - 1,
                ..self
            },
            East => Robot {
                x: self.x + 1,
                ..self
            },
            West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| match c {
            'A' => acc.advance(),
            'L' => acc.turn_left(),
            'R' => acc.turn_right(),
            _ => panic!("invalid instruction: {}", c),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
