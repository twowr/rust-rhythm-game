use crate::Size;
use crossterm::style::Color;
#[derive(PartialEq)]
pub struct Frame {
    pub size: Size,
    pub content: Vec<Color>,
}