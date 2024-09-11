use winit::{event::Modifiers, keyboard::PhysicalKey};

#[derive(Debug, Default)]
pub struct TickState {
    pub events: Vec<TickEvent>,
    pub delta_time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickEvent {
    ModifiersChanged(Modifiers),
    KeyPressed(PhysicalKey, Option<char>),
    KeyReleased(PhysicalKey, Option<char>),
}

#[derive(Debug, Default)]
pub struct RenderState {
    pub width: u32,
    pub height: u32,
}
