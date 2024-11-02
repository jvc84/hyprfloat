use toml;
use rand::Rng;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::{
    sync::{Arc, RwLock},
    collections::HashMap,
    process::exit,
    thread::sleep,
    string::ToString,
    env,
    time,
};
use hyprland::{
    data::Client,
    prelude::*,
    dispatch::Position::Exact,
    ctl::notify::Icon,
    dispatch::{
        Dispatch,
        DispatchType,
        DispatchType::{
            Exec, 
            ResizeActive, 
            ToggleFloating
        },
    },
};

pub mod data;
pub use data::*;


#[derive(Deserialize, Clone)]
pub struct Parameters {
    pub make_float: bool,
    pub toggle_float: bool,
    pub resize: bool,
    pub resize_x: bool,
    pub resize_y: bool,
    pub tiled: bool,
    pub origin_size: bool,
    pub count_system: String,
    pub dispatcher_var: String,
}

pub fn get_variables() -> Parameters {
    Parameters {
        make_float: false,
        toggle_float: false,
        resize: false,
        resize_x: false,
        resize_y: false,
        tiled: false,
        origin_size: false,
        count_system: "origin".to_string(),
        dispatcher_var: "any".to_string(),
    }
}


lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    pub static ref PARAMETERS: Arc<RwLock<Parameters>> = {
       Arc::new(RwLock::new(get_variables()))
    };
    pub static ref SIZE_PARAMETERS: Arc<RwLock<HashMap<String, i16>>> = Arc::new(RwLock::new(HashMap::new()));
    pub static ref POSITION_PARAMETERS: Arc<RwLock<HashMap<String, i16>>> = Arc::new(RwLock::new(HashMap::new()));
}


pub fn notify_error(message: &str )  {
    let _ = hyprland::ctl::notify::call(
        Icon::Error,
        time::Duration::from_secs(10),
        hyprland::ctl::Color::new(100, 50, 50 ,50) ,
        format!(" Hyprfloat: {}", message.to_string())

    );
}

fn get_parameter(axis: &str, arg: Arc<RwLock<HashMap<String, i16>>>, default_value: i16) -> i16 {
    let binding = arg.read().unwrap();
    let result = binding.get(axis).clone();
    match result.clone() {
        Some(_) => {
            *result.unwrap()
        },
        None => {
            default_value
        }
    }
}


