use std::io::stdout;
use std::ops::Mul;
use crate::frame::Frame;
use num_traits::cast::ToPrimitive;
use crossterm::terminal::enable_raw_mode;
use crossterm::style::{
    SetBackgroundColor,
    ResetColor,
    Print,
};
use crossterm::execute;
#[derive(PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl<T: ToPrimitive> Mul<T> for Size{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_f32().unwrap();
        let width = self.width as f32;
        let height = self.height as f32;
        Self{
            width: (width * rhs) as usize,
            height: (height * rhs) as usize,
        }
    }
}
impl Mul<Self> for Size {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            width: self.width * rhs.width,
            height: self.height * rhs.height,
        }
    }
}
pub struct Terminal {
    size: Size
}
impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn init() -> std::io::Result<Self> {
        enable_raw_mode().unwrap();
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Size{
                width: size.0 as usize,
                height: size.1 as usize,
            }
        })
    }
    pub fn display(&self, frame: Frame) -> crossterm::Result<()> {
        // haven't handled when frame size is different from terminal size
        let frame_size = &frame.resolution;
        if frame_size == &self.size {
            for y in 0..frame_size.height {
                for x in 0..frame_size.width {
                    execute!(stdout(),
                             SetBackgroundColor(*frame.content.get(y*frame.resolution.width + x).unwrap()),
                             Print(" "),
                             ResetColor,
                    ).unwrap();
                }
            }
        }
        Ok(())
    }
}