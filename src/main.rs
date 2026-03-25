use macroquad::prelude::*;

#[macroquad::main("Tutorial Macroquad")]
async fn main() {
    loop {
        clear_background(DARKBLUE);
        next_frame().await;
    }
}
