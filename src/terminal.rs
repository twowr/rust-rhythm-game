use std::io::stdout;
use crossterm::terminal::enable_raw_mode;
use crossterm::style::{
    SetBackgroundColor,
    Color, ResetColor,
    Print,
};
use crossterm::execute;
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
    fn ne(&self, other: &Self) -> bool {
        self.width != other.width || self.height != other.height
    }
}
pub struct frame {
    pub size: Size,
    pub content: Vec<Color>,
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
    pub fn display(&self, frame: &frame) -> crossterm::Result<()> {
        // haven't handled when frame size is different from terminal size
        let frame_size = &frame.size;
        if frame_size == &self.size {
            for y in 0..frame_size.height {
                for x in 0..frame_size.width {
                    execute!(stdout(),
                             SetBackgroundColor(*frame.content.get(y*frame.size.width + x).unwrap()),
                             Print(" "),
                             ResetColor,
                    ).unwrap();
                }
            }
        }
        Ok(())
    }
}