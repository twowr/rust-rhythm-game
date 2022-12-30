use crossterm::style::Color;

use crate::frame::Frame;
use crate::Vector;
#[derive(Debug)]
pub struct ScreenElement {
    pub position: Vector,
    pub origin: Vector,
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
    pub resolution: Vector,
}
impl Screen {
    pub fn init() -> Self {
        Self {
            elements: vec![],
            resolution: Vector { x: 0, y: 0 },
        }
    }
    pub fn render(&mut self, resolution: &Vector) -> Frame {
        self.elements.sort();
        let mut frame = Frame::init();
        frame.resolution = *resolution;
        for screen_element in self.elements.iter() {
            let offset = screen_element.position - screen_element.origin;
            let element_resolution = screen_element.frame.resolution;
            for y in 0..resolution.y {
                for x in 0..resolution.x {
                    if (y >= offset.y)
                    && (y <= offset.y + element_resolution.y)
                    && (x >= offset.x)
                    && (x <= offset.x + element_resolution.x)
                    {
                        let Vector { x: source_x, y: source_y } = Vector { x, y } - offset;
                        frame.content.push(screen_element.frame.content[source_y * element_resolution.x + source_x]);
                        print!("{:?}", frame.content);
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