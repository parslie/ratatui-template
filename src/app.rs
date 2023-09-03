use std::error::Error;

use ratatui::Terminal;

use crate::types::Backend;

pub fn run(terminal: &mut Terminal<Backend>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
