use macroquad::prelude::*;

const GRID_H: u8 = 220;
const GRID_W: u8 = 160;
const SCALE: f32 = 4.0;

fn win_conf() -> Conf {
    Conf {
        window_title: "Falling Sand".to_owned(),
        window_width: (GRID_W as f32 * SCALE) as i32,
        window_height: (GRID_H as f32 * SCALE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(win_conf)]
async fn main() {
    println!("Falling Sand!");
    loop {
        clear_background(BLACK);

        draw_text("Hello from Delta", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
