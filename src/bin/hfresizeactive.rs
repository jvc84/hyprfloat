use std::{
    env,
    process::exit
};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
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
use hyprfloat::{
    CONFIG_FILE,
    XDG_PATH,
    CONFIG_DATA,
    CLIENT_DATA,
    get_table,
    notify_error,
    config_data,
    common_help
};


lazy_static!(
    static ref ARGS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(env::args().collect()));
    static ref BORDERS_PARAM: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    static ref INVERT_PARAM: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    static ref EXACT: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
);


fn count_move_resize(axis: &str, global_resize: i16) -> (i16, i16) {
    let cli = CLIENT_DATA.read().unwrap().clone();
    let cli_axis = cli.axis_data.get(axis).unwrap();
    let conf = CONFIG_DATA.read().unwrap().clone();
    let conf_axis =conf.axis_data.get(axis).unwrap();
    
    let class = cli.class;
    let table = get_table("windows", CONFIG_FILE.clone().as_str());
    let list: toml::Table = table.as_table().unwrap().clone();
    let mut minimal_size: i16 = 20;
    
    if list.keys().collect::<Vec<_>>().contains(&&class.clone()) {
        let class_section = list[&class].clone();
        let value = "minimal_size".to_string();

        if class_section
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .contains(&&value.clone()) {
            let param_vec = &class_section.as_table().unwrap()[&value];

            let param = match axis {
                "x" => param_vec[0].as_integer().unwrap().clone() as u16,
                "y" => param_vec[1].as_integer().unwrap().clone() as u16,
                 _  => {
                    notify_error(format!("No such axis: {}", axis).as_str());
                    exit(0x0100);
                }
            };

            if param as i16 > minimal_size  {
                minimal_size = param as i16
            }
        }
    }

    let mut resize = global_resize;

    let border_position_min = cli_axis.window_pos;
    let border_position_max = cli_axis.window_pos + cli_axis.window_size;

    let mut padding_min = cli_axis.monitor_min_point + conf_axis.padding_min;
    let mut padding_max = cli_axis.monitor_max_point - conf_axis.padding_max;

    let mut window_size = cli_axis.window_size;
    let mut window_pos = cli_axis.window_pos;
    let window_start_pos = window_pos;
    let window_start_size = window_size;
    
    let mut margin = 0;
    if conf.stick_to_borders == true {
        margin = conf_axis.margin;
        if margin < 2 {
            margin = 2
        }
    }
    
    if  cli.floating == true && conf.detect_padding == true &&
        conf.invert_resize_in_stick_mode == true && conf.resize_through_borders == false &&
        conf.stick_to_borders == true &&
        ((window_pos + window_size >= padding_max) && window_pos > padding_min)  &&
        ((window_pos + window_size <= padding_max) && window_pos >= padding_min) {
        resize = -global_resize
    }
    
    if margin > resize {
        margin = resize
    }
    
    let working_area = padding_max - padding_min - margin;
    
    if ( window_start_size <= minimal_size || window_start_size + resize <= minimal_size) && resize < 0 {
        resize = -window_start_size + minimal_size;
    }

    if -resize > window_size {
        window_size = minimal_size;
        if conf.standard_resize == false {
            window_pos = window_start_pos + window_start_size / 2 - minimal_size / 2;
        }
    } else {
        if CONFIG_DATA.read().unwrap().standard_resize == false {
            window_pos = window_pos - resize / 2;
            window_size = window_size + resize
        } else {
            window_size = window_size + resize
        }

    }

    if EXACT.read().unwrap().clone() == true {
        if conf.detect_padding == true && conf.resize_through_borders == false &&
            ((window_pos <= padding_min && window_pos + window_size >= padding_max) &&
                border_position_max - border_position_min < padding_max - padding_min )
        {
            let distance_min = window_start_pos - padding_min;
            let distance_max = padding_max - (window_start_pos + window_start_size);

            if distance_min <= distance_max {
                padding_max = window_start_pos + window_start_size + resize;
            } else {
                padding_min = window_start_pos - resize;
            }
        }
    }

    if conf.detect_padding == true && conf.resize_through_borders == false {
        if  window_size >= working_area && 
            ( window_start_size <= working_area || window_start_size <= padding_max - padding_min)
            && EXACT.read().unwrap().clone() == false {

            window_size = working_area;
            
            if window_start_pos <= padding_min {
                window_pos = window_start_pos;
            } else if window_start_pos + window_start_size >= padding_max {
                window_pos = padding_min + margin;
            } else {
                window_pos = padding_min + margin / 2;
            }
            
        } else {
            let distance_min = border_position_min - padding_min;
            let distance_max = padding_max - border_position_max;

            if distance_min <= distance_max || window_start_size >= working_area {
                if conf.stick_to_borders == true {
                    if  window_pos <= padding_min || window_start_pos <= padding_min {
                        window_pos = padding_min
                    } 
                } else if  window_pos <= padding_min {
                    window_pos = padding_min
                } 
            } else{
                if conf.stick_to_borders == true {
                    if window_pos + window_size  >= padding_max || window_start_pos + window_start_size >= padding_max {
                        window_pos = padding_max - window_size;
                    }
                } else if window_pos + window_size >= padding_max {
                    window_pos = padding_max - window_size;
                }
            }
        }
    }

    (window_pos, window_size)
}


