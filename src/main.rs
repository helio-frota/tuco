use anyhow::Result;
use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    score: i16,
    steps: i16,
}

impl Player {
    fn new(x: f32, y: f32) -> Player {
        Player {
            x,
            y,
            score: 0,
            steps: 0,
        }
    }
}

const LEFT_TOP: u8 = 0;
const LEFT_BOTTOM: u8 = 1;
const RIGHT_TOP: u8 = 2;
const RIGHT_BOTTOM: u8 = 3;

fn rand_gold_position() -> u8 {
    rand::gen_range(0, 3)
}

fn basic_collision(
    x: f32,
    y: f32,
    gold_x: f32,
    gold_y: f32,
    width: f32,
    height: f32,
) -> bool {
    if x + width < gold_x
        || x > gold_x + width
        || y + height < gold_y
        || y > gold_y + height
    {
        return true;
    }
    false
}

fn went_to_gold_location(p: &Player, width: f32, height: f32) -> bool {
    basic_collision(p.x, p.y, width - 850., height - 550., 40., 40.)
        || basic_collision(p.x, p.y, width - 850., height - 200., 40., 40.)
        || basic_collision(p.x, p.y, width - 150., height - 550., 40., 40.)
        || basic_collision(p.x, p.y, width - 150., height - 200., 40., 40.)
}

fn gold_found(p: &Player, gold_position: u8) -> bool {
    (p.x == 140. && p.y == 160.0 && gold_position == LEFT_TOP)
        || (p.x == 120. && p.y == 500. && gold_position == LEFT_BOTTOM)
        || (p.x == 640. && p.y == 160. && gold_position == RIGHT_TOP)
        || (p.x == 640. && p.y == 500. && gold_position == RIGHT_BOTTOM)
}

#[macroquad::main("tuco")]
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
    const FONT_SIZE: u16 = 20;

    // The colors
    let blue = Color::from_rgba(38, 139, 210, 255);
    let dark_green = Color::from_rgba(0, 43, 54, 255);
    let yellow = Color::from_rgba(181, 137, 0, 255);

    // Time and speed initial settings
    let mut last_update = get_time();
    let mut nav_lock = false;
    let speed = 0.1;
    let mut counter = get_time();
    let mut count: f64 = 60.;

    // Player and gold initial settings
    let mut p = Player::new((width / 2.) - 20., height / 2.);
    let move_length = 20.;
    let mut gold_position = rand_gold_position();

    let mut game_over = false;

    let music_params = PlaySoundParams {
        looped: true,
        volume: 1.,
    };
    audio::play_sound(&music, music_params);

    loop {
        if !game_over {
            clear_background(dark_green);

            draw_text_ex(
                format!("SCORE: {}", p.score).as_str(),
                10.,
                20.,
                TextParams {
                    font_size: FONT_SIZE,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_text_ex(
                format!("STEPS: {}", p.steps).as_str(),
                (width / 2.) - 60.,
                20.,
                TextParams {
                    font_size: FONT_SIZE,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_text_ex(
                format!("TIME REMAINING: {}", count).as_str(),
                width - 200.,
                20.,
                TextParams {
                    font_size: FONT_SIZE,
                    font: Some(&font),
                    color: blue,
                    ..Default::default()
                },
            );

            draw_line(1., 30., width - 1., 30., 5., blue);

            draw_rectangle_lines(40., 70., 200., 170., 10., blue);
            draw_rectangle_lines(40., 390., 200., 170., 10., blue);
            draw_rectangle_lines(
                (width / 2.) + 160.,
                70.,
                200.,
                170.,
                10.,
                blue,
            );
            draw_rectangle_lines(
                (width / 2.) + 160.,
                390.,
                200.,
                170.,
                10.,
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
                0 => draw_rectangle(120., 130., 40., 40., yellow), // left_top
                1 => draw_rectangle(120., 460., 40., 40., yellow), // left bottom
                2 => {
                    draw_rectangle((width / 2.) + 240., 130., 40., 40., yellow)
                } // right top
                3 => {
                    draw_rectangle((width / 2.) + 240., 460., 40., 40., yellow)
                } // right bottom
                _ => warn!(":D"),
            }

            if is_key_down(KeyCode::H) && p.x > 18. && !nav_lock {
                p.steps += 1;
                p.x -= move_length;
                nav_lock = true;
                if went_to_gold_location(&p, width, height)
                    && gold_found(&p, gold_position)
                {
                    audio::play_sound_once(&gold_sound);
                    p.score += 1;
                    gold_position = rand_gold_position();
                }
            } else if is_key_down(KeyCode::L)
                && p.x < (width - 38.)
                && !nav_lock
            {
                p.steps += 1;
                p.x += move_length;
                nav_lock = true;
                if went_to_gold_location(&p, width, height)
                    && gold_found(&p, gold_position)
                {
                    audio::play_sound_once(&gold_sound);
                    p.score += 1;
                    gold_position = rand_gold_position();
                }
            } else if is_key_down(KeyCode::J)
                && p.y < (height - 10.)
                && !nav_lock
            {
                p.steps += 1;
                p.y += move_length;
                nav_lock = true;
                if went_to_gold_location(&p, width, height)
                    && gold_found(&p, gold_position)
                {
                    audio::play_sound_once(&gold_sound);
                    p.score += 1;
                    gold_position = rand_gold_position();
                }
            } else if is_key_down(KeyCode::K) && p.y > 60. && !nav_lock {
                p.steps += 1;
                p.y -= move_length;
                nav_lock = true;
                if went_to_gold_location(&p, width, height)
                    && gold_found(&p, gold_position)
                {
                    audio::play_sound_once(&gold_sound);
                    p.score += 1;
                    gold_position = rand_gold_position();
                }
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                nav_lock = false;
            }

            if get_time() - counter > 1. {
                counter = get_time();
                count -= 1.;
                if count == 0. {
                    game_over = true;
                }
            }
        } else {
            audio::stop_sound(&music);
            let text_size = measure_text("GAME OVER", None, FONT_SIZE as _, 1.);
            draw_text(
                "GAME OVER",
                width / 2. - text_size.width,
                height / 2.,
                50.,
                DARKGRAY,
            );
        }

        next_frame().await
    }
}
