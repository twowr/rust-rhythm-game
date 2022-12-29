use crate::Vector;
use num_traits::cast::ToPrimitive;
use std::ops::Mul;
use crossterm::style::Color;
pub struct Frame {
    pub resolution: Vector,
    pub content: Vec<Color>,
}
impl<T: ToPrimitive> Mul<T> for Frame{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.to_f32().unwrap();
        let new_resolution = self.resolution * rhs;
        let Vector { x, y } = new_resolution;
        let source_content = self.content;
        let mut new_content:Vec<Color> = vec![];
        for new_index in 0..(y * x) {
            let source_index = (new_index as f32 / rhs).floor() as usize;
            new_content[new_index] = source_content[source_index];
        }
        Self {
            resolution: new_resolution,
            content: new_content,
        }
    }
}
impl Mul<Vector> for Frame {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self::Output {
        let new_resolution = self.resolution * rhs;
        let Vector { x: new_x, y: new_y } = new_resolution;
        let source_content = self.content;
        let source_resolution = self.resolution;
        let mut new_content:Vec<Color> = vec![];
        for y in 0..new_y {
            for x in 0..new_x {
                let new_index = y * new_y + x;
                let source_index = y * (source_resolution.y / new_y) * source_resolution.x + x * (source_resolution.x / new_x);
                new_content[new_index] = source_content[source_index];
            }
        }
        Self {
            resolution: new_resolution,
            content: new_content,
        }

    }
}
impl Frame {
    pub fn init() -> Frame {
        Frame {
            resolution: Vector {
                x: 0,
                y: 0,
            },
            content: vec![],
        }
    }
}