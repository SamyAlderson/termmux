// tests/mux.rs
#[cfg(test)]
mod tests {
    use super::*;
    use termmux::*;

    #[test]
    fn test_mux_new() {
        let mux = Mux::new();
        assert!(mux.is_valid());
    }

    #[test]
    fn test_mux_add_window() {
        let mut mux = Mux::new();
        mux.add_window(Window::new()).unwrap();
        assert_eq!(mux.num_windows(), 1);
    }

    #[test]
    fn test_mux_get_window() {
        let mut mux = Mux::new();
        let window = mux.add_window(Window::new()).unwrap();
        assert_eq!(mux.get_window(0), window);
    }

    #[test]
    fn test_mux_remove_window() {
        let mut mux = Mux::new();
        let window = mux.add_window(Window::new()).unwrap();
        mux.remove_window(window).unwrap();
        assert_eq!(mux.num_windows(), 0);
    }

    #[test]
    fn test_mux_split_window() {
        let mut mux = Mux::new();
        let window = mux.add_window(Window::new()).unwrap();
        let new_window = window.split().unwrap();
        assert_eq!(mux.num_windows(), 2);
    }

    #[test]
    fn test_mux_resize_window() {
        let mut mux = Mux::new();
        let window = mux.add_window(Window::new()).unwrap();
        window.resize(10, 20).unwrap();
        assert_eq!(window.get_size(), (10, 20));
    }

    #[test]
    fn test_mux_session_create() {
        let mut mux = Mux::new();
        let window = mux.add_window(Window::new()).unwrap();
        let session = mux.create_session().unwrap();
        assert_eq!(session.num_windows(), 1);
    }

    #[test]
    fn test_mux_session_add_window() {
        let mut mux = Mux::new();
        let session = mux.create_session().unwrap();
        let window = session.add_window(Window::new()).unwrap();
        assert_eq!(session.num_windows(), 1);
    }
}