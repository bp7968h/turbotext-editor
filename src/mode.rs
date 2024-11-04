use crossterm::event::{self, KeyCode};
use anyhow::{Ok, Result};
use crate::Action;

pub enum Mode {
    Insert,
    Normal,
}

impl Mode {
    pub fn handle_event(&self, event: &event::Event) -> Result<Option<Action>> {
        match self {
            Mode::Normal => {
                Self::handle_normal_event(event)
            },
            Mode::Insert => {
                Self::handle_insert_event(event)
            },
        }
    }

    fn handle_normal_event(event: &event::Event) -> Result<Option<Action>> {
        match event {
            event::Event::Key(e) => match e.code {
                KeyCode::Char('q') => Ok(Some(Action::Quit)),
                KeyCode::Char('i') => Ok(Some(Action::SwitchMode)),
                KeyCode::Up => Ok(Some(Action::MoveUp)),
                KeyCode::Down => Ok(Some(Action::MoveDown)),
                KeyCode::Left => Ok(Some(Action::MoveLeft)),
                KeyCode::Right => Ok(Some(Action::MoveRight)),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn handle_insert_event(event: &event::Event) -> Result<Option<Action>> {
        match event {
            event::Event::Key(e) => match e.code {
                KeyCode::Esc => Ok(Some(Action::SwitchMode)),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}