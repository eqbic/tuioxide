/// Represents the current state of a TUIO entity.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TuioState {
    /// The entity has just appeared and been added to the session.
    Added,
    /// The entity is moving and its acceleration is increasing.
    Accelerating,
    /// The entity is moving but its acceleration is decreasing.
    Decelerating,
    /// The entity is no longer moving.
    Stopped,
    /// The entity has disappeared and been removed from the session.
    Removed,
    /// The entity is currently rotating.
    Rotating,
    /// The entity is present but has not changed since the last frame.
    Idle,
}