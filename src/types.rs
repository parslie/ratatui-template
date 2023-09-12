use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    Frame as GenericFrame,
    Terminal as GenericTerminal,
};

pub type Backend = CrosstermBackend<Stdout>;
pub type Frame<'a> = GenericFrame<'a, Backend>;
pub type Terminal = GenericTerminal<Backend>;
