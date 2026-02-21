use std::ops::Sub;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Sub for Position {
    type Output = Velocity;

    fn sub(self, rhs: Self) -> Self::Output {
        Velocity {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn speed(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
