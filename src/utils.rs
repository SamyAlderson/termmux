// Utility functions for the terminal multiplexer

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

// Function to create a new session ID
fn new_session_id() -> String {
    // This was tricky, but we need to generate a unique ID for each session
    // Using a simple timestamp-based approach for now
    let mut session_id = String::new();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    session_id.push_str(&timestamp.to_string());
    session_id
}

// Function to create a new window ID
fn new_window_id() -> String {
    // Similar to session ID generation, we need a unique ID for each window
    let mut window_id = String::new();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    window_id.push_str(&timestamp.to_string());
    window_id
}

// Function to get the current terminal size
fn get_terminal_size() -> (u32, u32) {
    // Not proud of this but it works for now
    // Using termion for cross-platform terminal io
    let stdout = io::stdout();
    let size = termion::terminal::size();
    let (width, height) = if let Some(size) = size {
        (size.width, size.height)
    } else {
        (80, 24) // Default values if size is unknown
    };
    (width, height)
}

// Function to create a new directory if it doesn't exist
fn create_dir_if_missing<P: AsRef<PathBuf>>(dir: P) -> io::Result<()> {
    // This is a simple implementation, but it should work for our use case
    fs::create_dir_all(dir.as_ref())?;
    Ok(())
}

// Function to get the current working directory
fn get_cwd() -> io::Result<String> {
    // Using the std::env::current_dir function to get the cwd
    // This is a simple way to get the current working directory
    let cwd = env::current_dir()?;
    Ok(cwd.to_string_lossy().into_owned())
}

// Function to get the absolute path of a directory
fn get_abs_dir<P: AsRef<PathBuf>>(dir: P) -> io::Result<PathBuf> {
    // Using the std::path::Path::absolute function to get the absolute path
    // This is a simple way to get the absolute path of a directory
    let abs_dir = dir.as_ref().to_path_buf().canonicalize()?;
    Ok(abs_dir)
}