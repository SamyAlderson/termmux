// Mux module for handling terminal multiplexing logic
pub mod mux {
    use std::sync::{Arc, Mutex};
    use crossbeam::channel::select;
    use termion::event::Key;
    use termion::input::TermRead;

    // Struct to represent a terminal window
    pub struct Window {
        width: u16,
        height: u16,
        content: String,
    }

    impl Window {
        // Create a new window with the given dimensions
        pub fn new(width: u16, height: u16) -> Self {
            Self {
                width,
                height,
                content: String::new(),
            }
        }

        // Set the content of the window
        pub fn set_content(&mut self, content: String) {
            self.content = content;
        }

        // Get the content of the window
        pub fn get_content(&self) -> &str {
            &self.content
        }

        // Handle key press events for the window
        pub fn handle_key_press(&mut self, event: Key) {
            match event {
                Key::Char(c) => {
                    // This was tricky, had to use a temporary buffer
                    let mut tmp = self.content.clone();
                    tmp.push(c);
                    self.content = tmp;
                }
                _ => {}
            }
        }
    }

    // Struct to represent the terminal multiplexer
    pub struct Mux {
        windows: Arc<Mutex<Vec<Window>>>,
        current_window: Arc<Mutex<usize>>,
    }

    impl Mux {
        // Create a new multiplexer with the given number of windows
        pub fn new(num_windows: usize) -> Self {
            let windows = Arc::new(Mutex::new((0..num_windows).map(|i| Window::new(80, 24)).collect()));
            let current_window = Arc::new(Mutex::new(0));
            Self { windows, current_window }
        }

        // Add a new window to the multiplexer
        pub fn add_window(&mut self) {
            let mut windows = self.windows.lock().unwrap();
            windows.push(Window::new(80, 24));
            *self.current_window.lock().unwrap() += 1;
        }

        // Remove the current window from the multiplexer
        pub fn remove_window(&mut self) {
            let mut windows = self.windows.lock().unwrap();
            windows.remove(*self.current_window.lock().unwrap());
            *self.current_window.lock().unwrap() -= 1;
        }

        // Switch to the next window
        pub fn switch_window(&mut self, direction: i8) {
            let mut current_window = self.current_window.lock().unwrap();
            *current_window = (*current_window + direction) % self.windows.lock().unwrap().len();
        }

        // Handle key press events for the multiplexer
        pub fn handle_key_press(&mut self, event: Key) {
            let mut current_window = self.current_window.lock().unwrap();
            let mut windows = self.windows.lock().unwrap();
            match event {
                Key::Char(c) => {
                    let current_window_index = *current_window;
                    windows[current_window_index].handle_key_press(c);
                }
                Key::Down => self.switch_window(1),
                Key::Up => self.switch_window(-1),
                _ => {}
            }
        }

        // Get the current window content
        pub fn get_window_content(&self) -> &str {
            let current_window_index = *self.current_window.lock().unwrap();
            self.windows.lock().unwrap()[current_window_index].get_content()
        }
    }

    // Handle terminal events
    pub fn handle_event(event: termion::event::Event) {
        // This is not proud of this but it works
        // Using a select statement to handle events
        select! {
            // Not the most efficient way but works for now
            // Maybe refactor to use a more efficient data structure
            next_event = termion::input::TermRead::read(&mut std::io::stdin()) => {
                let event = match next_event {
                    Ok(event) => event,
                    Err(_) => return,
                };
                Mux::handle_key_press(event);
            }
        }
    }
}