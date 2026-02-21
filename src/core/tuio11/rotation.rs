// use crate::{
//     common::{constants::MILLI_PER_SECOND, tuio_time::TuioTime},
//     tuio11::point::Point,
// };

// pub trait Rotation: Point {
//     fn angle(&self) -> f32;
//     fn set_angle(&mut self, angle: f32);
//     fn rotation_speed(&self) -> f32;
//     fn set_rotation_speed(&mut self, rotation_speed: f32);
//     fn rotation_acceleration(&self) -> f32;
//     fn set_rotation_acceleration(&mut self, rotation_acceleration: f32);

//     fn should_calculate_rotation(&self, angle: f32, rotation_speed: f32) -> bool {
//         self.angle() != angle && rotation_speed == 0.0
//     }

//     fn update_rotation(
//         &mut self,
//         current_time: TuioTime,
//         angle: f32,
//         rotation_speed: f32,
//         rotation_acceleration: f32,
//     ) {
//         if self.should_calculate_rotation(angle, rotation_speed) {
//             let dt = (current_time - self.current_time()).get_total_milliseconds() as f32
//                 / (MILLI_PER_SECOND as f32);
//             if dt > 0.0 {
//                 let last_angle = self.angle();
//                 let last_rotation_speed = self.rotation_speed();
//                 let mut da = (angle - last_angle) / (2.0 * std::f32::consts::PI);
//                 if da > 0.5 {
//                     da -= 1.0;
//                 } else if da <= -0.5 {
//                     da += 1.0;
//                 }

//                 self.set_rotation_speed(da / dt);
//                 self.set_rotation_acceleration((self.rotation_speed() - last_rotation_speed) / dt);
//             }
//         } else {
//             self.set_rotation_speed(rotation_speed);
//             self.set_rotation_acceleration(rotation_acceleration);
//         }

//         self.set_angle(angle);
//     }
// }
