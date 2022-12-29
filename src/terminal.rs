use std::io::stdout;
use std::ops::{ Mul, Add, Sub };
use crate::frame::Frame;
use num_traits::cast::ToPrimitive;
use crossterm::terminal::enable_raw_mode;
use crossterm::style::{
    SetBackgroundColor,
    ResetColor,
    Print,
};
use crossterm::execute;
#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}
impl<T: ToPrimitive> Mul<T> for Vector{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_f32().unwrap();
        let x = self.x as f32;
        let y = self.y as f32;
        Self{
            x: (x * rhs) as usize,
            y: (y * rhs) as usize,
        }
    }
}
impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
pub struct Terminal {
    size: Vector
}
impl Terminal {
    pub fn size(&self) -> &Vector {
        &self.size
    }
    pub fn init() -> std::io::Result<Self> {
        enable_raw_mode().unwrap();
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Vector{
                x: size.0 as usize,
                y: size.1 as usize,
            }
        })
    }
    pub fn display(&self, frame: Frame) -> crossterm::Result<()> {
        // haven't handled when frame size is different from terminal size
        let frame_size = &frame.resolution;
        if frame_size == &self.size {
            for y in 0..frame_size.y {
                for x in 0..frame_size.x {
                    execute!(stdout(),
                             SetBackgroundColor(*frame.content.get(y*frame.resolution.x + x).unwrap()),
                             Print(" "),
                             ResetColor,
                    ).unwrap();
                }
                println!("\r");
            }
        }
        Ok(())
    }
}