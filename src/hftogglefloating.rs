use std::collections::HashMap;
use std::process::exit;
use crate::{change_window_state, update_data, POSITION_VALUES, PARAMETERS, CONFIG_DATA, get_daemon_data, GIVE_ARGS, hftogglefloating};
use clap::Parser;


pub fn togglefloating(togglefloating_args: crate::Modes) {
    let parsed_args = match togglefloating_args {
        crate::Modes::Togglefloating {
            force,
            default_size,
            size,
            at,
            position,
        } => crate::TogglefloatingArgs {
                force,
                default_size,
                size,
                at,
                position,
            },
        _ => panic!("Cannot parse Togglefloating enum into TogglefloatingArgs"),
    };

    PARAMETERS.write().unwrap().binary = "hftogglefloating".to_string();

    
    match get_daemon_data(){
        Ok(_) => {},
        Err(e) => {
            println!("Cannot get daemon data: {}", e);
            exit(1)
        }
    };
    
    PARAMETERS.write().unwrap().position = parsed_args.position;
    PARAMETERS.write().unwrap().default_size = parsed_args.default_size;
    if parsed_args.default_size {
        PARAMETERS.write().unwrap().origin_size = true
    }

    if parsed_args.force {
        CONFIG_DATA.write().unwrap().detect_padding = false
    }

    if parsed_args.size.len() > 0 {
        let list = [("x", parsed_args.size[0]), ("y", parsed_args.size[1])];
        PARAMETERS.write().unwrap().origin_size = true;
        let mut map: HashMap<String, i16> = HashMap::new();

        for i in list {
            map.insert(i.0.to_string(), i.1 as i16);
        }

        PARAMETERS.write().unwrap().size = Some(map);
    }

    if parsed_args.at.len() > 0 {
        let list = [("x", parsed_args.at[0]), ("y", parsed_args.at[1])];
        let mut map: HashMap<String, i16> = HashMap::new();
        
        for i in list {
            map.insert(i.0.to_string(), i.1);
        }
        
        PARAMETERS.write().unwrap().at = Some(map);
    }

    update_data();
    let _ = change_window_state();
}