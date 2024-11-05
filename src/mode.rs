use std::fmt;

use crossterm::event::{self, KeyCode};
use anyhow::{Ok, Result};
use crate::Action;

pub enum Mode {
    Insert,
    Normal,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
        }
    }
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
                KeyCode::Up | KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
                KeyCode::Down | KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
                KeyCode::Left | KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
                KeyCode::Right | KeyCode::Char('l') => Ok(Some(Action::MoveRight)),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn handle_insert_event(event: &event::Event) -> Result<Option<Action>> {
        match event {
            event::Event::Key(e) => match e.code {
                KeyCode::Esc => Ok(Some(Action::SwitchMode)),
                KeyCode::Char(c) => Ok(Some(Action::Write(c))),
                KeyCode::Enter => Ok(Some(Action::NewLine)),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}