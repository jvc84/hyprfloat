use std::{
    process::exit
};
use hyprland::dispatch::{
    Dispatch,
    DispatchType,
    WindowMove::Direction,
    Direction::{Left, Up, Down, Right},
    DispatchType::MoveActive,
    Position::Exact,
};
use hyprfloat::{
    notify_error,
    window_position,
    config_data,
    update_data,
    CONFIG_FILE,
    CLIENT_DATA,
    CONFIG_DATA,
    COUNT_DATA,
    PARAMETERS,
    POSITION_VALUES,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, ignore_errors = false)]
struct Args {
    /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Direction to move window to
    #[arg(default_value_t = String::from("l"), hide_default_value = true, value_parser = ["l", "r", "u", "d"])]
    direction: String,
    /// Open window according to <POSITION> value
    #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
    position: String,
    /// Path to config file
    #[arg(short, long, default_value_t = CONFIG_FILE.clone())]
    config: String,
}


fn position_by_direction(direction: &str, axis: &str) -> i16 {
    let cli = CLIENT_DATA.read().unwrap();
    let cli_axis = cli.axis_data.get(axis).unwrap();
    let mut output = cli_axis.window_pos;
    
    let directions: (&str, &str) = match axis {
        "x" => ("l", "r"),
        "y" => ("u", "d"),
         _  => {
            notify_error(format!("No such axis: {axis}"));
            exit(0x0100)
        }
    };
    
    if direction == directions.0 {
        output = cli_axis.monitor_min_point + CONFIG_DATA.read().unwrap().axis_data.get(axis).unwrap().padding_min;
    } else if direction == directions.1 {
        output = COUNT_DATA.read().unwrap().get(axis).unwrap().max_position;
    }

    output
}


fn move_window(direction: &str) {
    let dispatcher: DispatchType;

    if CLIENT_DATA.read().unwrap().floating == true &&
        CONFIG_DATA.read().unwrap().detect_padding == true
    {
        dispatcher = MoveActive(Exact(
            position_by_direction(direction, "x"),
            position_by_direction(direction, "y"),
        ));
    } else {
        let direction: hyprland::dispatch::Direction = match direction {
            "l" => Left,
            "u" => Up,
            "d" => Down,
            "r" => Right,
             _  => {
                 notify_error(format!(
                     "No such direction:: {}", 
                     direction
                 ));
                 exit(0x0100)
             },
        };
        dispatcher  = DispatchType::MoveWindow(Direction(direction));
    }
    let _ = Dispatch::call(dispatcher);
}


fn main() {
    let parsed_args = Args::parse();

    *CONFIG_DATA.write().unwrap() = config_data(parsed_args.config);
    update_data();

    if parsed_args.force {
        CONFIG_DATA.write().unwrap().detect_padding = false
    }

    PARAMETERS.write().unwrap().dispatcher_arg = parsed_args.position.clone();
    if parsed_args.position != "any".to_string() {
        PARAMETERS.write().unwrap().count_system = "position".to_string();
    }
    
    if PARAMETERS.read().unwrap().dispatcher_arg != "any" {
        let _ = Dispatch::call(window_position());
    } else {
        move_window(parsed_args.direction.as_str()); 
    }
}
