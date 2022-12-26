use crate::Size;
use crossterm::style::Color;
pub struct Frame {
    pub resolution: Size,
    pub content: Vec<Color>,
}
impl std::ops::Mul for Frame {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        
    }
}