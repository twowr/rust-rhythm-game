mod terminal;
mod screen;
mod frame;
mod vector;
mod assest;
use terminal::Terminal;
use screen::{
    Screen, ScreenElement
};
use vector::{ Uvector, Ivector };
use assest::Asset;
fn main() {
    let term = Terminal::init().unwrap();
    let mut test_screen = Screen::init();
    test_screen.resolution = *term.size();
    test_screen.add(ScreenElement {
        position: (*term.size() / 2).into(),
        origin: Ivector { x: 2, y: 2 },
        frame: Asset::random_color_block(Uvector { x: 5, y: 5 }),
        z_order: 0,
    });
    term.display(&test_screen.render(term.size())).unwrap();
    loop {}
}