mod terminal;
use terminal::Terminal;
use terminal::Frame;
use terminal::Size;
use crossterm::style::Color;

fn main() {
    let term = Terminal::init().unwrap();
    term.display(&Frame {
        size: Size { width: 3, height: 3 },
        content: vec![Color::Red, Color::Black, Color::Blue,
                      Color::Green, Color::Yellow, Color::White,
                      Color::Grey, Color::Magenta, Color::DarkGreen,
        ]
    }).unwrap();
}
