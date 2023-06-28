use crate::viewport::{self, Viewport};

pub mod animation;
pub mod click;
pub mod load;
pub mod message;
pub mod resize;

pub trait EventHandler<T> {
    fn handle_event(&mut self, viewport: &mut Viewport, event: T);
    fn create(func: dyn Fn(T)) -> Self;
}

pub struct EventHandles {}
