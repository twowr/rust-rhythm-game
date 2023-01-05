use std::io::stdout;
use crate::frame::Frame;
use crate::vector::Uvector;
use crossterm::terminal::enable_raw_mode;
use crossterm::style::{
    SetBackgroundColor,
    ResetColor,
    Print,
};
use crossterm::execute;
pub struct Terminal {
    size: Uvector
}
impl Terminal {
    pub fn size(&self) -> &Uvector {
        &self.size
    }
    pub fn init() -> std::io::Result<Self> {
        enable_raw_mode().unwrap();
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Uvector{
                x: size.0 as usize,
                y: size.1 as usize,
            }
        })
    }
    pub fn display(&self, frame: &Frame) -> crossterm::Result<()> {
        // haven't handled when frame size is different from terminal size
        let frame_size = &frame.resolution;
        if frame_size == &self.size {
            for y in 0..frame_size.y {
                for x in 0..frame_size.x {
                    execute!(stdout(),
                             SetBackgroundColor(*frame.content.get(y * frame.resolution.x + x).unwrap()),
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