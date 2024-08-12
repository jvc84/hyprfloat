use fork::{daemon, Fork};

use hyprfloat::{
    make_args
};


fn main() {

    if let Ok(Fork::Child) = daemon(true, false) {
        make_args("open");
    }
}
