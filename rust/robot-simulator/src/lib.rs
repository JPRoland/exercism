// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = self.direction.turn_right();
        Robot {
            direction: new_direction,
            x: self.x,
            y: self.y,
        }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = self.direction.turn_left();
        Robot {
            direction: new_direction,
            x: self.x,
            y: self.y,
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };

        Robot {
            x,
            y,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |new_robot, inst| match inst {
                'A' => new_robot.advance(),
                'L' => new_robot.turn_left(),
                'R' => new_robot.turn_right(),
                _ => new_robot,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
