use rosc::OscBundle;

pub trait TuioManager {
    type TuioEntity;
    fn current_session_id(&self) -> i32;
    fn add(&mut self, entity: Self::TuioEntity);
    fn update(&mut self) -> &Vec<OscBundle>;
    fn remove(&mut self, entity: Self::TuioEntity);
}
