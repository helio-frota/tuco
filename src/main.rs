use color_eyre::Result;
use macroquad::prelude::*;
use player::Player;

extern crate rand;
use rand::Rng;

mod player;

fn rand_gold_position() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=3)
}

#[macroquad::main("rgold")]
async fn main() -> Result<()> {
    set_fullscreen(false);
    let width = 800.;
    let height = 600.;
    request_new_screen_size(width, height);

    let font = load_ttf_font("./resources/fonts/OpenSans-Regular.ttf").await?;

    let blue = Color::from_rgba(38, 139, 210, 255);
    let dark_green = Color::from_rgba(0, 43, 54, 255);
    let yellow = Color::from_rgba(181, 137, 0, 255);

    let mut player = Player::new((width / 2.0) - 20.0, height / 2.0);

    let move_length = 20.0;
    let seconds = 60;
    let mut gold_position = rand_gold_position();

    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut speed = 0.1;

    loop {
        clear_background(dark_green);

        draw_text_ex(
            format!("SCORE: {}", player.score).as_str(),
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
            format!("STEPS: {}", player.steps).as_str(),
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
            format!("TIME REMAINING: {}", seconds).as_str(),
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
            player.x,
            player.y,
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
            2 => {
                draw_rectangle((width / 2.0) + 240.0, 130.0, 40.0, 40.0, yellow)
            } // right top
            3 => {
                draw_rectangle((width / 2.0) + 240.0, 460.0, 40.0, 40.0, yellow)
            } // right bottom
            _ => println!(":D"),
        }

        if is_key_down(KeyCode::H) && player.x > 18.0 && !navigation_lock {
            player.steps += 1;
            player.x -= move_length;
            navigation_lock = true;
            // gold_position = rand_gold_position();
        } else if is_key_down(KeyCode::L)
            && player.x < (width - 38.0)
            && !navigation_lock
        {
            player.steps += 1;
            player.x += move_length;
            navigation_lock = true;
            // gold_position = rand_gold_position();
        } else if is_key_down(KeyCode::J)
            && player.y < (height - 10.0)
            && !navigation_lock
        {
            player.steps += 1;
            player.y += move_length;
            navigation_lock = true;
            // gold_position = rand_gold_position();
        } else if is_key_down(KeyCode::K) && player.y > 60.0 && !navigation_lock
        {
            player.steps += 1;
            player.y -= move_length;
            navigation_lock = true;
            // gold_position = rand_gold_position();
        }

        if get_time() - last_update > speed {
            last_update = get_time();
            navigation_lock = false;
        }

        next_frame().await
    }
}
