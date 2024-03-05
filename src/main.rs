// ----------
// Module imports necessary to run the app.
// https://ratatui.rs/tutorials/hello-world/#imports
use crossterm::{
    event::{self, KeyCode},
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

use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};
// ----------

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
                    ctx.print(
                        (area.width / 2) as f64,
                        (area.height / 2) as f64,
                        area.width.to_string(),
                    );
                    ctx.print(
                        (area.width / 2 + 50) as f64,
                        (area.height / 2 + 50) as f64,
                        area.height.to_string(),
                    );
                    ctx.draw(&Line {
                        x1: -150.0,
                        y1: 90.0,
                        x2: 150.0,
                        y2: 90.0,
                        color: Color::Cyan,
                    });
                });

            frame.render_widget(the_canvas, area);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    // ----------

    restore_terminal()
}
