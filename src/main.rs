mod terminal;
mod screen;
mod frame;
mod vector;
mod asset;
use terminal::Terminal;
use screen::{
    Screen, ScreenElement
};
use vector::{ Uvector, Ivector };
use asset::Asset;
fn main() {
    let terminal= Terminal::init().unwrap();
    let mut test_screen = Screen::init();
    test_screen.resolution = terminal.size();
    test_screen.add(ScreenElement {
        position: (terminal.size() / 2).into(),
        origin: Ivector { x: 2, y: 2 },
        frame: Asset::receiver(),//Asset::random_color_block(Uvector { x: 5, y: 5 }),
        z_order: 0,
    });
    terminal.display(&test_screen.render(terminal.size())).unwrap();
    loop {}
}