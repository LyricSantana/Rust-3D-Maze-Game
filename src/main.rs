mod map;
mod player;
mod render;

use macroquad::prelude::*;
use map::MAP;
use player::Player;
use render::draw_first_person;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Game".to_owned(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cell_width: f32 = screen_width() / 2.0 / MAP[0].len() as f32;
    let cell_height: f32 = screen_height() / MAP.len() as f32;
    
    let mut player = Player::new(50.0, 50.0);
    
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        } 
        
        let delta_time = get_frame_time();
        player.update(delta_time, &MAP);

        clear_background(BLACK);
        
        // Draw map
        for (y, row) in MAP.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    draw_rectangle( x as f32 * cell_width, y as f32 * cell_height, cell_width, cell_height, GREEN);
                }
            }
        }
        
        draw_first_person(&player, &MAP, cell_width, cell_height, 512.0, 512.0);

        //Draw player
        player.draw(&MAP, cell_width, cell_height);
        next_frame().await;
    }
}
