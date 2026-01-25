// use euclid::default::{Point2D, Vector2D};

// use crate::{
//     common::{constants::MILLI_PER_SECOND, tuio_state::TuioState, tuio_time::TuioTime},
//     tuio11::point::Point,
// };

// pub trait Translation {
//     fn position(&self) -> &Point2D<f32>;
//     fn velocity(&self) -> &Vector2D<f32>;
//     fn speed(&self) -> f32;
//     fn set_position(&mut self, position: Point2D<f32>);
//     fn set_velocity(&mut self, velocity: Vector2D<f32>);
//     fn set_acceleration(&mut self, acceleration: f32);
//     fn set_speed(&mut self, speed: f32);

//     fn should_calculate_translation(
//         &mut self,
//         position: Point2D<f32>,
//         velocity: Vector2D<f32>,
//     ) -> bool {
//         self.position().x != position.x
//             && velocity.x == 0.0
//             && self.position().y != position.y
//             && velocity.y == 0.0
//     }

//     fn update_translation(
//         &mut self,
//         current_time: TuioTime,
//         position: Point2D<f32>,
//         velocity: Vector2D<f32>,
//         acceleration: f32,
//     ) {
//         if self.should_calculate_translation(position, velocity) {
//             let dt = ((current_time - self.current_time()).get_total_milliseconds() as f32)
//                 / (MILLI_PER_SECOND as f32);
//             let delta_position = position - *self.position();
//             let distance = delta_position.length();
//             let last_motion_speed = self.speed();
//             if dt > 0.0 {
//                 self.set_velocity(delta_position / dt);
//                 self.set_speed(distance / dt);
//                 self.set_acceleration((self.speed() - last_motion_speed) / dt);
//             }
//         } else {
//             self.set_position(position);
//             self.set_velocity(velocity);
//             self.set_acceleration(acceleration);
//         }

//         self.set_position(position);
//         let state = if acceleration > 0.0 {
//             TuioState::Accelerating
//         } else if acceleration < 0.0 {
//             TuioState::Decelerating
//         } else {
//             TuioState::Stopped
//         };
//         self.set_state(state);
//     }
// }
