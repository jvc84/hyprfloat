use fork::{daemon, Fork};
use hyprfloat::{change_window_state, main_help};


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    if args[1] == "--help".to_string() {
        main_help("open")
    } else if let Ok(Fork::Child) = daemon(true, false) {
        change_window_state("open");
    }
}
