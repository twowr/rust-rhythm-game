use crate::frame::Frame;
use crate::Size;
pub struct Position {
    x: u16,
    y: u16,
}
pub struct ScreenElement {
    pub position: Position,
    pub origin: Position,
    pub frame: Frame,
    pub z_order: i8,
}
pub struct Screen {
    pub element: Vec<ScreenElement>,
    pub resolution: Size,
}
impl Screen {
    pub fn render(&self, resolution: Size) -> Frame {
        
    }
}