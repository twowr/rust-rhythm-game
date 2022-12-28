use crate::Size;
use num_traits::cast::ToPrimitive;
use std::ops::Mul;
use crossterm::style::Color;
pub struct Frame {
    pub resolution: Size,
    pub content: Vec<Color>,
}
impl<T: ToPrimitive> Mul<T> for Frame{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.to_f32().unwrap();
        let new_resolution = self.resolution * rhs;
        let Size { width, height } = new_resolution;
        let source_content = self.content;
        let mut new_content:Vec<Color> = vec![];
        for new_index in 0..(height * width) {
            let source_index = (new_index as f32 / rhs).floor() as usize;
            new_content[new_index] = source_content[source_index];
        }
        Self {
            resolution: new_resolution,
            content: new_content,
        }
    }
}
impl Mul<Size> for Frame {
    type Output = Self;

    fn mul(self, rhs: Size) -> Self::Output {
        let new_resolution = self.resolution * rhs;
        let Size { width: new_width, height: new_height } = new_resolution;
        let source_content = self.content;
        let source_resolution = self.resolution;
        let mut new_content:Vec<Color> = vec![];
        for y in 0..new_height {
            for x in 0..new_width {
                let new_index = y * new_height + x;
                let source_index = y * (source_resolution.height / new_height) * source_resolution.width + x * (source_resolution.width / new_width);
                new_content[new_index] = source_content[source_index];
            }
        }
        Self {
            resolution: new_resolution,
            content: new_content,
        }

    }
}