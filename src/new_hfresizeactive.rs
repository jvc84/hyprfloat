use std::{process::exit, thread};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use hyprland::{
    dispatch::{
        Dispatch,
        DispatchType,
        Position::{
            Delta,
            Exact
        },
    }
};
use crate::{get_tags, tag_window_once, notify_error, update_data, IfPercent, TAGS, CONFIG_FILE, CONFIG_DATA, CLIENT_DATA, EXACT, MONITOR_DATA, get_daemon_data, LAYERS_DATA, GIVE_ARGS, monitor_data, config_data, layers_data, position, WINDOWS_CONFIG_DATA, tag_it, compare_size_parameters, ResizeactiveArgs};
use clap::{Command, Parser};
use hyprland::data::Client;


lazy_static!(
    static ref BORDERS_PARAM: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    static ref INVERT_PARAM: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
);


fn count_move_resize(axis: &str, global_resize: i16, conf_minimal_size: u16) -> (i16, i16) {
    let monitor = MONITOR_DATA.read().unwrap().clone();
    let layers = LAYERS_DATA.read().unwrap().clone();
    let paddings = layers.get(axis).unwrap();
    let monitor_axis = monitor.axis_data.get(axis).unwrap().clone();
    let cli = CLIENT_DATA.read().unwrap().clone();
    let cli_axis = cli.axis_data.get(axis).unwrap();
    let conf = CONFIG_DATA.read().unwrap().clone();
    let conf_axis =conf.axis_data.get(axis).unwrap();


    let mut minimal_size: i16 = 20;

    if conf_minimal_size as i16 > minimal_size  {
        minimal_size = conf_minimal_size as i16
    }


    let mut resize = global_resize;
    let exact = EXACT.read().unwrap().clone();

    // let border_position_min = cli_axis.window_pos;
    // let border_position_max = cli_axis.window_pos + cli_axis.window_size;

    let mut padding_min = monitor_axis.min_point + paddings.padding_min;
    let mut padding_max = monitor_axis.max_point - paddings.padding_max;

    let mut window_size = cli_axis.window_size;
    let mut window_pos = cli_axis.window_pos;
    let window_start_pos = window_pos;
    let window_start_size = window_size;
    
    let mut margin = 0;
    if conf.stick_to_borders {
        margin = conf_axis.margin
    }

    let stick_min = &format!("{}_min", axis);
    let stick_max = &format!("{}_max", axis);
    let no_stick =  &format!("{}_no_stick", axis);


    let mut tags = TAGS.read().unwrap().clone();


    if conf.detect_padding && conf.stick_to_borders && window_size < padding_max - padding_min{
        if window_start_pos <= padding_min {
            // tag_it(axis, "min");
            tags = vec![stick_min.to_string()];

        } else if window_start_pos + window_start_size >= padding_max {
            // tag_it(axis, "max");
            tags = vec![stick_max.to_string()];


        } else if window_start_pos > padding_min && window_start_pos + window_start_size < padding_max {
            println!(":: start no stick");
            // tag_it(axis, "no_stick");
            tags = vec![no_stick.to_string()];
        }

    }


    if cli.floating && conf.detect_padding && conf.invert_resize_in_stick_mode && conf.stick_to_borders
        && !conf.resize_through_borders && !exact && (window_pos + window_size >= padding_max)
        && !tags.contains(&stick_min) && !tags.contains(&no_stick) {
         println!(":: Invert resize");
        resize = -global_resize
    }

    if margin > resize {
        margin = resize;
    }


    if ( window_start_size <= minimal_size ||
        window_start_size + resize <= minimal_size)
        && resize < 0 && !exact {
        resize = minimal_size - window_start_size;
    }


    if -resize > window_size {
        window_size = minimal_size;
        if !conf.standard_resize {
            window_pos = crate::custom_round(window_start_pos as f32 + window_start_size as f32 / 2f32 - minimal_size as f32 / 2f32) as i16;
        }
    } else {
        if !CONFIG_DATA.read().unwrap().standard_resize {
            window_pos = crate::custom_round(window_pos as f32 - resize as f32 / 2f32) as i16;
            window_size = window_size + resize
        } else {
            window_size = window_size + resize
        }

    }


    let working_area = padding_max - padding_min - margin;


    if conf.detect_padding {
        if window_size >= working_area  && !exact {
            window_size = working_area;
            window_pos = padding_min;
        }

    }


    if tags.contains(&no_stick) && conf.detect_padding && conf.stick_to_borders && !conf.resize_through_borders {
        if window_pos <= padding_min && window_pos + window_size < padding_max && window_size < working_area {
            println!(":: HEY Stick min");
            // tag_it(axis, "min");
            tags = vec![stick_min.to_string()];

        }
        else if (window_pos + window_size >= padding_max && window_pos > padding_min)
            // && window_size != working_area
        {
            // tag_it(axis, "max");
            println!(":: HEY Stick max");
            println!(":: window po {}, padding min {}", window_pos, padding_min);
            tags = vec![stick_max.to_string()];

        }
    }


    if (window_pos < padding_min && window_pos + window_size > padding_max) ||
        window_size == working_area && !tags.contains(stick_min) && !tags.contains(&stick_max) {

        tag_it(axis, "no_stick");
        tags = vec![no_stick.to_string()];

    } else if conf.detect_padding && !conf.resize_through_borders {
        if conf.stick_to_borders {

            // for i in tags.clone() {
            //     if &i != no_stick {
            //         tags.push(i.to_string());
            //     }
            // }

            // tags.retain(|x| x==no_stick);

            if !tags.contains(&no_stick)
                {
                if (window_pos <= padding_min || window_start_pos <= padding_min) &&
                    !tags.contains(&stick_max) {
                    println!(":: Stick min!!");
                    tag_it(axis, "min");
                    tags = vec![stick_min.to_string()];

                } else if (window_pos + window_size >= padding_max ||
                    window_start_pos + window_start_size >= padding_max) &&
                    !tags.contains(&stick_min) {
                    tag_it(axis, "max");

                    tags = vec![stick_max.to_string()];

                }
            }
        } else {
            tag_it(axis, "no_stick");

            tags = vec![no_stick.to_string()];

        }
    }


    if exact && conf.detect_padding  && !conf.resize_through_borders  &&
            window_size >= working_area
        {
        let working_area = padding_max - padding_min; // - margin;

        let distance_min = (window_start_pos - padding_min);
        let mut distance_max = padding_max - (window_start_pos + window_start_size);

        if distance_min - distance_max == 1 {
            distance_max -= 1
        } else if distance_max - distance_min == 1 {
            distance_max -= 1
        }


        if window_size == working_area {
            tag_it(axis, "no_stick");
            tags = vec![no_stick.to_string()];

            window_pos =  padding_min;
        } else if distance_min < distance_max {
            tag_it(axis, "min");
            tags = vec![stick_min.to_string()];

            padding_max = window_start_pos + window_start_size + resize;
        } else if distance_max < distance_min {
            tag_it(axis, "max");
            tags = vec![stick_max.to_string()];

            padding_min = window_start_pos - resize;

        } else {
            tag_it(axis, "no_stick");
            tags = vec![no_stick.to_string()];

            window_pos = padding_min; // + ( working_area - window_size)  / 2;
            
            if window_size > working_area {
                window_pos += (working_area  - window_size) / 2
            }
        }
    }


    if conf.detect_padding  && !conf.resize_through_borders {
         println!(":: detect");
        if !tags.contains(&no_stick) && conf.stick_to_borders {
             println!(":: do stick");
            if tags.contains(&stick_min) && !tags.contains(&stick_max) {
                 println!(":: stick min");
                window_pos = padding_min
            } else if tags.contains(&stick_max)  {
                 println!(":: stick max");
                window_pos = padding_max - window_size
            }
        } else {
            println!(":: Just detect");
            if window_size == working_area {
                window_pos = padding_min + margin / 2;
            } else if window_pos <= padding_min  && !conf.stick_to_borders{
                window_pos = padding_min
            } else if window_pos + window_size >= padding_max  && !conf.stick_to_borders{
                window_pos = padding_max - window_size
            }
        }
    }

    (window_pos, window_size)
}