fn resizeactive_help() {
    println!("\
    \nUSAGE:\
    \n\
    \n    hfresizeactive [ARGUMENTS] RESIZE_X RESIZE_Y\
    \n\
    \nARGUMENTS:\
    \n\
    {}\
    \n    -e, --exact                 - make size of floating window exactly RESIZE_X pixels on x axis, RESIZE_Y pixels on y axis\
    \n    -n, --no-invert             - do not invert resize in stick mode, even if `invert_resize_in_stick_mode` option in config equals `true`\
    \n\
    \nRESIZE_X       - resize window by x axis on RESIZE_X pixels according to config parameters\
    \nRESIZE_Y       - resize window by y axis on RESIZE_Y pixels according to config parameters\
    \n\
    \nDEFAULT CONFIG PATH:\
    \n\
    \n    `$HOME{}`
    ",
             common_help(),
             XDG_PATH.as_str()
    );

    exit(0x0100);
}


fn get_resize(axis: &str, negative_index: usize) -> i16 {
    let cli =  CLIENT_DATA.read().unwrap();
    let conf =  CONFIG_DATA.read().unwrap();
    let args = ARGS.read().unwrap();

    let index = args.len() - negative_index;
    let mut resize = args[index].clone().parse::<i16>().unwrap();

    if *EXACT.read().unwrap() == true {
       resize - cli.axis_data.get(axis).unwrap().window_size
    } else {
        
        if resize % 2 != 0 && conf.standard_resize == false && cli.floating == true {
            resize = resize + 1;
        }
        
        resize
    }
}


fn main() {
    *BORDERS_PARAM.write().unwrap() = CONFIG_DATA.read().unwrap().resize_through_borders.clone();
    *INVERT_PARAM.write().unwrap() = CONFIG_DATA.read().unwrap().invert_resize_in_stick_mode.clone();

    let args: Vec<String> = ARGS.read().unwrap().clone();

    let mut no_inverting = false;
    let mut resize_through_borders = false;

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-h" | "--help" => resizeactive_help(),
            "-c" | "--config" => *CONFIG_DATA.write().unwrap() = config_data(args[i + 1].to_string()),
            "-n" | "--no-invert" => no_inverting = true,
            "-e" | "--exact" => {
                *EXACT.write().unwrap() = true;
                no_inverting = true
            },
            "-f" | "--force" => resize_through_borders = true,
            _ => {
                if args.len() < 3 {
                    resizeactive_help()
                }
            }
        }
    }

    if no_inverting == true {
        CONFIG_DATA.write().unwrap().invert_resize_in_stick_mode = false;
    }
    if resize_through_borders == true {
        CONFIG_DATA.write().unwrap().resize_through_borders = true;
    }
    
    let cli =  CLIENT_DATA.read().unwrap();
    
    let resize_x = get_resize("x", 2);
    let resize_y = get_resize("y", 1);
    

    if cli.floating == true {
        let (position_x, resize_x) = count_move_resize("x", resize_x);
        let (position_y, resize_y) = count_move_resize("y", resize_y);
        
        
        let _ =Dispatch::call(
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
