use crate::core::tuio20::{bounds::Bounds, pointer::Pointer, symbol::Symbol, token::Token};

#[derive(Debug)]
pub enum PointerEvent {
    Add(Pointer),
    Update(Pointer),
    Remove(Pointer),
}

#[derive(Debug)]
pub enum TokenEvent {
    Add(Token),
    Update(Token),
    Remove(Token),
}

#[derive(Debug)]
pub enum BoundsEvent {
    Add(Bounds),
    Update(Bounds),
    Remove(Bounds),
}

#[derive(Debug)]
pub enum SymbolEvent {
    Add(Symbol),
    Update(Symbol),
    Remove(Symbol),
}
