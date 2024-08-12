pub mod client;
use hyprland::prelude::*;
use std::{env, fs, time};
use std::collections::HashMap;
use std::ffi::CString;
use std::process::{Command, Stdio, exit};
use std::thread::sleep;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::dispatch::DispatchType::{ResizeActive, ToggleFloating};
use hyprland::dispatch::Position::Exact;
use hyprland::shared::Address;
use toml;
use serde::Deserialize;
use rand::Rng;
use std::ops::Sub;
use hyprland::event_listener::EventListener;
// use getopts;
use hyprland::data::{Client, CursorPosition, Monitor, WorkspaceBasic};
use lazy_static::lazy_static;
use log::__private_api::loc;

pub fn parse_fullscreen(arg:hyprland::data::FullscreenMode) -> bool {
    if arg == hyprland::data::FullscreenMode::None {
        return false
    } else {
        return true
    }
}

pub fn client_data() -> FromClient {
    let active_window = Client::get_active()
        .unwrap()
        .unwrap_or(
            Client {
                address: Address::new(
                    "0x1a1a1a1a1a1a".to_string(),
                ),
                at: (500, 500),
                size: (10,10),
                workspace: WorkspaceBasic {
                    id: 4,
                    name: "Empty".to_string(),
                },
                floating: false,
                // fullscreen: false,
                // fullscreen_mode: 0,
                fullscreen: hyprland::data::FullscreenMode::None,
                fullscreen_client: hyprland::data::FullscreenMode::None,
                monitor: 0,
                initial_class: "Empty".to_string(),
                class: "Empty".to_string(),
                initial_title: "Empty".to_string(),
                title: "Empty".to_string(),
                pid: 28823,
                xwayland: true,
                pinned: false,
                grouped: vec![],
                mapped: true,
                focus_history_id: 1,
                swallowing: Some(
                    Box::<Address>::new(Address::new("0x0"))
                )

            }
        );

    let client = FromClient {
        window_pos: Descartes {
            x: active_window.at.0,
            y: active_window.at.1,
        },
        window_size: Descartes {
            x: active_window.size.0,
            y: active_window.size.1,
        },
        screen_size: Descartes {
            x: Monitor::get_active().unwrap().width  as i16,
            y: Monitor::get_active().unwrap().height as i16,
        },
        cursor_pos: Descartes{
            x: CursorPosition::get().unwrap().x as i16,
            y: CursorPosition::get().unwrap().y as i16,
        },
        address: active_window.address,
        floating: active_window.floating,
        // fullscreen: active_window.fullscreen
        fullscreen: crate::client::parse_fullscreen(active_window.fullscreen),
    };


    return client

}

#[derive(Debug, Deserialize)]
pub struct Descartes {
    pub x: i16,
    pub y: i16,
}


#[derive(Debug, Deserialize)]
pub struct FromClient {
    pub window_pos:  Descartes,
    pub window_size: Descartes,
    pub screen_size: Descartes,
    pub cursor_pos:  Descartes,
    pub address: Address,
    pub floating: bool,
    pub fullscreen: bool,
}

#[derive(Debug, Deserialize)]
pub struct Margins {
    pub left:   i16,
    pub top:    i16,
    pub bottom: i16,
    pub right:  i16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub margins: Margins,
    pub size: Descartes,
    pub detect_borders: bool,
    pub classic_resize: bool,
    pub stick_to_borders: bool,
    pub invert_keys_in_stick_mode: bool,
    pub resize_through_borders: bool,
}



#[derive(Debug, Deserialize)]
pub struct Count {
    pub max_pos:    Descartes,
    pub window_center: Descartes,
}


// Define size as argument
pub fn config_data() ->  Config {
    // if let Some(proj_dirs) =
    //     ProjectDirs::from("dev", "jvc84", "Hyprfloat")
    // {
    //     let config_dir = proj_dirs.config_dir();
    //     dbg!(config_dir);
    // }

    // let config_dir = "/home/adex/.config/hyprfloat/".to_string();
    let config_dir = "/home/adex/.config/hyprfloat/config.toml".to_string();

    let config_file = fs::read_to_string(
        // config_dir.join("oconfig.42"),
        config_dir
    ).unwrap_or("no Conf".to_string());

    let config: Config =
        toml::from_str(&config_file).unwrap();
    // dbg!(config)

    // let mut map = HashMap::new();
    // map.insert("config".to_string(), config);

    return config
}



pub fn count_data() -> Count {
    let loc_cli = client_data();
    let loc_conf = config_data();
    let count = Count {
        max_pos: Descartes {
            x: loc_cli.screen_size.x - loc_cli.window_size.x - loc_conf.margins.right,
            y: loc_cli.screen_size.y - loc_cli.window_size.y - loc_conf.margins.bottom,
        },
        window_center: Descartes {
            x: loc_cli.cursor_pos.x - (loc_cli.window_size.x / 2),
            y: loc_cli.cursor_pos.y - (loc_cli.window_size.y / 2),
        }
    };

    return count
}


// static ARGS: std::env::Args =  env::args();

