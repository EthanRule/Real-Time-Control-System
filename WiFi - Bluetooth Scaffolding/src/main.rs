use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};
use std::io;

// This is the main entry point of the Motor Control System. Logic from this entry point will be
// abstracted into seperate files and functions as the system grows. Fow now, this system will
// support LED operations.

// The LED struct will store state related to LED's the Rasperry Pi Pico offers.
struct LED {
    id: u8,
    active: bool,
    pulsing: bool,
    pulse_frequency: i32, // In seconds
}

impl LED {
    pub fn new(id: u8, active: bool, pulsing: bool, pulse_frequency: i32) -> Self {
        LED {
            id,
            active: false,
            pulsing: false,
            pulse_frequency: 0,
        }
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active;

        match self.active {
            true => println!("LED ON"),
            false => println!("LED OFF"),
        }
    }

    pub fn toggle_pulsing(&mut self) {
        self.pulsing = !self.pulsing;

        match self.pulsing {
            true => println!("LED PULSING"),
            false => println!("LED NOT PULSING"),
        }
    }

    pub fn decrease_pulse_frequency(&mut self) {
        if self.pulse_frequency == 0 {
            println!("PULSE FREQUENCY MINIMUM ALREADY");
            return;
        }
        self.pulse_frequency -= 1;
        println!("PULSE FREQUENCY: {}s", self.pulse_frequency);
    }

    pub fn increase_pulse_frequency(&mut self) {
        if self.pulse_frequency == 10 {
            println!("PULSE FREQUENCY MAXXED ALREADY");
            return;
        }
        self.pulse_frequency += 1;
        println!("PULSE FREQUENCY: {}s", self.pulse_frequency);
    }
}

fn main() -> Result<(), io::Error> {
    let mut led = LED::new(1u8, false, false, 0);

    // This is the main control loop. Keybinds are mapped here to functions.
    loop {
        terminal::enable_raw_mode()?;
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('1') => led.toggle_active(),
                    KeyCode::Char('2') => led.toggle_pulsing(),
                    KeyCode::Char('3') => led.decrease_pulse_frequency(),
                    KeyCode::Char('4') => led.increase_pulse_frequency(),
                    KeyCode::Char('q') => {
                        println!("Program Terminated");
                        break;
                    }
                    _ => println!("No keybind associated with: {:?}", key_event.code),
                }
            }
        }
    }

    terminal::disable_raw_mode()?; // Disable raw mode when done
    Ok(())
}
