use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::map::SIZE;

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
            facing: (PI / 2.0),
            size: 10.0,
        }
    }
    pub fn facing(&self) -> f32 {
        self.facing
    }
    pub fn update(&mut self, delta_time: f32, map: &[[u8; SIZE]; SIZE]) {
        const MOVE_SPEED: f32 = 150.0;
        const TURN_SPEED: f32 = 4.0;

        let cell_width: f32 = screen_width() / 2.0 / map[0].len() as f32;
        let cell_height: f32 = screen_height() / map.len() as f32;

        let mut move_vec = (0.0, 0.0);

        // control movement based on facing direction
        if is_key_down(KeyCode::W) {move_vec.0 += self.facing.cos(); move_vec.1 += self.facing.sin(); }
        if is_key_down(KeyCode::S) {move_vec.0 -= self.facing.cos(); move_vec.1 -= self.facing.sin(); }
        if is_key_down(KeyCode::D) {move_vec.0 += (self.facing + PI / 2.0).cos(); move_vec.1 += (self.facing + PI / 2.0).sin(); }
        if is_key_down(KeyCode::A) {move_vec.0 -= (self.facing + PI / 2.0).cos(); move_vec.1 -= (self.facing + PI / 2.0).sin(); }

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
    fn collides_with_map(&self, map: &[[u8; SIZE]; SIZE], cell_w: f32, cell_h: f32) -> bool {
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

    pub fn cast_ray(&self, angle: f32, map: &[[u8; SIZE]; SIZE], cell_w: f32, cell_h: f32) -> (f32, f32, f32) {
        let start_x = self.x + (self.size / 2.0);
        let start_y = self.y + (self.size / 2.0);

        let cos_dir = angle.cos();
        let sin_dir = angle.sin();

        let step_size = 0.5;
        let mut distance = 0.0;

        // draw ray. is it hitting something? if no, step it forward by one step, if yes, return hit position. repeat
        loop {
            let current_x = start_x + cos_dir * distance;
            let current_y = start_y + sin_dir * distance;

            let grid_x = (current_x / cell_w).floor() as i32;
            let grid_y = (current_y / cell_h).floor() as i32;

            if grid_x >= 0 && grid_x < map[0].len() as i32 && grid_y >= 0  && grid_y < map.len() as i32 {
                if map[grid_y as usize][grid_x as usize] == 1 {
                    return (current_x, current_y, distance);
                }
            } else {    
                return (current_x, current_y, distance);
            }

            distance += step_size;
        }
    }

    pub fn draw(&self, map: &[[u8; SIZE]; SIZE], cell_w: f32, cell_h: f32) {
        let start_x = self.x + (self.size / 2.0);
        let start_y = self.y + (self.size / 2.0);

        let fov = 60.0 * (PI / 180.0);
        let num_rays = 60;
        let angle_step = fov / num_rays as f32;
        let start_angle = self.facing - (fov / 2.0);

        let max_distance = 500.0;
        // draw the rays
        for i in 0..num_rays {
            let ray_angle = start_angle + (i as f32 * angle_step);
            let (hit_x, hit_y, distance) = self.cast_ray(ray_angle, map, cell_w, cell_h);

            //change color of ray depending on distance
            let factor = (distance / max_distance).min(1.0);
            let ray_color = Color::from_rgba(255.0 as u8, (255.0 * (1.0 - factor)) as u8, 0.0 as u8, 255);

            draw_line(start_x, start_y, hit_x, hit_y, 2.5, ray_color);
        }
        
        draw_rectangle(self.x, self.y, self.size, self.size, RED);
    }
}