use std::env;
use std::process::exit;
use hyprfloat::{
    CONFIG_DATA,
    client_data as cli,
    count_data as count,
    move_window,
    FromClient,
    Count
};

use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::dispatch::WindowMove::Direction;
use hyprland::dispatch::Direction::{Left, Up, Down, Right};
use hyprland::dispatch::DispatchType::MoveActive;
use hyprland::dispatch::Position::{Exact};
use lazy_static::lazy_static;

fn detect_dir(direction: &str, start_pos: i16,
              min_pos: i16,
              max_pos: i16,
              min_direction: &str,
              max_direction: &str    ) -> i16 {
    if direction == min_direction {
        min_pos
    } else if direction == max_direction {
        max_pos
    } else {
        start_pos
    }
}

lazy_static!(
    static ref LOC_CLI: FromClient = cli();
    static ref LOC_COUNT: Count = count();
);


fn move_dispatcher(arg: &str) {

    let dispatcher :DispatchType;

    if LOC_CLI.floating == true && CONFIG_DATA.detect_padding == true {
        let window_pos_x = detect_dir(
            arg,
            LOC_CLI.window_pos.x,
            LOC_CLI.screen_min.x + CONFIG_DATA.padding.left,
            LOC_COUNT.max_pos.x,
            "l",
            "r"
        );

        let window_pos_y = detect_dir(
            arg,
            LOC_CLI.window_pos.y,
            LOC_CLI.screen_min.y + CONFIG_DATA.padding.top,
            LOC_COUNT.max_pos.y,
            "u",
            "d"
        );

        dispatcher = MoveActive(Exact(window_pos_x, window_pos_y));

    } else {
        let direction: hyprland::dispatch::Direction = match arg {
            "l" => Left,
            "u" => Up,
            "d" => Down,
            "r" => Right,
            _  => exit(123),
        };

        dispatcher  = DispatchType::MoveWindow(Direction(direction));

    }
    let _ = Dispatch::call(dispatcher);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-p" | "--position" => {
            let dispatcher =  move_window(args[2].as_str(), LOC_CLI.clone(), LOC_COUNT.clone());
            let _ = Dispatch::call(dispatcher);

        },
        _ => {
            let bind = args[1].chars()
                .collect::<Vec<_>>()[0]
                .to_lowercase()
                .to_string();
            let arg = bind.as_str();
            
            move_dispatcher(arg)
                
        }
    }
}
