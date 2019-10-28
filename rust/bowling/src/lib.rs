#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    is_second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            is_second: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.is_second == true && self.rolls.last().unwrap() + pins > 10) {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        self.rolls.push(pins);
        self.is_second = if pins != 10 { !self.is_second } else { false };
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut idx = 0;

        for _ in 0..10 {
            if let (Some(&one), Some(&two)) = (self.rolls.get(idx), self.rolls.get(idx + 1)) {
                score += one + two;
                if is_strike_or_spare(one, two) {
                    if let Some(&three) = self.rolls.get(idx + 2) {
                        score += three;
                    } else {
                        return None;
                    }
                }
                idx += if one == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }

        Some(score)
    }
}

fn is_strike_or_spare(roll_1: u16, roll_2: u16) -> bool {
    roll_1 == 10 || roll_1 + roll_2 == 10
}
