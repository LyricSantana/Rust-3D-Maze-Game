use macroquad::prelude::*;

use crate::map::SIZE;
use crate::player::Player;
use std::f32::consts::PI;

pub fn draw_first_person(player: &Player, map: [[u8; SIZE]; SIZE], cell_width: f32, cell_height: f32, view_x: f32, view_width: f32, endpoint: (usize, usize)) {
    draw_rectangle(view_x, 0.0, view_width, screen_height(), RED);
    draw_rectangle(view_x, screen_height() / 2.0, view_width, screen_height() / 2.0, YELLOW);

    let fov = 60.0 * PI / 180.0;
    let num_rays = 500;
    let column_width = view_width / num_rays as f32;
    let start_angle = player.facing() - fov / 2.0;
    let projection_plane = (view_width / 2.0) / (fov / 2.0).tan();

    for i in 0..num_rays {
        let ray_angle = start_angle + (i as f32 + 0.5) * fov / num_rays as f32;
        let (distance, hit_endpoint) = player.cast_ray_with_endpoint(ray_angle, map, cell_width, cell_height, endpoint);
        let corrected_distance = (distance * (ray_angle - player.facing()).cos()).max(0.1);
        let wall_height = (cell_height * projection_plane / corrected_distance).min(screen_height() * 2.0);
        let wall_y = (screen_height() - wall_height) / 2.0;
        let brightness = (1.0 - corrected_distance / 500.0).clamp(0.2, 1.0);
        let wall_color = if hit_endpoint {Color::new(0.0, 0.2, 1.0, 1.0)} else {Color::new(0.0, brightness, 0.0, 1.0)};

        draw_rectangle(view_x + i as f32 * column_width, wall_y, column_width, wall_height, wall_color);
    }
}