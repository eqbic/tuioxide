use tuioxide_macros::profile;

use crate::common::{container::Container, tuio_time::TuioTime};

pub struct Symbol {
    container: Container,
    symbol: SymbolProfile,
}

impl Symbol {
    pub fn new(start_time: &TuioTime, symbol: SymbolProfile) -> Self {
        let container = Container::new(start_time);
        Self { container, symbol }
    }

    pub fn update(&mut self, time: &TuioTime, symbol: &SymbolProfile) {
        self.container.update(time);
        self.symbol = symbol.clone();
    }
}

#[derive(Debug, Clone)]
#[profile("/tuio2/sym")]
pub struct SymbolProfile {
    session_id: i32,
    type_user_id: i32,
    component_id: i32,
    group: String,
    data: String,
}
