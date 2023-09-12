use crossterm::event::KeyEvent;
use ratatui::prelude::*;

use crate::types::Frame;

pub trait CompositeWidget {
    fn update(&self, key: KeyEvent);
    fn render(&self, frame: &mut Frame, area: Rect);
}
