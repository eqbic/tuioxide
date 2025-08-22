#[derive(Debug, Clone, PartialEq)]
pub enum TuioState {
    Added,
    Accelerating,
    Decelerating,
    Stopped,
    Removed,
    Rotating,
    Idle,
}
