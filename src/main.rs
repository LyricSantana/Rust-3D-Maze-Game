use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Game".to_owned(),
        window_width: 512,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const MAP: [[u8; 25]; 25] = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,1,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,1,1,1,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,1,0,0,0,1,1,0,0,1,1,1,0,0,1,0,0,0,0,0,0,0,1],
        [1,0,1,1,1,0,1,0,0,1,0,1,0,0,0,1,1,1,0,0,0,0,0,0,1],
        [1,0,0,1,0,0,1,1,1,1,0,1,1,1,0,0,1,0,0,0,0,0,0,0,1],
        [1,0,0,1,0,0,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,1],
        [1,0,0,1,0,0,0,1,1,1,0,1,1,1,0,0,1,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    const MOVE_SPEED: f32 = 150.0;
    const TURN_SPEED: f32 = 2.5;
    
    let mut player_x: f32 = 50.0;
    let mut player_y: f32 = 50.0;
    let mut facing: f32 = 0.0;
    
    let cell_width: f32 = screen_width() / MAP.len() as f32;
    
    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);
        
        //draw map
        for (y, row) in MAP.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    draw_rectangle( x as f32 * cell_width, y as f32 * cell_width, cell_width, cell_width, GREEN);
                }
            }
        }
        
        draw_rectangle(player_x, player_y, 10.0, 10.0, RED);
        draw_line(player_x + 5.0, player_y + 5.0, player_x + 5.0 + (facing.cos() * 50.0), player_y + 5.0 + (facing.sin() * 50.0), 2.5, YELLOW);
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        } 
        if is_key_down(KeyCode::A) {
            player_x -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::D) {
            player_x += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::S) {
            player_y += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::W) {
            player_y -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            facing -= TURN_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            facing += TURN_SPEED * delta_time;
        }

        next_frame().await;
        
    }
}
