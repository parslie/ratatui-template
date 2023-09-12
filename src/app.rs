mod widget;

use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use ratatui::{prelude::*, widgets::Paragraph};

#[derive(PartialEq)]
enum Status {
    Idling,
    Exiting,
}

pub struct Data {
    status: Status,
}

fn update(event: Event, data: &mut Data) {
    if let Event::Key(key) = event {
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            data.status = Status::Exiting;
        }
    }
}

fn render<B: Backend>(frame: &mut Frame<B>) {
    let hello_world = Paragraph::new("Hello, world!\nPress CTRL+C to exit...");
    frame.render_widget(hello_world, frame.size());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let poll_duration = Duration::from_millis(500);
    let mut data = Data {
        status: Status::Idling,
    };

    while data.status != Status::Exiting {
        terminal.draw(|frame| render(frame))?;
        if event::poll(poll_duration)? {
            update(event::read()?, &mut data);
        }
    } 
    
    Ok(())
}
