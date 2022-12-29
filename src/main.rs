mod terminal;
mod screen;
mod frame;
use terminal::Terminal;
use frame::Frame;
use terminal::Vector;
use crossterm::style::Color;
use rand::random;
fn main() {
    let term = Terminal::init().unwrap();
    let mut content:Vec<Color> = vec![];
    for _ in 0..(term.size().y * term.size().x) {
        content.append(&mut vec![Color::Rgb { r: random(), g: random(), b: random() }]);
    }
    term.display(Frame {
        resolution: *term.size(),
        content: content,
    }).unwrap();
}
