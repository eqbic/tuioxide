use crate::tuio20::{Bounds, Pointer, Symbol, Token, bundle::Frame};

/// A collection of TUIO 2.0 events produced from a single OSC bundle frame.
///
/// Each field holds the events generated for its respective entity type during
/// the processing of one frame. An event is emitted for every entity that was
/// added, updated, or removed since the previous frame.
#[derive(Debug, Default)]
pub struct TuioEvents {
    /// Events for [`Pointer`] entities (`/tuio2/ptr`).
    ///
    /// Each element is a [`PointerEvent::Add`], [`PointerEvent::Update`], or
    /// [`PointerEvent::Remove`] variant depending on whether the pointer appeared,
    /// moved, or disappeared in this frame.
    pub pointer_events: Vec<PointerEvent>,

    /// Events for [`Token`] entities (`/tuio2/tok`).
    ///
    /// Each element is a [`TokenEvent::Add`], [`TokenEvent::Update`], or
    /// [`TokenEvent::Remove`] variant.
    pub token_events: Vec<TokenEvent>,

    /// Events for [`Bounds`] entities (`/tuio2/bnd`).
    ///
    /// Each element is a [`BoundsEvent::Add`], [`BoundsEvent::Update`], or
    /// [`BoundsEvent::Remove`] variant.
    pub bounds_events: Vec<BoundsEvent>,

    /// Events for [`Symbol`] entities (`/tuio2/sym`).
    ///
    /// Each element is a [`SymbolEvent::Add`], [`SymbolEvent::Update`], or
    /// [`SymbolEvent::Remove`] variant.
    pub symbol_events: Vec<SymbolEvent>,

    /// Event for the [`Frame`] itself, including the frame's timestamp and any
    /// other frame-level metadata.
    pub frame_event: Frame,
}

/// An event describing a change in the lifecycle of a TUIO 2.0 [`Pointer`].
///
/// Pointer events are emitted by the TUIO 2.0 client processor each time a
/// frame is received and the set of active pointers changes.
#[derive(Debug)]
pub enum PointerEvent {
    /// A new [`Pointer`] has appeared and been added to the active session.
    Add(Pointer),
    /// An existing [`Pointer`] has moved or changed state.
    Update(Pointer),
    /// A previously active [`Pointer`] is no longer present and has been removed.
    Remove(Pointer),
}

/// An event describing a change in the lifecycle of a TUIO 2.0 [`Token`].
///
/// Token events are emitted by the TUIO 2.0 client processor each time a
/// frame is received and the set of active tokens changes.
#[derive(Debug)]
pub enum TokenEvent {
    /// A new [`Token`] has appeared and been added to the active session.
    Add(Token),
    /// An existing [`Token`] has moved or changed state.
    Update(Token),
    /// A previously active [`Token`] is no longer present and has been removed.
    Remove(Token),
}

/// An event describing a change in the lifecycle of a TUIO 2.0 [`Bounds`].
///
/// Bounds events are emitted by the TUIO 2.0 client processor each time a
/// frame is received and the set of active bounding regions changes.
#[derive(Debug)]
pub enum BoundsEvent {
    /// A new [`Bounds`] region has appeared and been added to the active session.
    Add(Bounds),
    /// An existing [`Bounds`] region has moved or changed state.
    Update(Bounds),
    /// A previously active [`Bounds`] region is no longer present and has been removed.
    Remove(Bounds),
}

/// An event describing a change in the lifecycle of a TUIO 2.0 [`Symbol`].
///
/// Symbol events are emitted by the TUIO 2.0 client processor each time a
/// frame is received and the set of active symbols changes.
#[derive(Debug)]
pub enum SymbolEvent {
    /// A new [`Symbol`] has appeared and been added to the active session.
    Add(Symbol),
    /// An existing [`Symbol`] has changed its data or state.
    Update(Symbol),
    /// A previously active [`Symbol`] is no longer present and has been removed.
    Remove(Symbol),
}
