use std::ops::Sub;

/// A 2D position in normalized coordinates (typically in the range `[0.0, 1.0]`).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Position {
    /// Creates a new [`Position`] with the given `x` and `y` coordinates.
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

/// A 2D size with `width` and `height` components in normalized coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
    /// Width component.
    pub width: f32,
    /// Height component.
    pub height: f32,
}

impl Size {
    /// Creates a new [`Size`] with the given `width` and `height` values.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// A 2D velocity vector representing the rate of change of a [`Position`] per frame.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Velocity {
    /// Horizontal velocity component.
    pub x: f32,
    /// Vertical velocity component.
    pub y: f32,
}

impl Velocity {
    /// Returns the scalar speed, i.e. the Euclidean magnitude of this velocity vector.
    pub fn speed(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Creates a new [`Velocity`] with the given `x` and `y` components.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
