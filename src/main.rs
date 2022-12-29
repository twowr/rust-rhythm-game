mod terminal;
mod screen;
mod frame;
use terminal::Terminal;
use screen::{
    Screen, ScreenElement
};
use frame::Frame;
use terminal::Vector;
use crossterm::style::Color;
use rand::random;
fn main() {
    let term = Terminal::init().unwrap();
    let mut test_screen = Screen::init();
    test_screen.resolution = Vector::from(*term.size());
    test_screen.add(ScreenElement {
        position: *term.size() / 2,
        origin: Vector { x: 2, y: 2 },
        frame: Asset::random_color(Vector { x: 5, y: 5 }),
        z_order: 0,
    });
    term.display(&test_screen.render(term.size())).unwrap();
    loop {}
}
struct Asset;
impl Asset {
    fn random_color(resolution: Vector) -> Frame {
        let mut frame = Frame::init();
        frame.resolution = resolution;
        for _ in 0..(resolution.y * resolution.x) {
            frame.content.push(Color::Rgb { r: random(), g: random(), b: random() });
        }
        frame
    }
}