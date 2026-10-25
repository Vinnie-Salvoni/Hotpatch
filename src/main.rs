use macroquad::prelude::*;
use player::Player;

mod player;
#[macroquad::main("hotpatch")]
//seriously vinnie ffs, stop putting things here. you know async main needs to directly folow the macroquad::main attribute, not some other stuff. this is why your code doesn't compile.

async fn main() {
    //currently learning game loops, this seems fun
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(RED);
        player.update();
        player.draw();
        next_frame().await;
    }
}
