use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::io;

// This is the main entry point of the Motor Control System. Logic from this entry point will be
// abstracted into seperate files and functions as the system grows. Fow now, this system will
// support LED operations.

// The LED struct will store state related to LED's the Rasperry Pi Pico offers.
struct LED {
    id: bool,
    active: bool,
    pulsing: bool,
    pulse_frequency: i32, // In seconds
}

impl LED {
    pub fn new(id: bool, active: bool, pulsing: bool, pulse_frequency: i32) -> Self {
        LED {
            id,
            active: false,
            pulsing: false,
            pulse_frequency: 0,
        }
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    pub fn toggle_pulsing(&mut self) {
        self.pulsing = !self.pulsing;
    }

    pub fn set_pulse_frequency(&mut self, frequency: i32) {
        self.pulse_frequency = frequency;
    }
}

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => {
                    println!("'q' pressed. Exiting.");
                    break;
                }
                KeyCode::Up => println!("Up arrow pressed!"),
                _ => println!("Other key pressed: {:?}", key_event.code),
            }
        }
    }

    terminal::disable_raw_mode()?; // Disable raw mode when done
    Ok(())
}
