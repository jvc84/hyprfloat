use toml;
use rand::Rng;
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
            ResizeActive, 
            ToggleFloating
        },
    },
};
pub mod data;
pub use data::*;
pub mod hfopen_cache;
pub use hfopen_cache::*;

#[derive(Clone)]
pub struct Parameters {
    pub config_pre_size: bool,
    pub default_size: bool,
    pub resize_x: bool,
    pub resize_y: bool,
    pub tiled: bool,
    pub origin_size: bool,
    pub binary: String,
    pub count_system: String,
    pub dispatcher_arg: String,
}

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    pub static ref PARAMETERS: Arc<RwLock<Parameters>> = {
       Arc::new(RwLock::new(
            Parameters {
                config_pre_size: true,
                default_size: false,
                resize_x: false,
                resize_y: false,
                tiled: false,
                origin_size: false,
                binary: "".to_string(),
                count_system: "origin".to_string(),
                dispatcher_arg: "any".to_string(),
            }
        ))
    };
    pub static ref SIZE_PARAMETERS: Arc<RwLock<HashMap<String, i16>>> = Arc::new(RwLock::new(HashMap::new()));
    pub static ref AT_PARAMETERS: Arc<RwLock<HashMap<String, i16>>> = Arc::new(RwLock::new(HashMap::new()));
    pub static ref EXACT: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
    pub static ref POSITION_VALUES: Vec<&'static str> = vec![
         "l", "left",
         "r", "right",
         "t", "top",
         "b", "bottom",
        "tl", "top-left",
        "tr", "top-right",
        "bl", "bottom-left",
        "br", "bottom-right",
        "cursor",
        "center",
        "random",
        "far",
        "close",
        "opposite",
        "any"
    ];
    pub static ref CLASS: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("")));
    pub static ref BIN: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("")));
}


pub fn notify_error(message: String)  {
    let _ = hyprland::ctl::notify::call(
        Icon::Error,
        time::Duration::from_secs(10),
        hyprland::ctl::Color::new(100, 50, 70 ,50) ,
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
    let mut output = get_parameter(
        format!("config_size_{}", axis).as_str(),
        SIZE_PARAMETERS.clone(),
        CLIENT_DATA.read().unwrap().axis_data.get(axis).unwrap().window_size.clone()
    );

    if SIZE_PARAMETERS.read().unwrap().contains_key(axis) {
        output = binding.get(axis).unwrap().clone();
    } else if output < 20 && PARAMETERS.read().unwrap().clone().origin_size == true {
        output = ((COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / 4f32) * 1.6 ).round() as i16;
    }

    output
}


pub fn add_size_values(conf_value: String, class: String) {
    let cli = CLIENT_DATA.read().unwrap().clone();
    let monitor_id = cli.clone().monitor;
    let table = get_table("windows", CONFIG_FILE.clone().as_str());
    let list: toml::Table = table.as_table().unwrap().clone();

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
            SIZE_PARAMETERS.write().unwrap().insert( format!("{}x", conf_value), param_vec[0].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert( format!("{}y", conf_value), param_vec[1].as_integer().unwrap().clone() as u16 as i16);


        } else if class_section
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .contains(&&"monitor_any".to_string()) {
            let param_vec = &class_section.as_table().unwrap()[&"monitor_any".to_string()];
            SIZE_PARAMETERS.write().unwrap().insert(format!("{}x", conf_value), param_vec[0].as_integer().unwrap().clone() as u16 as i16);
            SIZE_PARAMETERS.write().unwrap().insert(format!("{}y", conf_value), param_vec[1].as_integer().unwrap().clone() as u16 as i16);
        } else {
            PARAMETERS.write().unwrap().config_pre_size = false
        }
    } else {
        PARAMETERS.write().unwrap().config_pre_size = false
    }
}


fn move_to_corner(min_point: i16, max_point: i16, resolution: i16, cursor_pos: i16, window_size: i16) -> i16 {
    if cursor_pos <= (resolution / 2 ) + min_point {
        min_point
    } else {
        max_point - window_size
    }
}


