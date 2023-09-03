use std::io::Stdout;

use ratatui::prelude::CrosstermBackend;

pub type Backend = CrosstermBackend<Stdout>;