fn compare_size_parameters(axis: &str) -> i16 {
    let binding = SIZE_PARAMETERS.read().unwrap();
    let config_value = get_parameter(format!("config_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), CLIENT_DATA.read().unwrap().axis_data.get(axis).unwrap().window_size.clone());
    let resize_arg = get_parameter(format!("resize_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0);
    let mut output = config_value.clone();

    if resize_arg == 1 {
        output = binding.get(axis).unwrap().clone();
    } else if output < 20 && PARAMETERS.read().unwrap().origin_size == true {
        output = ((COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / 6f32) * 1.6 ).round() as i16;
    }
    
    output
}


fn size(global_class: &str) {
    let cli = CLIENT_DATA.read().unwrap().clone();
    let monitor_id = cli.clone().monitor;
    let table = get_table("windows", CONFIG_FILE.clone().as_str());
    let list: toml::Table = table.as_table().unwrap().clone();
    let mut class  = global_class.to_string();

    if class.clone() == "".to_string() {
        class = cli.class;
    }
    
    if list.keys().collect::<Vec<_>>().contains(&&class.clone()) {
        let class_section = list[&class].clone();
        let value = format!("{}{}", "monitor_", monitor_id);
        
        if class_section
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .contains(&&value.clone()) {
            let param_vec = &class_section.as_table().unwrap()[&value];
            
            SIZE_PARAMETERS.write().unwrap().insert("config_size_y".to_string(), param_vec[1].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert("config_size_x".to_string(), param_vec[0].as_integer().unwrap().clone() as u16 as i16);

        } else if class_section
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .contains(&&"monitor_any".to_string()) {

            let param_vec = &class_section.as_table().unwrap()[&"monitor_any".to_string()];

            SIZE_PARAMETERS.write().unwrap().insert("config_size_x".to_string(), param_vec[0].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert("config_size_y".to_string(), param_vec[1].as_integer().unwrap().clone() as u16 as i16);
        }
    }

    let _ = Dispatch::call(ResizeActive(Exact(
        compare_size_parameters("x"),
        compare_size_parameters("y"),
    )));
}


fn move_to_corner(min_point: i16, max_point: i16, resolution: i16, cursor_pos: i16, window_size: i16) -> i16 {
    if cursor_pos <= (resolution / 2 ) + min_point {
        min_point
    } else {
        max_point - window_size
    }
}


pub fn position(axis: &str) -> Result<i16, String> {
    let conf = CONFIG_DATA.read().unwrap().clone();
    let conf_axis = conf.axis_data.get(axis).unwrap().clone();
    let cli = CLIENT_DATA.read().unwrap().clone();
    let cli_axis = cli.axis_data.get(axis).unwrap().clone();
    let vars = PARAMETERS.read().unwrap().clone();
    let system = vars.count_system.as_str();
    let param = vars.dispatcher_var.as_str();

    let size_offset = get_parameter(axis, SIZE_PARAMETERS.clone(), 0) / -2;
    let mut offset = -4;

    if vars.origin_size == false {
        offset = ((COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / -12f32) *1.6).round() as i16;
    } else if size_offset != 0 {
        offset = size_offset;
    }

    let mut min_point = 0;
    let mut max_point = cli_axis.monitor_max_point - cli_axis.monitor_min_point;
    let mut resolution = max_point;
    let mut cursor_pos = cli_axis.cursor_pos - cli_axis.monitor_min_point;
    let window_size = cli_axis.window_size;
    let window_position = cli_axis.window_pos - cli_axis.monitor_min_point;

    if conf.detect_padding == true {
        min_point = min_point + conf_axis.padding_min;
        max_point = max_point - conf_axis.padding_max;
        resolution = resolution  + conf_axis.padding_min - conf_axis.padding_max;
        cursor_pos = cursor_pos + conf_axis.padding_min;
    }

    if system == "position" {
        offset = window_size / -2
    }

    let mut position : Result<i16, String> = match param {
        "cursor"   => {
            if conf.detect_padding == true {
                cursor_pos = cursor_pos - conf_axis.padding_min
            }
            Ok(cursor_pos + offset)
        },
        "center"   => Ok((max_point + min_point - offset * -2) / 2),
        "close"    => Ok(move_to_corner(min_point, max_point, resolution, cursor_pos, offset * -2)),
        "far"   => {
            if conf.detect_padding  == true {
                resolution  = resolution + conf_axis.padding_min * 2;
            }
            Ok(move_to_corner(min_point, max_point, resolution, resolution - cursor_pos, offset * -2))
        },
        "opposite" => {
            if conf.detect_padding  == true {
                max_point  = max_point + conf_axis.padding_min * 2;
            }
            Ok(max_point - cursor_pos + offset)
        },
        "random"   => {
            if system == "position" {
                Ok(window_position + offset)
            } else {
                let mut rng = rand::thread_rng();
                Ok(rng.gen_range(min_point..=(max_point + offset)))
            }
        },
        _          => {
            if system == "position" {
                Ok(window_position)
            } else {
                Err("any".to_string())
            }
        }
    };
    
    match system {
        "position" => {
            let mut output = position?;
            if CONFIG_DATA.read().unwrap().detect_padding == true && param != "center" && param != "any" {
                if output <= min_point {
                    output = min_point
                } else if output + window_size >= max_point {
                    output = max_point - window_size
                }
            }
            position = Ok(get_parameter(axis, POSITION_PARAMETERS.clone(), output) + cli_axis.monitor_min_point);
        },
        "origin" => {
            if get_parameter(format!("position_{}", axis).as_str(), POSITION_PARAMETERS.clone(), 0) == 1  &&
                get_parameter(format!("resize_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0) == 1 {
                position = Ok(
                    POSITION_PARAMETERS.read().unwrap().get(axis).unwrap().clone()
                );
            } else if get_parameter(format!("position_{}", axis).as_str(), POSITION_PARAMETERS.clone(), 0) == 1 {
                position = Ok(
                    POSITION_PARAMETERS.read().unwrap().get(axis).unwrap().clone()
                );
            }
        }, 
         _  => {
             notify_error(format!("No such coordinate system parameter: {}", system).as_str());
             exit(0x0100)
         }
    }
    
    position
}


pub fn window_position() -> DispatchType<'static> {
    DispatchType::MoveActive(Exact(
        position("x").unwrap(),
        position("y").unwrap(),
    ))
}


pub fn dispatch_client() {
    update_data();
    let cli = CLIENT_DATA.read().unwrap().clone();
    let vars = PARAMETERS.read().unwrap();

    if vars.make_float == true && cli.floating == false && vars.tiled == false ||
        vars.toggle_float == true ||
        vars.tiled == true && cli.floating == true {
        let _ = Dispatch::call(ToggleFloating(None));
    }

    if Client::get_active().unwrap().unwrap().floating == false { exit(0x0100) }
    
    if vars.resize == true {
        let _ = Dispatch::call(ResizeActive(
            Exact(
                CONFIG_DATA.read().unwrap().axis_data.get("x").unwrap().default_size,
                CONFIG_DATA.read().unwrap().axis_data.get("y").unwrap().default_size,
            )
        ));
    } else { size("") }

    update_data();
    let cord = window_position();
    let _ = Dispatch::call(cord);
}


fn origin_size(axis: &str) -> i16 {
    let backup = get_parameter(format!("config_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 8);
    let resize = get_parameter(format!("resize_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0);
    if resize == 1  {
        SIZE_PARAMETERS.read().unwrap().get(axis).unwrap().clone()
    } else {
        backup
    }
}


fn origin_position(axis: &str) -> String {
    let result = position(axis).clone();

    if let Ok(_) = result {
        result.unwrap().to_string()
    } else {
        "".to_string()
    }
}


fn dispatch_window() {
    let vars = PARAMETERS.read().unwrap();
    let start_addr = Client::get_active()
        .unwrap()
        .unwrap_or(
            empty_client()
        ).address;

    if vars.make_float == true && parse_fullscreen(Client::get_active().unwrap().unwrap_or(empty_client()).fullscreen) {
        let mut event = hyprland::event_listener::EventListener::new();

        event.add_window_open_handler(
            move |_| {
                dispatch_client();
                exit(0x0100)
            });
        let _ = event.start_listener();

    } else {
        for _i in 0..=20 {
            let mid_addr = Client::get_active()
                .unwrap()
                .unwrap_or(
                    empty_client()
                ).address;

            if (mid_addr != start_addr && vars.make_float == true)
                || (mid_addr == start_addr && vars.toggle_float == true) {
                dispatch_client();
                break
            }
            sleep(time::Duration::from_millis(50));
        }
    }
}

pub fn main_help(purpose: &str) {
    let mut binary = "";
    let mut executable = "";
    let mut open_parameters = "";
    
    match purpose {
        "open" => {
            binary = "hfopen";
            executable = "\"EXECUTABLE\"";
            open_parameters = " \
            \n    -t | --tiled                          - open window tiled\
            \n    -o | --origin-size                    - let program open window with specific size and then resize it.\
            \n        Recommended when size is predefined via config or console arguments\n";
        },
        "togglefloating" => binary = "hftogglefloating",
        _  => {
            notify_error(format!("No such purpose: {}", purpose).as_str());
            exit(0x0100)
        }    
    }

    println!("\
    \nUSAGE:\
    \n\n    {} [ARGUMENTS] {}\
    \n\nARGUMENTS:\
    \n\n    --help                                - show this message\
    \n    -c PATH | --config PATH               - define PATH for config\
    \n\n    -p PARAMETER | --position PARAMETER   - move/open window accordig to PARAMETER\
    \n        PARAMETERS:\
    \n            cursor      - move/open window at the cursor position\
    \n            center      - move/open window at the center\
    \n            close       - move/open window at the closest corner from cursor\
    \n            far         - move/open window at the farthest corner from cursor\
    \n            opposite    - move/open window at the mirror of cursor position\
    \n            random      - move/open window at the random position on screen\
    \n{}    \
    \n    -r | --resize                         - resize window according to config parameter `default_size`\
    \n    -w SIZE | --width  SIZE               - set window width to SIZE\
    \n    -h SIZE | --height SIZE               - set window height to SIZE\
    \n    -x POSITION | --x-pos POSITION        - set window move/open position on x axis to POSITION\
    \n    -y POSITION | --y-pos POSITION        - set window move/open position on y axis to POSITION\
    \n\nDEFAULT CONFIG PATH:\
    \n\n    `$HOME{}`
    ",
    binary,
    executable,
    open_parameters,
    XDG_PATH.as_str()
    );
    
    exit(0x0100)
}


pub fn change_window_state(purpose: &str) {

    /////// Parse arguments ///////

    if ARGS.len() < 2 && purpose == "open" {
        main_help(purpose);
    }

    for (i, arg) in ARGS[1..ARGS.len()].iter().enumerate() {
        match arg.as_str() {
            "--help"  =>  main_help(purpose),
            "--config" | "-c" => *CONFIG_DATA.write().unwrap() = config_data(ARGS[i + 2].to_string()),
            "--position" | "-p" => PARAMETERS.write().unwrap().dispatcher_var = ARGS[i + 2].to_string(),
            "--resize" | "-r" => PARAMETERS.write().unwrap().resize = true,
            "--tiled" | "-t" => PARAMETERS.write().unwrap().tiled = true,
            "--origin-size" | "-o" =>  PARAMETERS.write().unwrap().origin_size = true,
            "--width" | "-w" => {
                PARAMETERS.write().unwrap().origin_size = true;
                SIZE_PARAMETERS.write().unwrap().insert("resize_x".to_string(), 1);
                SIZE_PARAMETERS.write().unwrap().insert("x".to_string(), ARGS[i + 2].parse::<u16>().unwrap() as i16);
            },
            "--height" | "-h" =>{
                PARAMETERS.write().unwrap().origin_size = true;
                SIZE_PARAMETERS.write().unwrap().insert("resize_y".to_string(), 1);
                SIZE_PARAMETERS.write().unwrap().insert("y".to_string(), ARGS[i + 2].parse::<u16>().unwrap() as i16);
            },
            "--x-pos" | "-x" => {
                POSITION_PARAMETERS.write().unwrap().insert("position_x".to_string(), 1);
                POSITION_PARAMETERS.write().unwrap().insert("x".to_string(), ARGS[i + 2].parse::<i16>().unwrap());
            },
            "--y-pos" | "-y" =>{
                POSITION_PARAMETERS.write().unwrap().insert("position_y".to_string(), 1);
                POSITION_PARAMETERS.write().unwrap().insert("y".to_string(), ARGS[i + 2].parse::<i16>().unwrap());
            },
            _ => continue,
        };
    }

    /////// Apply parameters ///////

    let vars = PARAMETERS.read().unwrap().clone();

    let mut float = "float";
    if vars.tiled == true {
        float = "tiled"
    }

    if purpose == "togglefloating" {
        PARAMETERS.write().unwrap().toggle_float = true;
    } else if purpose == "open" {
        PARAMETERS.write().unwrap().make_float = true;

        let origin = format!(
            "move {} {}",
            origin_position("x"),
            origin_position("y"),
        );

        let mut start_size = "".to_string();
        if vars.origin_size == true {
            start_size = format!(
                "size {} {}",
                origin_size("x"),
                origin_size("y"),
            );
        }

        let _ = Dispatch::call(Exec(
            format!(
                "[{};{};{}] {}",
                float,
                origin,
                start_size,
                ARGS[ARGS.len() - 1]
            ).as_str()
        ));
    }

    PARAMETERS.write().unwrap().count_system = "position".to_string();

    dispatch_window()
}
