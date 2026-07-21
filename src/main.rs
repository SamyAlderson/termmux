//! Main entry point for the terminal multiplexer.

#![allow(unused_imports)]

use crossbeam::channel::{select, Receiver, Sender};
use std::{error::Error, io};
use termion::event::Key;
use termion::input::TermRead;

mod mux;
mod utils;

/// The main function, which is the entry point for the program.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the terminal for input and output
    let (tx, rx) = crossbeam::channel::unbounded();
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Create a sender and receiver for inter-process communication
    let (sender, receiver) = tx.split();

    // Spawn a task to handle input from the user
    let input_task = std::thread::spawn(move || {
        for c in stdin.keys() {
            match c {
                Ok(key) => {
                    if let Some(event) = key.to_event() {
                        match event.key {
                            Key::Char('q') => {
                                // Quit the program when the user presses 'q'
                                return;
                            }
                            _ => sender.send(event).unwrap(),
                        }
                    }
                }
                Err(e) => eprintln!("Error reading from stdin: {}", e),
            }
        }
    });

    // Spawn a task to handle output to the user
    let output_task = std::thread::spawn(move || {
        for event in receiver {
            match event {
                event @ Key::Char(_) => {
                    // Handle character input from the user
                    // this was tricky
                    mux::handle_character(event);
                }
                _ => {
                    // Handle other types of input from the user
                    // not proud of this but it works
                    mux::handle_special(event);
                }
            }
        }
    });

    // Wait for the input or output task to finish
    select! {
        _ = input_task.join() => (),
        _ = output_task.join() => (),
    }

    // Print a message to the user when the program exits
    println!("Goodbye!");

    Ok(())
}

// Re-export the public API for the terminal multiplexer
pub use mux::{handle_character, handle_special};