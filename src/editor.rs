use crate::{Action, Mode};
use anyhow::Result;
use crossterm::{cursor, event::read, execute, queue, style::{self, Print, Color, SetForegroundColor, ResetColor}, terminal};
use std::io::{self, Write};

pub struct Editor {
    size: (u16, u16),
    cursor_y: u16,
    cursor_x: u16,
    mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            size: (0,0),
            cursor_y: 0,
            cursor_x: 0,
            mode: Mode::Normal,
        }
    }

    fn draw_surface(&self, stdout: &mut io::Stdout) -> Result<()> {
        self.draw_status_line(stdout)?;
        queue!(stdout, cursor::MoveTo(self.cursor_x, self.cursor_y))?;
        stdout.flush()?;

        Ok(())
    }

    fn draw_status_line(&self, stdout: &mut io::Stdout) -> Result<()> {
        queue!(stdout, cursor::MoveTo(0, self.size.1 - 1))?;
        queue!(stdout, SetForegroundColor(Color::DarkGreen), Print(format!(" {}", self.mode)))?;

        let status_text = format!("Ln: {}, Col: {}", self.cursor_y, self.cursor_x);
        let x_position = self.size.0.saturating_sub(status_text.len() as u16 + 1);

	queue!(stdout, cursor::MoveTo(x_position, self.size.1 - 1))?;
	queue!(stdout, Print(status_text), ResetColor)?;

        Ok(())
    }

    fn write_to_screen(&mut self, stdout: &mut io::Stdout, c: char) -> Result<()> {
        queue!(stdout, style::Print(c))?;
        stdout.flush()?;
        self.cursor_x += 1;

        if self.cursor_x >= terminal::size()?.0 {
            self.handle_new_line();
        }

        Ok(())
    }

    fn handle_new_line(&mut self) -> () {
        self.cursor_y += 1u16;
        self.cursor_x = 0;
    }

    pub fn init(&mut self, stdout: &mut io::Stdout) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(stdout, terminal::EnterAlternateScreen)?;
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        self.size = terminal::size()?;

        loop {
            self.draw_surface(stdout)?;

            let event = read()?;
            if let Some(action) = self.mode.handle_event(&event)? {
                match action {
                    Action::Quit => break,
                    Action::SwitchMode => {
                        self.mode = match self.mode {
                            Mode::Normal => Mode::Insert,
                            Mode::Insert => Mode::Normal,
                        }
                    }
                    Action::MoveUp => self.cursor_y = self.cursor_y.saturating_sub(1u16),
                    Action::MoveDown => self.cursor_y += 1u16,
                    Action::MoveLeft => self.cursor_x = self.cursor_x.saturating_sub(1u16),
                    Action::MoveRight => self.cursor_x += 1u16,
                    Action::Write(c) => self.write_to_screen(stdout, c)?,
                    Action::NewLine => self.handle_new_line(),
                }
            }
        }

        execute!(stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}
