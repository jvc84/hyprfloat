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
    let config_value = get_parameter(format!("define_config_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), CLIENT_DATA.read().unwrap().axis_data.get(axis).unwrap().window_size.clone());
    let resize_arg = get_parameter(format!("define_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0);
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
            
            SIZE_PARAMETERS.write().unwrap().insert("define_config_size_y".to_string(), param_vec[1].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert("define_config_size_x".to_string(), param_vec[0].as_integer().unwrap().clone() as u16 as i16);

        } else if class_section
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .contains(&&"monitor_any".to_string()) {

            let param_vec = &class_section.as_table().unwrap()[&"monitor_any".to_string()];

            SIZE_PARAMETERS.write().unwrap().insert("define_config_size_x".to_string(), param_vec[0].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert("define_config_size_y".to_string(), param_vec[1].as_integer().unwrap().clone() as u16 as i16);
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
    let params = PARAMETERS.read().unwrap().clone();
    let system = params.count_system.as_str();
    let param = params.dispatcher_var.as_str();

    let size_offset = get_parameter(axis, SIZE_PARAMETERS.clone(), 0) / -2;
    let mut offset = -4;

    if params.origin_size == false {
        offset = ((COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / -12f32) * 1.6).round() as i16;
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
        "l" | "left" => {
            let mut output = min_point;
            if axis == "y" {
                output = (max_point + min_point + offset * 2) / 2
            }
            Ok(output)
        },
        "r" | "right" => {
            let mut output = max_point + offset * 2;
            if axis == "y" {
                output = (max_point + min_point + offset * 2) / 2
            }
            Ok(output)
        },
        "t" | "top" =>  {
            let mut output = min_point;
            if axis == "x" {
                output = (max_point + min_point + offset * 2) / 2
            }
            Ok(output)
        },
        "b" | "bottom" => {
            let mut output = max_point + offset * 2;
            if axis == "x" {
                output = (max_point + min_point + offset * 2) / 2
            }
            Ok(output)
        }, 
        "tl" | "top-left" => Ok(min_point),
        "tr" | "top-right" => {
            let mut output = min_point;
            if axis == "x" {
                output = max_point + offset * 2
            }
            Ok(output)
        },
        "bl" | "bottom-left" => {
            let mut output = min_point;
            if axis == "y" {
                output = max_point + offset * 2
            }
            Ok(output)
        },
        "br" | "bottom-right" => Ok(max_point + offset * 2),
        "cursor"  => {
            if conf.detect_padding == true {
                cursor_pos = cursor_pos - conf_axis.padding_min
            }
            Ok(cursor_pos + offset)
        },
        "center"   => Ok((max_point + min_point - offset * -2) / 2),
        "close"    => Ok(move_to_corner(min_point, max_point, resolution, cursor_pos, offset * -2)),
        "far"      => {
            if conf.detect_padding  == true {
                resolution = cli_axis.monitor_max_point - cli_axis.monitor_min_point + conf_axis.padding_min;
                cursor_pos = cli_axis.cursor_pos - cli_axis.monitor_min_point + conf_axis.padding_max - conf_axis.padding_min;
            }

            Ok(move_to_corner(min_point, max_point, resolution, resolution - cursor_pos, offset * -2))
        },
        "opposite" => {
            if conf.detect_padding  == true {
                cursor_pos = cursor_pos - conf_axis.padding_min;
            }
            Ok(max_point - cursor_pos  + min_point + offset)
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
            if get_parameter(format!("define_position_{}", axis).as_str(), POSITION_PARAMETERS.clone(), 0) == 1 {
                if get_parameter(format!("define_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0) == 1 {
                    notify_error(POSITION_PARAMETERS.read().unwrap().get(axis).unwrap().clone().to_string().as_str());
                    position = Ok(
                        POSITION_PARAMETERS.read().unwrap().get(axis).unwrap().clone() 
                    );
                } else {
                    let mut k = 1;
                    if params.origin_size == false {
                        k = 2
                    }

                    position = Ok(
                        POSITION_PARAMETERS.read().unwrap().get(axis).unwrap().clone()
                            - (((COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / -12f32) * 1.6).round() as i16  / k)
                    );
                }
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
    let params = PARAMETERS.read().unwrap();

    if params.make_float == true && cli.floating == false && params.tiled == false ||
        params.toggle_float == true ||
        params.tiled == true && cli.floating == true {
        let _ = Dispatch::call(ToggleFloating(None));
    }

    if Client::get_active().unwrap().unwrap().floating == false { exit(0x0100) }
    
    if params.resize == true {
        let _ = Dispatch::call(ResizeActive(
            Exact(
                CONFIG_DATA.read().unwrap().axis_data.get("x").unwrap().default_size,
                CONFIG_DATA.read().unwrap().axis_data.get("y").unwrap().default_size,
            )
        ));
    } else { size("") }

    update_data();
    let _ = Dispatch::call(window_position());
}


fn origin_size(axis: &str) -> i16 {
    let resize = get_parameter(format!("define_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 0);
    if resize == 1  {
        SIZE_PARAMETERS.read().unwrap().get(axis).unwrap().clone()
    } else {
        get_parameter(format!("define_config_size_{}", axis).as_str(), SIZE_PARAMETERS.clone(), 8)

    }
}


fn origin_position(axis: &str) -> String {
    let result = position(axis);

    if let Ok(_) = result {
        result.unwrap().to_string()
    } else {
        "".to_string()
    }
}


fn dispatch_window() {
    let params = PARAMETERS.read().unwrap();
    let start_addr = Client::get_active()
        .unwrap()
        .unwrap_or(
            empty_client()
        ).address;

    if params.make_float == true && parse_fullscreen(Client::get_active().unwrap().unwrap_or(empty_client()).fullscreen) {
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

            if (mid_addr != start_addr && params.make_float == true)
                || (mid_addr == start_addr && params.toggle_float == true) {
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
    let mut function = "";
    
    match purpose {
        "open" => {
            function = "open";
            binary = "hfopen";
            executable = "\"EXECUTABLE\"";
            open_parameters = " \
            \n\
            \n    -t | --tiled                               - open window tiled\
            \n    -o | --origin-size                         - let program open window with specific size and then resize it.\
            \n        Recommended when size is predefined via config or console arguments\n";
        },
        "togglefloating" => {
            binary = "hftogglefloating";
            function = "move";
        },
        _  => {
            notify_error(format!("No such purpose: {}", purpose).as_str());
            exit(0x0100)
        }    
    }

    println!("\
    \nUSAGE:\
    \n\
    \n    {binary} [ARGUMENTS] {executable}\
    \n\
    \nARGUMENTS:\
    \n\
    \n    -h               | --help                  - show this message\
    {open_parameters}\
    \n    -d               | --default-size          - resize window according to config parameter `default_size`\
    \n    -c PATH          | --config PATH           - define PATH for config\
    \n    -s SIZE_XxSIZE_Y | --size SIZE_XxSIZE_Y    - set window size by x axis to SIZE_X, by y axis to SIZE_Y\
    \n    -m POS_XxPOS_Y   | --move POS_XxPOS_Y      - set window {function} position by x axis to POS_X, by y axis to POS_Y\
    \n    -p PARAMETER     | --position PARAMETER    - {function} window according to PARAMETER\
    \n        PARAMETERS:\
    \n            cursor             - at the cursor position\
    \n            center             - at the center\
    \n            close              - at the closest corner from cursor\
    \n            far                - at the farthest corner from cursor\
    \n            opposite           - at the mirror of cursor position\
    \n            random             - at the random position on screen\
    \n            l  | left          - at the left center position\
    \n            r  | right         - at the right center position\
    \n            t  | top           - at the top center position\
    \n            b  | bottom        - at the bottom center position\
    \n            tl | top-left      - at the top-left corner\
    \n            tr | top-right     - at the top-right corner\
    \n            bl | bottom-left   - at the bottom-left corner\
    \n            br | bottom-right  - at the bottom-right corner\
    \n\
    \nDEFAULT CONFIG PATH:\
    \n\
    \n    `$HOME{}`
    ",
    XDG_PATH.as_str()
    );
    
    exit(0x0100)
}


pub fn change_window_state(purpose: &str) {

    /////// Parse arguments ///////
    
    let mut origin = false;
    if ARGS.len() < 2 && purpose == "open" {
        main_help(purpose);
    }

    for (i, arg) in ARGS[1..ARGS.len()].iter().enumerate() {
        match arg.as_str() {
            "-h" | "--help"         => main_help(purpose),
            "-c" | "--config"       => *CONFIG_DATA.write().unwrap() = config_data(ARGS[i + 2].to_string()),
            "-p" | "--position"     => PARAMETERS.write().unwrap().dispatcher_var = ARGS[i + 2].to_string(),
            "-d" | "--default-size" => PARAMETERS.write().unwrap().resize = true,
            "-t" | "--tiled"        => PARAMETERS.write().unwrap().tiled = true,
            "-o" | "--origin-size"  => PARAMETERS.write().unwrap().origin_size = true,
            "-s" | "--size"         => {
                let size_list = ARGS[i + 2].split("x").collect::<Vec<&str>>();
                PARAMETERS.write().unwrap().origin_size = true;

                SIZE_PARAMETERS.write().unwrap().insert("define_size_x".to_string(), 1);
                SIZE_PARAMETERS.write().unwrap().insert("define_size_y".to_string(), 1);

                SIZE_PARAMETERS.write().unwrap().insert("x".to_string(), size_list[0].parse::<u32>().unwrap() as i16);
                SIZE_PARAMETERS.write().unwrap().insert("y".to_string(), size_list[1].parse::<u32>().unwrap() as i16);
                
            },
            "-m" | "--move"         => {
                let position_list = ARGS[i + 2].split("x").collect::<Vec<&str>>();

                POSITION_PARAMETERS.write().unwrap().insert("define_position_x".to_string(), 1);
                POSITION_PARAMETERS.write().unwrap().insert("define_position_y".to_string(), 1);
                
                POSITION_PARAMETERS.write().unwrap().insert("x".to_string(), position_list[0].parse::<i16>().unwrap());
                POSITION_PARAMETERS.write().unwrap().insert("y".to_string(), position_list[1].parse::<i16>().unwrap());
            },
            _                       => continue,
        };
    }

    /////// Apply parameters ///////

    let params = PARAMETERS.read().unwrap().clone();

    let mut float = "float";
    if params.tiled == true {
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
        if params.origin_size == true {
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
