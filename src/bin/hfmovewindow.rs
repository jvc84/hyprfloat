use std::env;
use std::process::exit;
use hyprland::dispatch::{
    Dispatch,
    DispatchType,
    WindowMove::Direction,
    Direction::{Left, Up, Down, Right},
    DispatchType::MoveActive,
    Position::Exact,
};
use hyprfloat::{XDG_PATH, CLIENT_DATA, CONFIG_DATA, COUNT_DATA, PARAMETERS, notify_error, window_position, config_data};



fn detect_direction(direction: &str, axis: &str) -> i16 {
    let cli = CLIENT_DATA.read().unwrap();
    let cli_axis = cli.axis_data.get(axis).unwrap();
    let mut output = cli_axis.window_pos;
    let mut directions: Vec<&str> = vec![];

    match axis {
        "x" => directions = vec!["l", "r"],
        "y" => directions = vec!["u", "d"],
         _  => {
            notify_error("No such axis");
            exit(0x0100)
        }
    }

    if direction == directions[0] {
        output =  cli_axis.monitor_min_point + CONFIG_DATA.read().unwrap().axis_data.get(axis).unwrap().padding_min;
    } else if direction == directions[1] {
        output = COUNT_DATA.read().unwrap().get(axis).unwrap().max_pos;
    }

    output
}

fn move_window(arg: &str) {
    let dispatcher: DispatchType;

    if CLIENT_DATA.read().unwrap().floating == true && CONFIG_DATA.read().unwrap().detect_padding == true {
        dispatcher = MoveActive(Exact(
            detect_direction(arg, "x"),
            detect_direction(arg, "y"),
        ));
    } else {
        let direction: hyprland::dispatch::Direction = match arg {
            "l" => Left,
            "u" => Up,
            "d" => Down,
            "r" => Right,
             _  => exit(0x0100),
        };
        dispatcher  = DispatchType::MoveWindow(Direction(direction));
    }
    let _ = Dispatch::call(dispatcher);
}

fn movewindow_help() {
    println!("\
    \nUSAGE:\
    \n\n    hfmovewindow [ARGUMENTS] [DIRECTION]\
    \n\nARGUMENTS:\
    \n\n    --help                                - show this message\
    \n    -c PATH | --config PATH               - define PATH for config\
    \n    -p PARAMETER | --position PARAMETER   - move/open window according to PARAMETER\
    \n        PARAMETERS: cursor, center, close, far, opposite, random\
    \n\nDIRECTIONS:\
    \n\n    l        - move window left according to config parameters\
    \n    r        - move window right according to config parameters\
    \n    u        - move window up according to config parameters\
    \n    d        - move window down according to config parameters\
    \n\nDEFAULT CONFIG PATH:\
    \n\n    `$HOME{}`
    ",
    XDG_PATH.as_str()
    );
        
    exit(0x0100);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.clone()[1..args.len()].iter().enumerate() {
        match arg.as_str() {
            "--help" => movewindow_help(),
            "--config" | "-c" => {
                *CONFIG_DATA.write().unwrap() = config_data(args[i + 2].clone());
            },
            "--position" | "-p" => {
                PARAMETERS.write().unwrap().dispatcher_var = args[i + 2].clone();
                PARAMETERS.write().unwrap().count_system = "position".to_string();
            },
            _ => {
                continue;
            }
        }
    }

    let all_directions: Vec<_> = vec!["l", "r", "u", "d"];

    if PARAMETERS.read().unwrap().dispatcher_var != "any" {
        let _ = Dispatch::call(window_position());
    } else if args.len() > 1 && 
        all_directions.contains(&args.clone()[args.len() - 1].as_str()) {
        
        let arg = args.clone()[args.len() - 1].to_string();
        move_window(arg.as_str());
    } else {
        movewindow_help()
    }

}
