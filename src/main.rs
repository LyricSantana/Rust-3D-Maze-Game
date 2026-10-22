mod map;
mod player;
mod render;

use macroquad::prelude::*;
use map::{generate_maze, SIZE};
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
    let map = generate_maze();

    let map_veiw_width = screen_width() / 2.0;
    let cell_width = screen_width() / 2.0 / SIZE as f32;
    let cell_height = screen_height() / SIZE as f32;
    
    let player_size = 10.0;
    let player_x = cell_width + (cell_width - player_size) / 2.0;
    let player_y = cell_height + (cell_height - player_size) / 2.0;
    let mut player = Player::new(player_x, player_y);

    
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        } 
        
        let delta_time = get_frame_time();
        player.update(delta_time, &map);

        clear_background(BLACK);
        
        // Draw map
        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    draw_rectangle( x as f32 * cell_width, y as f32 * cell_height, cell_width, cell_height, GREEN);
                }
            }
        }
        draw_first_person(&player, &map, cell_width, cell_height, map_veiw_width, 512.0);

        //Draw player
        player.draw(&map, cell_width, cell_height);
        next_frame().await;
    }
}
