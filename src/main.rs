use std::io::{self, Write};
use crossterm::{cursor, event::{ read }, execute, queue, terminal};
use turbotext_editor::{Action, Mode};

fn main() -> anyhow::Result<()>{
    let mut stdout = io::stdout();
    let mut mode = Mode::Normal;
    let cursor_x: u16 = 0;
    let cursor_y: u16 = 0;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    loop {
        queue!(stdout, cursor::MoveTo(cursor_x, cursor_y))?;
        stdout.flush()?;

        let event = read()?;
        let action = mode.handle_event(&event)?;

        if let Some(action) = action {
            match action {
                Action::Quit => break,
                Action::SwitchMode => {
                    mode = match mode {
                        Mode::Normal => Mode::Insert,
                        Mode::Insert => Mode::Normal,
                    }
                },
                _ => {}
            }   
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
