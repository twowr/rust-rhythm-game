use crossterm::style::Color;
use crate::frame::Frame;
use crate::vector::{ Uvector, Ivector };
#[derive(Debug)]
pub struct ScreenElement {
    pub position: Ivector,
    pub origin: Ivector,
    pub frame: Frame,
    pub z_order: i8,
}
impl PartialEq for ScreenElement {
    fn eq(&self, other: &Self) -> bool {
        self.z_order == other.z_order
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Eq for ScreenElement {}
impl PartialOrd for ScreenElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.z_order.cmp(&other.z_order))
    }
}
impl Ord for ScreenElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z_order.cmp(&other.z_order)
    }
}
pub struct Screen {
    pub elements: Vec<ScreenElement>,
    pub resolution: Uvector,
}
impl Screen {
    pub fn init() -> Self {
        Self {
            elements: vec![],
            resolution: Uvector { x: 0, y: 0 },
        }
    }
    pub fn render(&mut self, resolution: Uvector) -> Frame {
        self.elements.sort();
        let mut frame = Frame::init();
        frame.resolution = resolution.into();
        for screen_element in self.elements.iter() {
            let offset:Uvector = (screen_element.position - screen_element.origin).into();
            let element_resolution: Uvector = screen_element.frame.resolution.abs().into();
            for y in 0..resolution.y {
                for x in 0..resolution.x {
                    if (y >= offset.y)
                    && (y <= offset.y + element_resolution.y.saturating_sub(1))
                    && (x >= offset.x)
                    && (x <= offset.x + element_resolution.x.saturating_sub(1))
                    {
                        let Uvector { x: source_x, y: source_y } = Uvector { x: x, y: y } - offset;
                        frame.content.push(screen_element.frame.content[source_y * element_resolution.x + source_x]);
                    } else {
                        frame.content.push(Color::Black);
                    }
                }
            }
        }
        frame

    }
    pub fn add(&mut self, element: ScreenElement) {
        self.elements.push(element);
    }
}