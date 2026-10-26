/*
Made by Lyric Santana October 2026 for week 1 of Hack Club's Terra program
*/
mod map;
mod player;
mod render;

use macroquad::prelude::*;
use map::{generate_maze, SIZE, find_endpoint};
use player::Player;
use render::draw_first_person;

enum Gamestate {
    Start,
    Playing, 
    Won
}

fn window_conf() -> Conf {
    // let icon: [u8; 1024] = [255; 1024];
    Conf {
        window_title: "Maze Game".to_owned(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        //was gonna add an icon but found out it was more complex than i though 
        // icon: Some(macroquad::miniquad::conf::Icon {
        //     small: icon,
        //     medium: [255; 4096],
        //     big: [255; 16384],
        // }),
        ..Default::default()
    }
}

// stupid boring gross redundant code that I hated writing
fn draw_start_screen() {
    clear_background(BLACK);

    let title = "Maze Game";
    let subtitle = "Find the Blue Exit";
    let controls = "WASD to Move | Arrow Keys to Turn | M to Toggle Map";
    let prompt = "Press ENTER to Start";

    let title_size = 64.0;
    let subtitle_size = 24.0;
    let controls_size = 24.0;
    let prompt_size = 24.0;
    
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;
    let controls_width = measure_text(controls, None, controls_size as u16, 1.0).width;
    let prompt_width = measure_text(prompt, None, prompt_size as u16, 1.0).width;
    
    draw_text(title, (screen_width() - title_width) / 2.0, screen_height() * 0.3, title_size, GREEN);
    draw_text(subtitle, (screen_width() - subtitle_width) / 2.0, screen_height() * 0.4, subtitle_size, WHITE);
    draw_text(controls, (screen_width() - controls_width) / 2.0, screen_height() * 0.6, controls_size, WHITE);
    draw_text(prompt, (screen_width() - prompt_width) / 2.0, screen_height() * 0.8, prompt_size, WHITE);
}

fn draw_win_screen(elapsed_seconds: f64) {
    clear_background(BLACK);

    let title = "You Escaped!";
    let subtitle = "You Found Your Way Out of the Maze";
    let prompt = "Press ENTER to Play Again";
    let quit = "Press ESCAPE to quit";
    // took me way too long to figure out you have to use "format!"
    let time = format!("Your Time: {:.1} seconds!", elapsed_seconds);

    let title_size = 64.0;
    let subtitle_size = 24.0;
    let prompt_size = 24.0;
    let quit_size = 24.0;
    let time_size = 24.0;

    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;
    let prompt_width = measure_text(prompt, None, prompt_size as u16, 1.0).width;
    let quit_width = measure_text(quit, None, quit_size as u16, 1.0).width;
    let time_width = measure_text(&time, None, time_size as u16, 1.0).width;

    draw_text(title, (screen_width() - title_width) / 2.0, screen_height() * 0.3, title_size, GREEN);
    draw_text(subtitle, (screen_width() - subtitle_width) / 2.0, screen_height() * 0.4, subtitle_size, WHITE);
    draw_text(prompt, (screen_width() - prompt_width) / 2.0, screen_height() * 0.6, prompt_size, WHITE);
    draw_text(quit, (screen_width() - quit_width) / 2.0, screen_height() * 0.8, quit_size, WHITE);
    draw_text(time, (screen_width() - time_width) / 2.0, screen_height() * 0.9, time_size, WHITE);


}

#[macroquad::main(window_conf)]
async fn main() {
    let mut map = generate_maze();
    let mut endpoint = find_endpoint(&map);

    let map_view_width = screen_width() / 2.0;
    let cell_width = screen_width() / 2.0 / SIZE as f32;
    let cell_height = screen_height() / SIZE as f32;
    
    let player_size = 5.0;
    let player_x = cell_width + (cell_width - player_size) / 2.0;
    let player_y = cell_height + (cell_height - player_size) / 2.0;
    let mut player = Player::new(player_x, player_y);

    let mut state = Gamestate::Start;

    let mut show_map = false;
    
    let mut game_start_time = 0.0;
    let mut elapsed_seconds = 0.0;

    //main loop
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        } 

        match state {
            Gamestate::Start => {
                if is_key_pressed(KeyCode::Enter) {
                    game_start_time = get_time();
                    elapsed_seconds = 0.0;
                    show_map = false;
                    state = Gamestate::Playing;
                }
                draw_start_screen();
            }

            Gamestate::Playing => {
                if is_key_pressed(KeyCode::M) {
                    show_map = !show_map;
                }
                let delta_time = get_frame_time();
                player.update(delta_time, &map);

                elapsed_seconds = get_time() - game_start_time;

                if player.reached_endpoint(endpoint, cell_width, cell_height) {
                    state = Gamestate::Won; 
                } else {
                    clear_background(BLACK);
                
                // Draw map
                if show_map {

                    for (y, row) in map.iter().enumerate() {
                        for (x, &cell) in row.iter().enumerate() {
                            if cell == 1 {
                                draw_rectangle( x as f32 * cell_width, y as f32 * cell_height, cell_width, cell_height, GREEN);
                            } 
                        }
                    }
                }
                draw_rectangle(endpoint.0 as f32 * cell_width, endpoint.1 as f32 * cell_height, cell_width, cell_height, BLUE);
                draw_first_person(&player, map,cell_width,cell_height,map_view_width,screen_width() - map_view_width,endpoint);
                
                let timer_text = format!("Time: {:.1}s", elapsed_seconds);

                draw_text(timer_text, 12.0, 28.0, 22.0, WHITE);

                //Draw player
                player.draw(&map, cell_width, cell_height);
            }
        }
        Gamestate::Won => {
            draw_win_screen(elapsed_seconds);
            if is_key_pressed(KeyCode::Enter) {
                map = generate_maze();
                endpoint = find_endpoint(&map);
                player = Player::new(player_x, player_y);
                state = Gamestate::Start;
            }
        }
    }
    next_frame().await;
    }
}
