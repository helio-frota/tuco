// Using the external dependencies.
extern crate allegro;
extern crate allegro_font;
extern crate allegro_primitives;
extern crate allegro_ttf;

use allegro::*;
use allegro_font::*;
use allegro_primitives::*;
use allegro_ttf::*;
use rand::Rng;

// Creating some constants to add a bit of meaning.
const DISPLAY_WIDTH: i32 = 800;
const DISPLAY_HEIGHT: i32 = 600;
const RECT_THICKNESS: i32 = 5;

// This enum represents the 4 possible positions of the gold in the game.
enum GoldPosition {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

// This struct represents the Player.
struct Player {
    x: i32,
    y: i32,
    score: i32,
    move_peed: i32,
    steps: i32,
}

/**
 * This function is used only to return a random number between 0 and 3
 * to be used as a gold position.
 */
fn rand_gold_position() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=3)
}

/**
 * This is a very primitive and dumb collision check.
 */
fn basic_collision(x: i32, y: i32, gold_x: i32, gold_y: i32, width: i32, height: i32) -> bool {
    if x + width < gold_x || x > gold_x + width || y + height < gold_y || y > gold_y + height {
        return false;
    }
    true
}

/**
 * This function is to check if the Player collided with the gold.
 * If yes then obviously that means the Player picked the gold coin
 * (not exactly a coin but whatever).
 */
fn collided(player: Player) -> bool {
    basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 850,
        DISPLAY_HEIGHT - 550,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 850,
        DISPLAY_HEIGHT - 200,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 150,
        DISPLAY_HEIGHT - 550,
        40,
        40,
    ) || basic_collision(
        player.x,
        player.y,
        DISPLAY_WIDTH - 150,
        DISPLAY_HEIGHT - 200,
        40,
        40,
    )
}

allegro_main! {
    // We need to init the allegro stuffs.
    let core = Core::init().unwrap();
    // Things like draw_line and other primitives are part of PrimitivesAddon
    let primitives = PrimitivesAddon::init(&core).unwrap();
    // We need to install/register the keyboard to be able to use it.
    core.install_keyboard().unwrap();
    // We need to init the font addon to be able to use it.
    let font_addon = FontAddon::init(&core).unwrap();
    // We need to init the TTF addon
    let ttf_addon = TtfAddon::init(&font_addon).unwrap();
    // Instantiating a new display.
    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    // Setting the window title (not sure if we need this for this game).
    display.set_window_title("Ecstasy of gold");

    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

    // Loading our custom font from the disk.
    let font = ttf_addon.load_ttf_font("OpenSans-Regular.ttf",
    36, Flag::zero()).unwrap();

    // Instantiating and registering all the sources that creates events.
    let eq = EventQueue::new(&core).unwrap();
    eq.register_event_source(timer.get_event_source());
    eq.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut redraw = true;
    timer.start();

    let dark_green = Color::from_rgb_f(0.0, 43.0, 54.0);
    let yellow = Color::from_rgb_f(181.0, 137.0, 0.0);
    let black = Color::from_rgb_f(0.0, 0.0, 0.0);

    'exit: loop {

        if redraw && eq.is_empty() {
            core.clear_to_color(black);

            core.draw_text(&font, yellow,
            (display.get_width() / 2) as f32, (display.get_height() / 2) as f32,
            FontAlign::Centre, &rand_gold_position().to_string());

            core.draw_text(&font, dark_green,
                10.0, 1.0, FontAlign::Left, "SCORE: 10");
            core.draw_text(&font, dark_green,
                ((DISPLAY_WIDTH / 2) - 150) as f32, 1.0,
                FontAlign::Left, "STEPS: 10");
            core.draw_text(&font, dark_green,
                (DISPLAY_WIDTH - 360) as f32, 1.0,
                FontAlign::Left, "TIME REMAINING: 10");
            primitives.draw_line(1.0, 50.0,
                (DISPLAY_WIDTH - 1) as f32, 50.0, dark_green,
                RECT_THICKNESS as f32);

            primitives.draw_rectangle(80.0, 140.0, 280.0, 340.0,
                dark_green, RECT_THICKNESS as f32);
            primitives.draw_rectangle(80.0, 480.0, 280.0, 680.0,
                dark_green, RECT_THICKNESS as f32);

            primitives.draw_rectangle((DISPLAY_WIDTH - 80) as f32,
            140.0, (DISPLAY_WIDTH - 280) as f32,
            340.0, dark_green, RECT_THICKNESS as f32);

            primitives.draw_rectangle((DISPLAY_WIDTH - 80) as f32,
            480.0, (DISPLAY_WIDTH - 280) as f32,
            680.0, dark_green, RECT_THICKNESS as f32);

            core.flip_display();
            redraw = false;
        }

        match eq.wait_for_event() {
            DisplayClose{..} => break 'exit,
            KeyDown{keycode: k, ..} if k == KeyCode::Escape => {
                println!("bye");
                break 'exit;
            },
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}
