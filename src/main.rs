// ----------
// Module imports necessary to run the app.
// https://ratatui.rs/tutorials/hello-world/#imports
use crossterm::{
    event::{self, Event::Key, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    symbols::Marker,
    widgets::canvas::*,
};

// ----------

use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};

mod player;

use player::Player;

// ----------
// Required `init` and `restore` functions.
// https://ratatui.rs/tutorials/hello-world/#setting-up-and-restoring-the-terminal
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
// ----------

fn main() -> io::Result<()> {
    let mut terminal = init_terminal()?;

    let mut p = Player::new(0.0, 0.0);

    // ----------
    // the game loop
    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let the_canvas = Canvas::default()
                .marker(Marker::Bar)
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&Line {
                        x1: -150.0,
                        y1: 90.0,
                        x2: 150.0,
                        y2: 90.0,
                        color: Color::Cyan,
                    });

                    ctx.layer();

                    ctx.print(
                        (area.width / 2) as f64,
                        (area.height / 2) as f64,
                        area.width.to_string(),
                    );

                    ctx.print(
                        (area.width / 2 + 50) as f64,
                        (area.height / 2 + 50) as f64,
                        p.steps.to_string(),
                    );
                });

            frame.render_widget(the_canvas, area);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Key(ke) = event::read()? {
                if ke.code == KeyCode::Char('q') || ke.code == KeyCode::Esc {
                    break;
                } else if ke.code == KeyCode::Char('h')  {
                    p.steps += 1;
                }
            }
        }
    }
    // ----------

    restore_terminal()
}
