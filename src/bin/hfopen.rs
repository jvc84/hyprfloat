use fork::{daemon, Fork};
use hyprfloat::define_args;


fn main() {
    if let Ok(Fork::Child) = daemon(true, false) {
        define_args("open");
    }
}