pub fn position(axis: &str) -> Result<i16, String> {
    update_data();
    let conf = CONFIG_DATA.read().unwrap().clone();
    let conf_axis = conf.axis_data.get(axis).unwrap().clone();
    let cli = CLIENT_DATA.read().unwrap().clone();
    let cli_axis = cli.axis_data.get(axis).unwrap().clone();
    let params = PARAMETERS.read().unwrap().clone();
    let at_params = AT_PARAMETERS.read().unwrap();
    let size_params = SIZE_PARAMETERS.read().unwrap();
    let system = params.count_system.as_str();
    let dispatcher_arg = params.dispatcher_arg.as_str();

    let mut check_borders = false;
    let mut offset = -4;

    if size_params.contains_key(axis) {
        offset = size_params.get(axis).unwrap().clone() / -2;
    } else if params.default_size == true {
        offset = conf_axis.default_size / -2
    } else if params.origin_size == false {
        if system == "origin" {
            offset =  ((COUNT_DATA.read().unwrap().get("y").unwrap().monitor_resolution as f32 / -16f32) * 1.6).round() as i16;
        }
}


    let mut min_point = 0;
    let mut max_point = cli_axis.monitor_max_point - cli_axis.monitor_min_point;
    let mut resolution = max_point;
    let mut cursor_pos = cli_axis.cursor_pos - cli_axis.monitor_min_point;
    let mut window_size = cli_axis.window_size;
    let window_position = cli_axis.window_pos - cli_axis.monitor_min_point;

    if conf.detect_padding == true {
        min_point = min_point + conf_axis.padding_min;
        max_point = max_point - conf_axis.padding_max;
        resolution = resolution  + conf_axis.padding_min - conf_axis.padding_max;
        cursor_pos = cursor_pos + conf_axis.padding_min;
    }

    if  system == "position" {
        offset = cli_axis.window_size / -2;
    }

    let mut position : Result<i16, String> = match dispatcher_arg {
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
        "t" | "top" => {
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
        "br" | "bottom-right" => {
            Ok(max_point + offset * 2)
        },
        "center"   => Ok((max_point + min_point - offset * -2) / 2),
        "close"    => {
            Ok(move_to_corner(min_point, max_point, resolution, cursor_pos, offset * -2))
        },
        "far"      => {
            if conf.detect_padding  == true {
                resolution = cli_axis.monitor_max_point - cli_axis.monitor_min_point + conf_axis.padding_min;
                cursor_pos = cli_axis.cursor_pos - cli_axis.monitor_min_point + conf_axis.padding_max - conf_axis.padding_min;
            }

            Ok(move_to_corner(min_point, max_point, resolution, resolution - cursor_pos, offset * -2))
        },
        "cursor"   => {
            check_borders = true;
            if conf.detect_padding == true {
                cursor_pos = cursor_pos - conf_axis.padding_min
            }
            Ok(cursor_pos + offset)
        },
        "opposite" => {
            check_borders = true;
            if conf.detect_padding == true {
                cursor_pos = cursor_pos - conf_axis.padding_min;
            }
            Ok(max_point - cursor_pos  + min_point + offset)
        },
        "random"   => {
            check_borders = true;
            if system == "position" {
                if params.origin_size == true {
                    Ok(window_position + offset)
                } else {
                    Ok(window_position)
                }

            } else {
                let mut rng = rand::rng();
                Ok(rng.random_range(min_point..=(max_point + offset * 2)))

            }
        },
        _          => {
            check_borders = true;
            if system == "position" {
                if params.origin_size == true &&
                    SIZE_PARAMETERS.read().unwrap().contains_key(format!("config_size_{}", axis).as_str()) 
                {
                    Ok(window_position - window_size / 2)
                } else {
                    Ok(window_position)
                }
            } else {
                Err("any".to_string())
            }
        }
    };
    
    if at_params.contains_key(axis) {
        position = Ok(at_params.get(axis).unwrap().clone());
        
        if system == "origin" && size_params.contains_key(axis) == false {
            let mut k = 1;
            if params.origin_size == false {
                k = 2
            }

            position = Ok(
                at_params.get(axis).unwrap().clone() -
                    (COUNT_DATA.read().unwrap().get("x").unwrap().monitor_resolution as f32 / -12f32 * 1.6).round() as i16 / k
           );
        }
    }

    window_size = offset * -2;

    let mut output = position?;
    if check_borders == true {
        if output <= min_point {
            output = min_point
        } else if output + window_size >= max_point {
            output = max_point - window_size
        }
    }

    position = Ok(output);
    
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
    check_class();

    let cli = CLIENT_DATA.read().unwrap().clone();
    let conf = CONFIG_DATA.read().unwrap().clone();
    let params = PARAMETERS.read().unwrap().clone();

    if params.binary == "hfopen".to_string() && 
        ((!cli.floating && !params.tiled) || (params.tiled && cli.floating)) ||
        params.binary == "hftogglefloating".to_string() {
        let _ = Dispatch::call(ToggleFloating(None));
    }

    if Client::get_active().unwrap().unwrap().floating == false { exit(0x0100) }

    if params.default_size {
        SIZE_PARAMETERS.write().unwrap().insert("x".to_string(), conf.axis_data.get("x").unwrap().default_size);
        SIZE_PARAMETERS.write().unwrap().insert("y".to_string(), conf.axis_data.get("y").unwrap().default_size);
    }
        add_size_values("config_size_".to_string(), CLIENT_DATA.read().unwrap().clone().class);
        let _ = Dispatch::call(ResizeActive(Exact(
            compare_size_parameters("x"),
            compare_size_parameters("y"),
        )));


    update_data();
    let _ = Dispatch::call(window_position());
}


pub fn get_origin_size(axis: &str) -> i16 {
    let size_params =  SIZE_PARAMETERS.read().unwrap();

    if size_params.contains_key(axis) {
        size_params.get(axis).unwrap().clone()
    } else if PARAMETERS.read().unwrap().clone().default_size == true {
        CONFIG_DATA.read().unwrap().axis_data.get(axis).unwrap().default_size
    } else {
        get_parameter(
            format!("config_size_{}", axis).as_str(),
            SIZE_PARAMETERS.clone(),
            8
        )
    }
}


pub fn origin_position(axis: &str) -> String {
    let result = position(axis);

    if let Ok(_) = result {
        result.unwrap().to_string()
    } else {
        "".to_string()
    }
}


pub fn change_window_state() {
    PARAMETERS.write().unwrap().count_system = "position".to_string();
    let params = PARAMETERS.read().unwrap().clone();

    if params.binary == "hfopen".to_string() {
        let mut event = hyprland::event_listener::EventListener::new();

        event.add_window_open_handler(
            move |_| {
                dispatch_client();
                exit(0x0100)
            });
        let _ = event.start_listener();

    } else {
        let start_addr = Client::get_active()
            .unwrap()
            .unwrap_or(
                empty_client()
            ).address;

        for _i in 0..=20 {
            let mid_addr = Client::get_active()
                .unwrap()
                .unwrap_or(
                    empty_client()
                ).address;

            if (mid_addr != start_addr && params.binary == "hfopen".to_string())
                || (mid_addr == start_addr && params.binary == "hftogglefloating".to_string()) {

                dispatch_client();
                break
            }
            sleep(time::Duration::from_millis(50));
        }
    }
}
