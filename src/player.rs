use macroquad::prelude::*;
use std::f64::consts::PI;

pub struct Player {
    x: f32,
    y: f32,
    facing: f32,
    size: f32,
}

impl Player {
    pub fn new(x:f32, y:f32) -> Self {
        Self {
            x,
            y,
            facing: (PI / 2.0) as f32,
            size: 10.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, map: &[[u8; 25]; 25]) {
        const MOVE_SPEED: f32 = 150.0;
        const TURN_SPEED: f32 = 2.5;

        let cell_width: f32 = screen_width() / map[0].len() as f32;
        let cell_height: f32 = screen_height() / map.len() as f32;

        let mut move_vec = (0.0, 0.0);

        // control movement based on facing direction
        if is_key_down(KeyCode::W) {move_vec.0 += self.facing.cos(); move_vec.1 += self.facing.sin(); }
        if is_key_down(KeyCode::S) {move_vec.0 -= self.facing.cos(); move_vec.1 -= self.facing.sin(); }
        if is_key_down(KeyCode::D) {move_vec.0 += (self.facing + PI as f32 / 2.0).cos(); move_vec.1 += (self.facing + PI as f32 / 2.0).sin(); }
        if is_key_down(KeyCode::A) {move_vec.0 -= (self.facing + PI as f32 / 2.0).cos(); move_vec.1 -= (self.facing + PI as f32 / 2.0).sin(); }

        let (mx, my): (f32, f32) = move_vec;
        let length = (mx * mx + my * my).sqrt();

        if length > 0.0 {
            let old_x = self.x;
            self.x += (mx / length) * MOVE_SPEED * delta_time;
            if self.collides_with_map(map, cell_width, cell_height) { // detect collision
                self.x = old_x;
            }
            let old_y = self.y;
            self.y += (my / length) * MOVE_SPEED * delta_time;
            if self.collides_with_map(map, cell_width, cell_height) {
                self.y = old_y;
            }
        }

        let mut turn_dir = 0.0;
        if is_key_down(KeyCode::Left) {turn_dir -= 1.0; }
        if is_key_down(KeyCode::Right) {turn_dir += 1.0; }
        self.facing += turn_dir * TURN_SPEED * delta_time;
    }

    // ew gross yucky function ewww
    fn collides_with_map(&self, map: &[[u8; 25]; 25], cell_w: f32, cell_h: f32) -> bool {
        // map coordinates down to grid cell indexes
        let start_x = (self.x / cell_w).floor() as i32;
        let end_x = ((self.x + self.size) / cell_w).floor() as i32;
        let start_y = (self.y / cell_h).floor() as i32;
        let end_y = ((self.y + self.size) / cell_h).floor() as i32;

        //iterate through all cells covered by the players box area
        for grid_y in start_y..=end_y {
            for grid_x in start_x..=end_x {
                //check if player is out of bounds
                if grid_x >= 0 && grid_x < map[0].len() as i32 && grid_y >= 0  && grid_y < map.len() as i32 {
                    if map[grid_y as usize][grid_x as usize] == 1 {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
    false
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 10.0, 10.0, RED);
        draw_line(
        self.x + (self.size / 2.0), 
        self.y + (self.size / 2.0), 
        self.x + (self.size / 2.0) + (self.facing.cos() * 50.0), 
        self.y + (self.size / 2.0) + (self.facing.sin() * 50.0), 
        2.5, 
        YELLOW
        );
    }
}