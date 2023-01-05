use crate::frame::Frame;
use crossterm::style::Color;
use crate::vector::Uvector;
use rand::random;
pub struct Asset;
impl Asset {
    pub fn random_color_block(resolution: Uvector) -> Frame {
        let mut frame = Frame::init();
        frame.resolution = resolution;
        for _ in 0..(resolution.y * resolution.x) {
            frame.content.push(Color::Rgb { r: random(), g: random(), b: random() });
        }
        frame
    }
}