#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    roll_index: usize,
    rolls: [u16; 21],
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.roll_index >= self.rolls.len() {
            return Err(Error::GameComplete);
        }
        self.rolls[self.roll_index] = pins;
        let frame_index = self.roll_index / 2;

        if frame_index < 9 {
            match (self.is_frame_begin(), self.is_frame_end()) {
                (true, _) if pins > 10 => return Err(Error::NotEnoughPinsLeft),
                (_, true) if self.rolls[self.roll_index - 1] + pins > 10 => {
                    return Err(Error::NotEnoughPinsLeft)
                }
                _ => (),
            }
            self.roll_index += if pins == 10 { 2 } else { 1 };
            Ok(())
        } else if frame_index == 9 {
            let r18 = self.rolls[18];
            let r19 = self.rolls[19];
            match (self.is_frame_begin(), self.is_frame_end(), r18, r19) {
                (true, _, _, _) if pins > 10 => return Err(Error::NotEnoughPinsLeft),
                (_, true, a, b) if a != 10 && a + b > 10 => return Err(Error::NotEnoughPinsLeft),
                (_, true, 10, b) if b > 10 => return Err(Error::NotEnoughPinsLeft),
                _ => (),
            }
            if self.is_strike(frame_index)
                || self.is_frame_end() && self.is_spare(frame_index)
                || !self.is_frame_end()
            {
                self.roll_index += 1;
            } else {
                self.roll_index = self.rolls.len();
            }
            Ok(())
        } else {
            let r18 = self.rolls[18];
            let r19 = self.rolls[19];
            let r20 = self.rolls[20];
            match (r18, r19, r20) {
                (10, b, c) if b != 10 && b + c > 10 => Err(Error::NotEnoughPinsLeft),
                (_, _, c) if c > 10 => Err(Error::NotEnoughPinsLeft),
                _ => {
                    self.roll_index += 1;
                    Ok(())
                }
            }
        }
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index * 2] == 10
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        (self.rolls[frame_index * 2] + self.rolls[frame_index * 2 + 1]) == 10
    }

    fn is_frame_begin(&self) -> bool {
        self.roll_index % 2 == 0
    }

    fn is_frame_end(&self) -> bool {
        self.roll_index % 2 == 1
    }

    fn frame_score(&self, frame_index: usize) -> u16 {
        if self.is_strike(frame_index) && frame_index == 9 {
            let n1_score = self.rolls[frame_index * 2 + 1];
            let n2_score = self.rolls[frame_index * 2 + 2];
            10 + n1_score + n2_score
        } else if self.is_strike(frame_index) && frame_index == 8 {
            let n1_score = self.rolls[(frame_index + 1) * 2];
            let n2_score = self.rolls[(frame_index + 1) * 2 + 1];
            10 + n1_score + n2_score
        } else if self.is_strike(frame_index) {
            let n1_score = self.rolls[(frame_index + 1) * 2];
            let mut n2_score = self.rolls[(frame_index + 1) * 2 + 1];
            if n1_score == 10 {
                n2_score = self.rolls[(frame_index + 2) * 2];
            }
            10 + n1_score + n2_score
        } else if self.is_spare(frame_index) {
            let n1_score = self.rolls[(frame_index + 1) * 2];
            10 + n1_score
        } else {
            self.rolls[frame_index * 2] + self.rolls[frame_index * 2 + 1]
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.roll_index < self.rolls.len() {
            None
        } else {
            Some(
                (0..10)
                    .map(|frame_index| self.frame_score(frame_index))
                    .sum(),
            )
        }
    }
}
