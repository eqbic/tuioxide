use tuioxide_macros::profile;

#[profile("/tuio2/tok")]
pub struct TokenProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    position_x: f32,
    position_y: f32,
    angle: f32,
    velocity_x: Option<f32>,
    velocity_y: Option<f32>,
    angle_speed: Option<f32>,
    acceleration: Option<f32>,
    rotation_acceleration: Option<f32>,
}
