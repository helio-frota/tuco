extern crate allegro;
extern crate allegro_font;

use allegro::*;
use allegro_font::*;
use rand::Rng;

const DISPLAY_WIDTH: i32 = 800;
const DISPLAY_HEIGHT: i32 = 600;
const RECT_THICKNESS: i32 = 5;

enum GoldPosition {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

struct Player {
    x: i32,
    y: i32,
    score: i32,
    move_peed: i32,
    steps: i32,
}

fn rand_gold_position() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..=3);
}

fn basic_collision(x: i32, y: i32, gold_x: i32, gold_y: i32, width: i32, height: i32) -> bool {
    if x + width < gold_x || x > gold_x + width || y + height < gold_y || y > gold_y + height {
        return false;
    }
    return true;
}

fn collided(player: Player) -> bool {
    return basic_collision(
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
    );
}

allegro_main! {
    let core = Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();

    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font = Font::new_builtin(&font_addon).unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();
    'exit: loop
    {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));
            core.draw_text(&font, Color::from_rgb_f(1.0, 1.0, 1.0),
                (display.get_width() / 2) as f32, (display.get_height() / 2) as f32,
                FontAlign::Centre, &rand_gold_position().to_string());
            core.flip_display();
            redraw = false;
        }

        match queue.wait_for_event()
        {
            DisplayClose{..} => break 'exit,
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}
