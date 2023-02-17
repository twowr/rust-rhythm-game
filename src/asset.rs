use crate::{frame::Frame, vector::Ivector};
use crossterm::style::Color;
use crate::vector::Uvector;
use rand::random;
pub struct Asset;
impl Asset {
    pub fn random_color_block(resolution: Uvector) -> Frame {
        let mut frame = Frame::init();
        frame.resolution = resolution.into();
        for _ in 0..(resolution.y * resolution.x) {
            frame.content.push(Color::Rgb { r: random(), g: random(), b: random() });
        }
        frame
    }
    pub fn receiver() -> Frame {
        Frame {
            resolution: Ivector { x: 5, y: 5 },
            content: vec!(
                Color::Black, Color::Grey, Color::Grey, Color::Grey, Color::Black,
                Color::Grey, Color::Grey, Color::Grey, Color::Grey, Color::Grey,
                Color::Grey, Color::Grey, Color::Grey, Color::Grey, Color::Grey,
                Color::Grey, Color::Grey, Color::Grey, Color::Grey, Color::Grey,
                Color::Black, Color::Grey, Color::Grey, Color::Grey, Color::Black,

            )
        }
    }
}