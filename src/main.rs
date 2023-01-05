mod terminal;
mod screen;
mod frame;
mod vector;
use terminal::Terminal;
use screen::{
    Screen, ScreenElement
};
use frame::Frame;
use vector::{ Uvector, Ivector };
use crossterm::style::Color;
use rand::random;
fn main() {
    let term = Terminal::init().unwrap();
    let mut test_screen = Screen::init();
    test_screen.resolution = *term.size();
    test_screen.add(ScreenElement {
        position: (*term.size() / 2).into(),
        origin: Ivector { x: 2, y: 2 },
        frame: Asset::random_color(Uvector { x: 5, y: 5 }),
        z_order: 0,
    });
    term.display(&test_screen.render(term.size())).unwrap();
    loop {}
}
struct Asset;
impl Asset {
    fn random_color(resolution: Uvector) -> Frame {
        let mut frame = Frame::init();
        frame.resolution = resolution;
        for _ in 0..(resolution.y * resolution.x) {
            frame.content.push(Color::Rgb { r: random(), g: random(), b: random() });
        }
        frame
    }
}