fn get_resize(axis: &str, resize: i16) -> i16 {
    let cli = &CLIENT_DATA.read().unwrap();

    match *EXACT.read().unwrap() {
        true =>  resize - cli.axis_data.get(axis).unwrap().window_size,
        false => resize
    }
}


pub fn resizeactive(resizeactive_args: crate::Modes) {
    *BORDERS_PARAM.write().unwrap() = CONFIG_DATA.read().unwrap().detect_padding.clone();
    *INVERT_PARAM.write().unwrap() = CONFIG_DATA.read().unwrap().resize_through_borders.clone();

    
    let parsed_args =  match resizeactive_args {
        crate::Modes::Resizeactive {
            resize_x,
            resize_y,
            force,
            no_invert,
            exact,
        }         => ResizeactiveArgs {
                resize_x,
                resize_y,
                force,
                no_invert,
                exact,
            },
        _ => panic!("Could parse Resizeactive enum into ResizeactiveArgs")
    }; 

    let mut no_inverting = false;
    let mut resize_through_borders = false;
    
    if parsed_args.no_invert {
        no_inverting = true;
    }
    if parsed_args.exact {
        *EXACT.write().unwrap() = true;
        no_inverting = true;
    }
    if parsed_args.force {
        resize_through_borders = true
    }

    update_data();
    match get_daemon_data() {
        Ok(()) => {
            println!(":: OK");
        },
        Err(e) => {
            eprintln!(":: ERROR: {}", e);
            exit(1);
        }
    }

    if no_inverting {
        CONFIG_DATA.write().unwrap().invert_resize_in_stick_mode = false;
    }
    if resize_through_borders {
        CONFIG_DATA.write().unwrap().resize_through_borders = true;
    }
    

    let cli =  CLIENT_DATA.read().unwrap().clone();
    
    use hyprland::shared::*;
    println!("Client data: {:#?}", Client::get_active().unwrap().unwrap());

    *TAGS.write().unwrap() = cli.tags;

    println!(":: Outer tags: {:?}", TAGS.read().unwrap().clone());


    let class = cli.class;
    let list = WINDOWS_CONFIG_DATA.read().unwrap().clone();
    let mut param_tup: (u16, u16)  = (0,0);


    let resize_x = get_resize("x", parsed_args.resize_x.check_percent("x"));
    let resize_y = get_resize("y", parsed_args.resize_y.check_percent("y"));

    println!(":: Resize x: {}, y: {}", resize_x, resize_y);


    if cli.floating {

        if list.keys().collect::<Vec<_>>().contains(&&class.clone()) {
            let class_section = list[&class].clone();
            let value = "minimal_size".to_string();

            if class_section
                .as_table()
                .unwrap()
                .keys()
                .collect::<Vec<_>>()
                .contains(&&value.clone()) {
                let section_value = class_section.
                    as_table()
                    .unwrap()[&value]
                    .as_array()
                    .unwrap();
                param_tup.0 = section_value[0].as_integer().unwrap() as u16;
                param_tup.1 = section_value[1].as_integer().unwrap() as u16;
            }
        }


        let (position_x, resize_x) = count_move_resize("x", resize_x, param_tup.0);
        let (position_y, resize_y) = count_move_resize("y", resize_y, param_tup.1);

        let _ = Dispatch::call(
            DispatchType::MoveActive(
                Exact(position_x, position_y),
            )
        );

        let _ = Dispatch::call(
            DispatchType::ResizeActive(
                Exact(resize_x, resize_y),
            )
        );

    } else {
        let _ = Dispatch::call(
            DispatchType::ResizeActive(
                Delta(resize_x, resize_y),
            )
        );
    }
}