lazy_static! {
    static ref args:  Vec<String> = env::args().collect();
}
pub fn make_args( param: &str) {
    let mut dispatcher_var: &str = &"default";
    let mut  do_float: bool = false;
    let mut toggle_float: bool = false;
    let mut resize: bool = false;
    let mut width: i16 = crate::config_data().size.x;
    let mut height: i16 = crate::config_data().size.y;


        /////// Args ///////
        // let args:  Vec<String> = env::args().collect();
        //
        // let mut  args: &mut  &'static Vec<String> = &mut  vec!["hi".to_string()];
        //
        // for arg in pargs {
        //     args.push(arg)
        // }

    // let args = getopts
    // unsafe {
        for (i, arg) in args[1..args.len()].iter().enumerate() {
            // if arg.contains("--position") ||  arg.contains("-p=") {
            //     let bind:Vec<&str> = arg.split("=").collect();
            //     dispatcher_var = bind[1];
            //
            // } else
            {
                match arg.as_str() {
                    "--position" | "-p" => dispatcher_var = args[i + 2].as_str(),
                    "--resize" | "-r" => resize = true,
                    // "anti" |
                    // "center" |
                    // "cursor" |
                    // "random"    => dispatcher_var = arg,
                    "--width" | "-w" => width = args[i + 2].parse::<i16>().unwrap(),
                    "--height" | "-h" => height = args[i + 2].parse::<i16>().unwrap(),
                    _ => continue,
                }
            }
        }


        /////// Command ///////
        // add double space handling
        if param == "togglefloating" {
            toggle_float = true;
        } else if param == "open" {
            do_float = true;

            let command: Vec<&str> = args[args.len() - 1].split(" ").collect();
            let mut runit = Command::new(command[0]);

            for arg in &command[1..=command.len() - 1] {
                if arg != &"" {
                    runit.arg(arg);
                }
            }

            let _ = runit.spawn();
        }


        /////// Cycle ///////
        let start_addr = crate::client_data().address;




    for _i in 0 ..= 50 {
        sleep(time::Duration::from_millis(100));
        let mid_addr = crate::client_data().address;

        if (mid_addr != start_addr && do_float == true) ||
            (mid_addr == start_addr && toggle_float == true) {
            let loc_cli = client_data();

            release_vars(resize, do_float, toggle_float, width, height, dispatcher_var);
            break
        }

    }
}



pub fn release_vars(resize:bool, do_float: bool, toggle_float: bool, width: i16, height: i16, dispatcher_var: &str ) {
    let loc_conf = config_data();

    if do_float == true && client_data().floating == false || toggle_float == true  {
        let _ = Dispatch::call(ToggleFloating(None));

    }

    if client_data().floating == false {
        exit(0x0100);
    }

    if resize == true || width != loc_conf.size.x || height != loc_conf.size.y {
        let _ = Dispatch::call(ResizeActive(Exact(width, height)));
    }


    let dispatcher: DispatchType = move_window(dispatcher_var);
    let _ = Dispatch::call(dispatcher);
}


fn move_anti(pos: i16, max_pos: i16, win_size: i16) -> i16{
    if pos >= max_pos / 2
    { 0 }
    else
    { max_pos - win_size }
}


    pub fn detect_padding(window_pos: i16, min_pos: i16, max_pos: i16) -> i16 {
        let mut output = window_pos;

        if crate::config_data().detect_borders == true {
            if window_pos <= min_pos {
                output = min_pos
            } else if window_pos >= max_pos {
                output = max_pos
            }
        }

        return output
    }


    pub fn move_window(param: &str) -> DispatchType<'static> {
        let loc_cli = client_data();
        let loc_count = count_data();


        let mut window_pos_x: i16;
        let mut window_pos_y: i16;

        let mut def_pos_x: i16 = loc_cli.window_pos.x;
        let mut def_pos_y: i16 = loc_cli.window_pos.y;


        // Command::new("dunstify").arg(param).spawn();

        if param == "cursor" {
            def_pos_x = count_data().window_center.x;
            def_pos_y = count_data().window_center.y;

        } else if param == "random" {
            let mut rng = rand::thread_rng();

            def_pos_x = rng.gen_range(1..=(loc_cli.screen_size.x - loc_cli.window_size.x));
            def_pos_y = rng.gen_range(1..=(loc_cli.screen_size.y - loc_cli.window_size.y));

        } else if param == "opposite" {
            def_pos_x = loc_cli.screen_size.x - loc_cli.cursor_pos.x - loc_cli.window_size.x / 2;
            def_pos_y = loc_cli.screen_size.y - loc_cli.cursor_pos.y - loc_cli.window_size.y / 2;

        } else if param == "anti" {
            def_pos_x = move_anti(loc_cli.cursor_pos.x, loc_cli.screen_size.x, loc_cli.window_size.x);
            def_pos_y = move_anti(loc_cli.cursor_pos.y, loc_cli.screen_size.y, loc_cli.window_size.y);

            // let _ = Command::new("dunstify")
            //     .arg("anti").spawn();

        } else if param != "default" {
            exit(0x0100)
        }


        window_pos_x = detect_padding(
            def_pos_x,
            config_data().margins.left,
            count_data().max_pos.x
        );

        window_pos_y = detect_padding(
            def_pos_y,
            config_data().margins.top,
            count_data().max_pos.y,
        );

        return DispatchType::MoveActive(Exact(window_pos_x, window_pos_y))

    }


// Unwrap Struct
// struct Board {
//     tiles: Vec<Tile>,
// }
//
// impl Board {
//     pub fn print(&self) {
//         for tile in self.tiles {
//             println!("{}", match tile {
//                 Tile::Empty => " ",
//                 Tile::Cross => "X"
//                 Tile::Circle => "O",
//             });
//         }
//     }
// }


// Struct to Vec
// fn main(){}
// struct FruitStore{
//     fruit:String,
// }
// impl FruitStore{
//     pub fn calculate(&self){
//         let mut x:Vec<fn(&Self)->()> = vec![];
//         x.push(FruitStore::get_fruit);
//     }
//     pub fn get_fruit(&self){
//         self.fruit.clone();
//     }
// }


// fn main() {}

