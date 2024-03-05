use crate::gold::GoldPosition;
use crate::player::Player;

/**
 * This is a very primitive and dumb collision check.
 */
pub fn basic_collision(
    x: i32,
    y: i32,
    gold_x: i32,
    gold_y: i32,
    width: i32,
    height: i32,
) -> bool {
    if x + width < gold_x
        || x > gold_x + width
        || y + height < gold_y
        || y > gold_y + height
    {
        return false;
    }
    true
}

fn gold_found_left_top(player: &Player, gold_position: i32) -> bool {
    if (player.x > 150 && player.x < 200)
        && (player.y > 200 && player.y < 250)
        && gold_position == GoldPosition::LeftTop as i32
    {
        return true;
    }
    false
}

fn gold_found_left_bottom(player: &Player, gold_position: i32) -> bool {
    if (player.x > 150 && player.x < 200)
        && (player.y > 560 && player.y < 570)
        && gold_position == GoldPosition::LeftBottom as i32
    {
        return true;
    }
    false
}

fn gold_found_right_top(player: &Player, gold_position: i32) -> bool {
    if (player.x > 870 && player.x < 880)
        && (player.y > 200 && player.y < 250)
        && gold_position == GoldPosition::RightTop as i32
    {
        return true;
    }
    false
}

fn gold_found_right_bottom(player: &Player, gold_position: i32) -> bool {
    if (player.x > 870 && player.x < 880)
        && (player.y > 560 && player.y < 570)
        && gold_position == GoldPosition::RightBottom as i32
    {
        return true;
    }
    false
}

fn gold_found(player: &Player, gold_position: i32) -> bool {
    if gold_found_left_top(player, gold_position)
        || gold_found_left_bottom(player, gold_position)
        || gold_found_right_top(player, gold_position)
        || gold_found_right_bottom(player, gold_position)
    {
        return true;
    }
    false
}
