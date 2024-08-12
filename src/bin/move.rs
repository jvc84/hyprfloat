use std::env;
use hyprfloat::{
    config_data as conf,
    client_data as cli,
    count_data as count,
};

use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::dispatch::WindowMove::Direction;
use hyprland::dispatch::Direction::{Left, Up, Down, Right};
use hyprland::dispatch::Position::{Exact};
use std::process::exit;

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

fn main() {

    let args: Vec<String> = env::args().collect();
    let bind = args[1].chars()
        .collect::<Vec<_>>()[0]
        .to_lowercase()
        .to_string();
    let arg = bind.as_str();

    let dispatcher :DispatchType;

    if cli().floating == true && conf().detect_borders == true {
        let window_pos_x = detect_dir(
            arg,
            cli().window_pos.x,
            conf().margins.left,
            count().max_pos.x,
            "l",
            "r"
        );

        let window_pos_y = detect_dir(
            arg,
            cli().window_pos.y,
            conf().margins.top,
            count().max_pos.y,
            "u",
            "d"
        );

        dispatcher = DispatchType::MoveActive(Exact(window_pos_x, window_pos_y));

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
