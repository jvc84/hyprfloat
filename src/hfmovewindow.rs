use std::{
    process::exit
};
use hyprland::dispatch::{
    Dispatch,
    DispatchType,
    WindowMove::Direction,
    Direction::{Left, Up, Down, Right},
    DispatchType::{MoveActive, MoveWindow},
    Position::Exact,
};
use crate::{notify_error, window_position, update_data, CLIENT_DATA, CONFIG_DATA, COUNT_DATA, TAGS, get_tags, tag_it, PARAMETERS, POSITION_VALUES, MONITOR_DATA, get_daemon_data, LAYERS_DATA, GIVE_ARGS, tag_window, ResizeactiveArgs};
use clap::Parser;


fn position_by_direction(direction: &str, axis: &str) -> i16 {
    let monitor = MONITOR_DATA.read().unwrap();
    let monitor_axis = monitor.axis_data.get(axis).unwrap().clone();
    let layers = LAYERS_DATA.read().unwrap().clone();
    let layers_axis = layers.get(axis).unwrap().clone();
    // let conf = CONFIG_DATA.read().unwrap().clone();
    // let conf_axis = conf.axis_data.get(axis).unwrap().clone();
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

    // let _ = tag_window(axis, "no_stick", true);

    let tags = TAGS.read().unwrap().clone();

    if direction == directions.0 {
        tag_it(axis, "min");

        // let _ = tag_window(axis, "min", false);
        // let _ = tag_window(axis, "max", true);
        output = monitor_axis.min_point + layers_axis.padding_min;
    } else if direction == directions.1 {
        tag_it(axis, "max");
        
        // let _ = tag_window(axis, "min", true);
        // let _ = tag_window(axis, "max", false);
        output = monitor_axis.max_point - layers_axis.padding_max - cli_axis.window_size;
    }

    output
}


fn get_direction(direction: &str) {
    let dispatcher: DispatchType;

    if CLIENT_DATA.read().unwrap().floating == true &&
        CONFIG_DATA.read().unwrap().detect_padding == true
    {   
        let m_x = position_by_direction(direction, "x");
        let m_y = position_by_direction(direction, "y");
        
        dispatcher = MoveActive(Exact(
            m_x,
            m_y
        ));
    } else {
        let direction: hyprland::dispatch::Direction = match direction {
            "l" => Left,
            "r" => Right,
            "u" => Up,
            "d" => Down,

             _  => {
                 notify_error(format!(
                     "No such direction:: {}", 
                     direction
                 ));
                 exit(0x0100)
             },
        };
        dispatcher  = MoveWindow(Direction(direction));
    }
    let _ = Dispatch::call(dispatcher);
}


pub fn movewindow(movewindow_args: crate::Modes) {

    let parsed_args =  match movewindow_args {
        crate::Modes::Movewindow {
            force,
            direction,
            position,
        }         => crate::MovewindowArgs {
            force,
            direction,
            position
        },
        _ => panic!("Could parse Movewindow enum into MovewindowArgs")
    };
    

    update_data();
    let _ = get_daemon_data();

    if parsed_args.force {
        CONFIG_DATA.write().unwrap().detect_padding = false
    }


    PARAMETERS.write().unwrap().position = parsed_args.position.clone();
    if parsed_args.position != "any".to_string() {
        PARAMETERS.write().unwrap().count_system = "position".to_string();
        *TAGS.write().unwrap() = get_tags();
        let _ = Dispatch::call(window_position());
    } else {
        get_direction(parsed_args.direction.as_str()); 
    }
}
