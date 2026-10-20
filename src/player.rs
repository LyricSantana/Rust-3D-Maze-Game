use macroquad::prelude::*;

pub struct Player {
    x: f32,
    y: f32,
    facing: f32,
}

impl Player {
    pub fn new(x:f32, y:f32) -> Self {
        Self {
            x,
            y,
            facing: 0.0,
        }
    }

    pub fn update(&mut self, delta_time:f32) {
        const MOVE_SPEED: f32 = 150.0;
        const TURN_SPEED: f32 = 2.5;

        let mut move_vec = (0.0, 0.0);

        if is_key_down(KeyCode::A) {
            move_vec.0 -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_vec.0 += 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_vec.1 += 1.0;
        }
        if is_key_down(KeyCode::W) {
            move_vec.1 -= 1.0;
        }

        let (mx, my): (f32, f32) = move_vec;
        let length = (mx * mx + my * my).sqrt();

        if length > 0.0 {
            self.x += (mx / length) * MOVE_SPEED * delta_time;
            self.y += (my / length) * MOVE_SPEED * delta_time;
        }

        let mut turn_dir = 0.0;

        if is_key_down(KeyCode::Left) {
            turn_dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            turn_dir += 1.0;
        }
        self.facing += turn_dir * TURN_SPEED * delta_time;

    }
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 10.0, 10.0, RED);
        draw_line(self.x + 5.0, self.y + 5.0, self.x + 5.0 + (self.facing.cos() * 50.0), self.y + 5.0 + (self.facing.sin() * 50.0), 2.5, YELLOW);
    }
}