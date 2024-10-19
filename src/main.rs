use color_eyre::Result;
use macroquad::{audio::{self, PlaySoundParams}, prelude::*};
use player::Player;

extern crate rand;
use rand::Rng;

mod player;

// Util functions
fn rand_gold_position() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=3)
}

#[macroquad::main("rgold")]
async fn main() -> Result<()> {
    // Window
    set_fullscreen(false);
    let width = 800.;
    let height = 600.;
    request_new_screen_size(width, height);

    // Assets
    set_pc_assets_folder("resources");
    let font = load_ttf_font("fonts/OpenSans-Regular.ttf").await?;
    let gold_sound = audio::load_sound("sounds/gold.wav").await?;
    let music = audio::load_sound("sounds/music.wav").await?;
    let font_size = 20.0;

    // The colors
    let blue = Color::from_rgba(38, 139, 210, 255);
    let dark_green = Color::from_rgba(0, 43, 54, 255);
    let yellow = Color::from_rgba(181, 137, 0, 255);

    // Time and speed initial settings
    let mut last_update = get_time();
    let mut nav_lock = false;
    let speed = 0.1;
    let mut counter = get_time();
    let mut count: f64 = 10.0;

    // Player and gold initial settings
    let mut p = Player::new((width / 2.0) - 20.0, height / 2.0);
    let move_length = 20.0;
    let mut gold_position = rand_gold_position();

    let mut game_over = false;

    let music_params = PlaySoundParams {
        looped: true,
        volume: 1.0,
    };
    audio::play_sound(&music, music_params);

    loop {
        if !game_over {
            clear_background(dark_green);

            draw_text_ex(
                format!("SCORE: {}", p.score).as_str(),
                10.0,
                20.0,
                TextParams {
                    font_size: 20,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_text_ex(
                format!("STEPS: {}", p.steps).as_str(),
                (width / 2.0) - 60.0,
                20.0,
                TextParams {
                    font_size: 20,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_text_ex(
                format!("TIME REMAINING: {}", count).as_str(),
                width - 200.0,
                20.0,
                TextParams {
                    font_size: 20,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_line(1.0, 30.0, width - 1.0, 30.0, 5.0, blue);

            draw_rectangle_lines(40.0, 70.0, 200.0, 170.0, 10.0, blue);
            draw_rectangle_lines(40.0, 390.0, 200.0, 170.0, 10.0, blue);
            draw_rectangle_lines(
                (width / 2.0) + 160.0,
                70.0,
                200.0,
                170.0,
                10.0,
                blue,
            );
            draw_rectangle_lines(
                (width / 2.0) + 160.0,
                390.0,
                200.0,
                170.0,
                10.0,
                blue,
            );

            draw_text_ex(
                "X",
                p.x,
                p.y,
                TextParams {
                    font_size: 40,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            match gold_position {
                0 => draw_rectangle(120.0, 130.0, 40.0, 40.0, yellow), // left_top
                1 => draw_rectangle(120.0, 460.0, 40.0, 40.0, yellow), // left bottom
                2 => draw_rectangle(
                    (width / 2.0) + 240.0,
                    130.0,
                    40.0,
                    40.0,
                    yellow,
                ), // right top
                3 => draw_rectangle(
                    (width / 2.0) + 240.0,
                    460.0,
                    40.0,
                    40.0,
                    yellow,
                ), // right bottom
                _ => warn!(":D"),
            }

            if is_key_down(KeyCode::H) && p.x > 18.0 && !nav_lock {
                p.steps += 1;
                p.x -= move_length;
                nav_lock = true;
                audio::play_sound_once(&gold_sound);
                // gold_position = rand_gold_position();
            } else if is_key_down(KeyCode::L)
                && p.x < (width - 38.0)
                && !nav_lock
            {
                p.steps += 1;
                p.x += move_length;
                nav_lock = true;
                // gold_position = rand_gold_position();
            } else if is_key_down(KeyCode::J)
                && p.y < (height - 10.0)
                && !nav_lock
            {
                p.steps += 1;
                p.y += move_length;
                nav_lock = true;
                // gold_position = rand_gold_position();
            } else if is_key_down(KeyCode::K) && p.y > 60.0 && !nav_lock {
                p.steps += 1;
                p.y -= move_length;
                nav_lock = true;
                // gold_position = rand_gold_position();
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                nav_lock = false;
            }

            if get_time() - counter > 1.0 {
                counter = get_time();
                count -= 1.0;
                if count == 0.0 {
                    game_over = true;
                }
            }
        } else {
            audio::stop_sound(&music);
            let text_size =
                measure_text("GAME OVER", None, font_size as _, 1.0);
            draw_text(
                "GAME OVER",
                width / 2.0 - text_size.width,
                height / 2.0,
                50.0,
                DARKGRAY,
            );
        }

        next_frame().await
    }
}
