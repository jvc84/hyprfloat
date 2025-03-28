// use toml;
use rand::Rng;
use std::sync::{Arc, RwLock};
use std::{
    str::FromStr,
    collections::HashMap,
    process::exit,
    thread::sleep,
    string::ToString,
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


use std::thread;
pub mod data;
pub use data::*;
pub mod hfopen_cache;
pub use hfopen_cache::*;

pub mod hfdaemon;
pub use hfdaemon::*;

pub mod hfopen;
pub use hfopen::*;
pub mod hftogglefloating;
pub use hftogglefloating::*;

pub mod new_hfresizeactive;
pub use new_hfresizeactive::*;

// pub mod hfresizeactive;
// pub use hfresizeactive::*;

pub mod hfmovewindow;
pub use hfmovewindow::*;

pub mod globals;
pub use globals::*;
pub mod args;
pub use args::*;

pub mod structs;
pub use structs::*;


pub fn tag_it(axis: &str, anker: &str) {
    let conf =  CONFIG_DATA.read().unwrap().clone();
    if conf.detect_padding && conf.stick_to_borders && !conf.resize_through_borders {
        let min = "min";
        let max = "max";
        let no_stick = "no_stick";

        let mut no_stick_untag = true;
        let mut min_untag = true;
        let mut max_untag = true;

        if anker == no_stick {
            no_stick_untag = false;
        } else if anker == min {
            min_untag = false;
        } else if anker == max {
            max_untag = false;
        }

        let _ = tag_window(axis, no_stick, no_stick_untag);
        let _ = tag_window(axis, min, min_untag);
        let _ = tag_window(axis, max, max_untag);
    }
}

pub fn tag_window_once(axis: &str, pos: &str, untag: bool) {
    let conf =  CONFIG_DATA.read().unwrap().clone();
    if conf.detect_padding && conf.stick_to_borders && !conf.resize_through_borders {
        let _ = tag_window(axis, pos, untag);    
    }
}


pub fn get_tags() -> Vec<String>  {
    println!("Get tags");
    let output = std::process::Command::new("/usr/bin/hypr_tags")
        .output()
        .expect("failed to run '/usr/bin/hypr_tags'. Is it installed?");
    let string_output = String::from_utf8(output.stdout).unwrap();

    println!("Gotten TAGS as string: {}", string_output);
    let cut_output = string_output.split('\n').collect::<Vec<&str>>()[0];

    cut_output.split(", ").map(|x| x.to_string()).collect::<Vec<String>>()
}


pub fn tag_window(axis: &str, pos: &str, untag: bool) {
        let tag = &format!("{}_{}", axis, pos);
        let tag_arg = match untag {
            true =>  &format!("-{}", tag),
            false => &format!("+{}", tag)
        };


        if  !untag {
            println!("Tagging {}", tag_arg);
        } else if untag {
            println!("Untagging {}", tag_arg);
        }

        // let pass_arg = format!("-- {tag_arg}");
        let dispatcher = hyprland::dispatch::DispatchType::Custom("tagwindow", tag_arg.as_str());

        let _ = Dispatch::call(dispatcher);


        println!("The tag: {}", tag);
}


pub fn notify_error(message: String)  {
    let _ = hyprland::ctl::notify::call(
        Icon::Error,
        time::Duration::from_secs(10),
        hyprland::ctl::Color::new(100, 50, 70 ,50) ,
        format!("Hyprfloat: {}", message)
    );
}

fn get_parameter(axis: &str, map: Arc<RwLock<HashMap<String, i16>>>, default_value: i16) -> i16 {
    let binding = map.read().unwrap();
    let result = binding.get(axis);
    match result {
        Some(_) => {
            *result.unwrap()
        },
        None => {
            default_value
        }
    }
}


fn compare_size_parameters(axis: &str) -> i16 {
    let monitor = MONITOR_DATA.read().unwrap().clone();
    let x_resolution = monitor.axis_data.get("x").unwrap().clone().resolution;
    let binding = PARAMETERS.read().unwrap().clone().size.unwrap_or_else(|| HashMap::new());
    let mut output = get_parameter(
        axis,
        Arc::new(RwLock::new(binding.clone())),
        CLIENT_DATA.read().unwrap().clone().axis_data.get(axis).unwrap().window_size
    );
    
    if binding.contains_key(axis) {
        output = *binding.get(axis).unwrap();
    } else if PARAMETERS.read().unwrap().count_system == *"origin" &&
        binding.contains_key(format!("origin_{}", axis).as_str()) {
        output = *binding.get(format!("origin_{}", axis).as_str()).unwrap();
    } else if output <= 20 && PARAMETERS.read().unwrap().clone().origin_size {
        output = ((x_resolution as f32 / 4f32) * 1.6 ).round() as i16;
    }

    output
}


fn move_to_corner(
    min_point: i16,
    max_point: i16,
    resolution: i16,
    cursor_pos: i16,
    window_size: i16
) -> i16 {
    if cursor_pos <= (resolution / 2 ) + min_point {
        min_point
    } else {
        max_point - window_size
    }
}


pub fn custom_round(x: f32) -> i32 {
    if (0.0..0.5).contains(&x) {
        x.floor() as i32
    } else {
        x.round() as i32
    }
}


fn if_last_char_marks_percent(axis: &str, s: &str) -> Option<i16> {
    let monitor = &MONITOR_DATA.read().unwrap();
    let monitor_axis = &monitor.axis_data.get(axis).unwrap();
    let _conf = &CONFIG_DATA.read().unwrap();
    let layers = &LAYERS_DATA.read().unwrap();
    let layers_axis = &layers.get(axis).unwrap();
    

    match s.chars().last() {
        Some('%') => {
            let trimmed_s = &s[..s.len() - 1];
            let output = f32::from_str(trimmed_s).
                map_err(|e| format!("Parse error after removing '%': {}", e))
                .unwrap() * (monitor.axis_data.get(axis).unwrap().resolution as f32 / 100f32);

            Some(custom_round(output) as i16)
        },
        Some('$') => {
            let trimmed_s = &s[..s.len() - 1];
            let output = (f32::from_str(trimmed_s).
                map_err(|e| format!("Parse error after removing '$': {}", e))
                .unwrap() *
                ((monitor_axis.resolution as f32 - (layers_axis.padding_min + layers_axis.padding_max)  as f32)
                    / 100f32)
            );
            println!("Resolution: {:?}", monitor_axis.resolution as f32 - (layers_axis.padding_min + layers_axis.padding_max)  as f32);
            // println!("Output: {}", output);

            Some(custom_round(output) as i16)
        },
        _ => {
            None
        }
    }
}


pub trait IfPercent {
    fn check_percent(self, axis: &str) -> i16;
}


impl IfPercent for toml::value::Value {
    fn check_percent(self, axis: &str) -> i16 {
        let value = self;
        
        let output = match value.as_str() {
            Some(s) => {
                match if_last_char_marks_percent(axis, s) {
                    Some(c) => c,
                    None => value.as_integer().unwrap() as i16
                }
            },
            None => {
                value.as_integer().unwrap() as i16
            }
        };

        output
    }
}

impl IfPercent for String {
    fn check_percent(self, axis: &str) -> i16 {
        let value = self;

        let output = match i32::from_str(value.as_str()) {
            Ok(x) => {
                x as i16
            },
            Err(e) => {
                match if_last_char_marks_percent(axis, value.as_str()) {
                    Some(c) => c,
                    None => {
                        eprintln!("Cannot parse value: {}, Error: {}", value, e);
                        exit(1)
                    }
                }
            }
        };

        output
    }
}


pub fn position(axis: &str) -> Result<i16, String> {
    update_data();
    let conf = &CONFIG_DATA.read().unwrap();
    let layers = &LAYERS_DATA.read().unwrap();
    let paddings = &layers.get(axis).unwrap();
    let conf_axis = &conf.axis_data.get(axis).unwrap();
    let monitor = MONITOR_DATA.read().unwrap().clone();
    let y_resolution = monitor.axis_data.get("y").unwrap().resolution;
    let x_resolution = monitor.axis_data.get("x").unwrap().resolution;
    let monitor_axis = &monitor.axis_data.get(axis).unwrap();
    let cli = &CLIENT_DATA.read().unwrap();
    let cli_axis = &cli.axis_data.get(axis).unwrap();
    let params = PARAMETERS.read().unwrap().clone();
    let at_params = params.at.clone().unwrap_or_default();
    let size_params: HashMap<String, i16> = params.size.clone().unwrap_or_default();
    let system = params.count_system.as_str();
    let dispatcher_arg = params.position.as_str();

    let mut window_offset  = -cli_axis.window_size;

    if system == "origin" {
        window_offset = -10;
    }

    if params.count_system == "origin" &&
        size_params.clone().contains_key(format!("origin_{}", axis).as_str()) {
        println!("default: origin");
        window_offset = *size_params.get(format!("origin_{}", axis).as_str()).unwrap() / -2;

    } else if size_params.clone().contains_key(axis) {
        println!("default: Key axis");
        window_offset = *size_params.get(axis).unwrap() / -2;
    } else if params.default_size {
        println!("default: Default size");
        window_offset = conf_axis.default_size / -2
    }  else if size_params.clone().contains_key(axis)  {
        println!("default: Contains config_size");
        window_offset = *size_params.get(axis).unwrap() / -2;
    } else if system == "origin" {
        println!("Default");
        window_offset = ((y_resolution as f32 / -8f32) * 1.6).round() as i16;
    } else {
        // window_offset = cli_axis.window_size / -2 ;
        println!("Default start offset:{}", window_offset);
    }

    
    println!("Count system: {}", params.count_system);
    println!("Window offset: {}", window_offset);

    let mut monitor_offset = 0;
    if params.count_system == "origin" {
        monitor_offset = monitor_axis.min_point
    }
    // else {
    //     exit(1)
    // }


    println!("Monitor offset: {}", monitor_offset);

    let mut min_point = monitor_axis.min_point - monitor_offset;
    let mut max_point = monitor_axis.max_point - monitor_offset;
    let mut cursor_pos = cli_axis.cursor_pos - monitor_offset;
    let window_position = cli_axis.window_pos - monitor_offset;

    let mut resolution = monitor_axis.max_point - monitor_axis.min_point;
    let mut window_size = cli_axis.window_size;

    if conf.detect_padding {
        min_point += paddings.padding_min;
        max_point -= paddings.padding_max;
        resolution += paddings.padding_min - paddings.padding_max;
        cursor_pos += paddings.padding_min;
    }


    if params.count_system == "position" {
        println!("min, Max: c{:?}", (min_point, max_point));
    }

    if  system == "position" {
        window_offset = cli_axis.window_size / -2;
    }


    let mut position : Result<i16, String> = match dispatcher_arg {
        "l" | "left"          => {
            let mut output = min_point;
            if axis == "y"   {
                println!("Axis y in move Left");
                tag_it(axis, "no_stick");
                output = (max_point + min_point + window_offset * 2) / 2
            }
            Ok(output)
        },
        "r" | "right"         => {
            let mut output = max_point + window_offset * 2;
            if axis == "y" {
                tag_it(axis, "no_stick");
                output = (max_point + min_point + window_offset * 2) / 2
            }
            Ok(output)
        },
        "t" | "top"           => {
            let mut output = min_point;
            if axis == "x" {
                tag_it(axis, "no_stick");
                output = (max_point + min_point + window_offset * 2) / 2
            }
            Ok(output)
        },
        "b" | "bottom"        => {
            let mut output = max_point + window_offset * 2;
            if axis == "x" {
                tag_it(axis, "no_stick");
                output = (max_point + min_point + window_offset * 2) / 2
            }
            Ok(output)
        },
        "tl" | "top-left"     => {
            tag_it(axis, "min");
            Ok(min_point)
        },
        "tr" | "top-right"    => {
            let mut output = min_point;
            if axis == "x" {
                tag_it(axis, "max");
                output = max_point + window_offset * 2
            } else {
                tag_it(axis, "min");
            }
            Ok(output)
        },
        "bl" | "bottom-left"  => {
            let mut output = min_point;
            if axis == "y" {
                tag_it(axis, "max");
                output = max_point + window_offset * 2
            } else {
                tag_it(axis, "min");
            }
            Ok(output)
        },
        "br" | "bottom-right" => {
            tag_it(axis, "max");
            Ok(max_point + window_offset * 2)
        },
        "center"   => {
            tag_it(axis, "no_stick");
            // println!("max point: {}", max_point);
            // println!("min point: {}", min_point);
            // println!("Window pos: {}", (max_point as f32 + min_point as f32 - window_offset as f32 * -2f32) / 2f32);
            // println!("window_size {}", window_offset * -2);
            
            Ok(custom_round((max_point as f32 + min_point as f32 - window_offset as f32 * -2f32) / 2f32) as i16)
            // Ok((max_point + min_point - window_offset * -2) / 2)
        },
        "close"    => {
            Ok(move_to_corner(min_point, max_point, resolution, cursor_pos, window_offset * -2))
        },
        "far"      => {
            if conf.detect_padding {
                resolution = monitor_axis.max_point - monitor_axis.min_point + paddings.padding_min;
                cursor_pos = cli_axis.cursor_pos - monitor_axis.min_point + paddings.padding_max - paddings.padding_min;
            }

            Ok(move_to_corner(min_point, max_point, resolution, resolution - cursor_pos, window_offset * -2))
        },
        "cursor"   => {
            if conf.detect_padding {
                cursor_pos -= paddings.padding_min
            }
            println!("Axis: {}, Window offset {}, cursor pos {}", axis, window_offset, cursor_pos);
            Ok(cursor_pos + window_offset)
        },
        "opposite" => {
            if conf.detect_padding {
                cursor_pos -= paddings.padding_min;
            }
            Ok(max_point - cursor_pos  + min_point + window_offset)
        },
        "random"   => {
            if system == "position" && params.binary == "hfopen" {
                if params.origin_size {
                    Ok(window_position + window_offset)
                } else {
                    Ok(window_position)
                }
            } else {
                let mut rng = rand::rng();
                let mut range_min_point = min_point;
                let mut range_max_point = max_point + window_offset * 2;
                if range_min_point > range_max_point {
                    std::mem::swap(&mut range_min_point, &mut range_max_point)
                }
                
                Ok(rng.random_range(range_min_point..=range_max_point))

            }
        },
        _          => {
            if system == "position" {
                if params.origin_size &&
                    size_params.contains_key(axis)
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

    // println!("Position:{}", position.clone().unwrap());

    if at_params.contains_key(axis) {
        position = Ok(*at_params.get(axis).unwrap());

        println!("At contains: {:?}", position.clone());
       
        if system == "origin" && size_params.is_empty() && !params.size_from_args {
            let mut k = 1;
            if !params.origin_size {
                k = 2
            }

            position = Ok(
                *at_params.get(axis).unwrap() -
                    (x_resolution as f32 / -12f32 * 1.6).round() as i16 / k
            );
        }
    } else {
        window_size = window_offset * -2;

        let mut output = position?;
        if output <= min_point {
            output = min_point
        } else if output + window_size >= max_point {
            output = max_point - window_size
        }
        
        position = Ok(output);
    }

    println!("Position Inside {} :{}", axis, position.clone().unwrap());

    position
}


pub fn window_position() -> DispatchType<'static> {
    let x = position("x").unwrap();
    let y = position("y").unwrap();
    println!("Positions of {}:: x:{}, y:{}", PARAMETERS.read().unwrap().clone().count_system, x, y);

    if PARAMETERS.read().unwrap().count_system == "origin" {
        PARAMETERS.write().unwrap().count_system = "position".to_string();
    }

    DispatchType::MoveActive(Exact(
        x,y
    ))
}


pub fn dispatch_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dispatching client");

    update_data();

    let cli = CLIENT_DATA.read()?.clone();

    let conf = CONFIG_DATA.read()?.clone();
    let params = PARAMETERS.read()?.clone();

    if params.binary == "hftogglefloating" && cli.floating && !params.default_size
        && !conf.toggle_to_open_size  {
        notify_error("Write from toggle".to_string());
        write_class_data_into_cache(
            BIN.read()?.clone(),
            CLASS.read()?.clone()
        ).unwrap_or_else(|e| {
            eprintln!("Error writing class data to cache: {}", e);
        });
    }
    

    let mut toggled_float = false;
    if (params.binary == "hfopen" && params.tile == cli.floating) ||
        params.binary == "hftogglefloating" {
        toggled_float = true;
        let _ = Dispatch::call(ToggleFloating(None));
    }

    if !Client::get_active().unwrap().unwrap().floating { exit(0x0100) }

    if params.default_size {
        let mut map: HashMap<String, i16> = HashMap::new();

        map.insert("x".to_string(), conf.axis_data.get("x").unwrap().default_size);
        map.insert("y".to_string(), conf.axis_data.get("y").unwrap().default_size);

        PARAMETERS.write()?.size = Some(map);

    } else if !params.size_from_args {
        notify_error("read from config/cache".to_string());
        match predefine_size_by_class(cli.clone().class) {
            Ok(tuple) => {
                let mut map: HashMap<String, i16> = HashMap::new();

                map.insert("x".to_string(), tuple.0);
                map.insert("y".to_string(), tuple.1);

                PARAMETERS.write()?.size = Some(map);
            },
            Err(error) => {
                println!("No data in cache or config on post origin stage: {}", error);
            }
        };
    }

    let _ = Dispatch::call(ResizeActive(Exact(
        compare_size_parameters("x"),
        compare_size_parameters("y"),
    )));


    update_data();
    
    if params.binary == "hfopen" {
        println!("Write from open");
        let _ = write_class_data_into_cache(
            BIN.read()?.clone(),
            CLASS.read()?.clone()
        ).unwrap_or_else(|e| {
            eprintln!("Error writing class data to cache: {}", e);
        });
    }

    let _ = Dispatch::call(window_position());

    Ok(())
}


pub fn origin_position(axis: &str) -> String {
    let result = position(axis);

    if let Ok(x) = result {
        x.to_string()
    } else {
        "".to_string()
    }
}


pub fn change_window_state() {
    println!("Change state");

    PARAMETERS.write().unwrap().count_system = "position".to_string();
    let params = PARAMETERS.read().unwrap().clone();

    if params.binary == "hfopen".to_string() {
        println!("Event");
        let mut event = hyprland::event_listener::EventListener::new();

        event.add_window_opened_handler(
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

        for _i in 0..=30 {
            let current_addr = Client::get_active()
                .unwrap()
                .unwrap_or(
                    empty_client()
                ).address;
            
            if (current_addr != start_addr && params.binary == "hfopen") ||
                (current_addr == start_addr && params.binary == "hftogglefloating") {

                let _ = dispatch_client();
                break
            }
            sleep(time::Duration::from_millis(50));
        }
    }
}

