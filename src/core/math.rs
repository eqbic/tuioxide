use std::ops::{Add, Sub};

/// A 2D position in normalized coordinates (typically in the range `[0.0, 1.0]`).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // ── Position ────────────────────────────────────────────────────────────

    #[test]
    fn position_new_stores_components() {
        let p = Position::new(0.3, 0.7);
        assert_relative_eq!(p.x, 0.3);
        assert_relative_eq!(p.y, 0.7);
    }

    #[test]
    fn position_add_sums_components() {
        let a = Position::new(0.2, 0.3);
        let b = Position::new(0.5, 0.1);
        let result = a + b;
        assert_relative_eq!(result.x, 0.7);
        assert_relative_eq!(result.y, 0.4);
    }

    #[test]
    fn position_add_returns_position() {
        // The Output type of Add<Position> for Position is Position (not Velocity).
        let a = Position::new(0.0, 0.0);
        let b = Position::new(1.0, 1.0);
        let result: Position = a + b;
        assert_eq!(result, Position::new(1.0, 1.0));
    }

    #[test]
    fn position_add_identity() {
        let p = Position::new(0.5, 0.5);
        let zero = Position::new(0.0, 0.0);
        assert_eq!(p + zero, p);
    }

    #[test]
    fn position_sub_yields_velocity() {
        let a = Position::new(0.8, 0.6);
        let b = Position::new(0.3, 0.1);
        let v: Velocity = a - b;
        assert_relative_eq!(v.x, 0.5);
        assert_relative_eq!(v.y, 0.5);
    }

    #[test]
    fn position_sub_negative_components() {
        let a = Position::new(0.1, 0.2);
        let b = Position::new(0.4, 0.9);
        let v: Velocity = a - b;
        assert_relative_eq!(v.x, -0.3);
        assert_relative_eq!(v.y, -0.7);
    }

    #[test]
    fn position_sub_same_gives_zero_velocity() {
        let p = Position::new(0.5, 0.5);
        let v: Velocity = p - p;
        assert_relative_eq!(v.x, 0.0);
        assert_relative_eq!(v.y, 0.0);
        assert_relative_eq!(v.speed(), 0.0);
    }

    #[test]
    fn position_default_is_origin() {
        let p = Position::default();
        assert_relative_eq!(p.x, 0.0);
        assert_relative_eq!(p.y, 0.0);
    }

    // ── Velocity ────────────────────────────────────────────────────────────

    #[test]
    fn velocity_new_stores_components() {
        let v = Velocity::new(3.0, 4.0);
        assert_relative_eq!(v.x, 3.0);
        assert_relative_eq!(v.y, 4.0);
    }

    #[test]
    fn velocity_speed_pythagorean_triple() {
        // 3-4-5 right triangle
        let v = Velocity::new(3.0, 4.0);
        assert_relative_eq!(v.speed(), 5.0);
    }

    #[test]
    fn velocity_speed_zero_vector() {
        let v = Velocity::new(0.0, 0.0);
        assert_relative_eq!(v.speed(), 0.0);
    }

    #[test]
    fn velocity_speed_unit_x() {
        let v = Velocity::new(1.0, 0.0);
        assert_relative_eq!(v.speed(), 1.0);
    }

    #[test]
    fn velocity_speed_unit_y() {
        let v = Velocity::new(0.0, 1.0);
        assert_relative_eq!(v.speed(), 1.0);
    }

    #[test]
    fn velocity_speed_negative_components() {
        // Magnitude is always positive regardless of sign
        let v = Velocity::new(-3.0, -4.0);
        assert_relative_eq!(v.speed(), 5.0);
    }

    #[test]
    fn velocity_default_is_zero() {
        let v = Velocity::default();
        assert_relative_eq!(v.speed(), 0.0);
    }

    // ── Size ─────────────────────────────────────────────────────────────────

    #[test]
    fn size_new_stores_components() {
        let s = Size::new(0.4, 0.8);
        assert_relative_eq!(s.width, 0.4);
        assert_relative_eq!(s.height, 0.8);
    }

    #[test]
    fn size_default_is_zero() {
        let s = Size::default();
        assert_relative_eq!(s.width, 0.0);
        assert_relative_eq!(s.height, 0.0);
    }

    #[test]
    fn size_equality() {
        let a = Size::new(0.5, 0.5);
        let b = Size::new(0.5, 0.5);
        assert_eq!(a, b);
    }

    #[test]
    fn size_inequality() {
        let a = Size::new(0.5, 0.5);
        let b = Size::new(0.5, 0.6);
        assert_ne!(a, b);
    }
}
