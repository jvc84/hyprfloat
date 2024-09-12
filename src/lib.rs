use std::path::PathBuf;
use std::thread::sleep;
use std::string::ToString;
use std::boxed::Box;
use std::{
    env,
    fs,
    time
};
use std::process::{
    Command,
    exit
};

use hyprland::prelude::*;
use hyprland::dispatch::Position::Exact;
use hyprland::shared::Address;
use hyprland::ctl::notify::Icon;
use hyprland::dispatch::DispatchType::{
    ResizeActive,
    ToggleFloating
};
use hyprland::dispatch::{
    Dispatch,
    DispatchType
};
use hyprland::data::{
    Client,
    Monitor,
    WorkspaceBasic,
    CursorPosition
};

use toml;
use serde::Deserialize;
use rand::Rng;
use lazy_static::lazy_static;
use simple_home_dir::*;



#[derive(Deserialize, Clone)]
pub struct Descartes {
    pub x: i16,
    pub y: i16,
}


#[derive(Deserialize, Clone)]
pub struct FromClient {
    pub window_pos:  Descartes,
    pub window_size: Descartes,
    pub screen_min: Descartes,
    pub screen_max: Descartes,
    pub cursor_pos:  Descartes,
    pub address: Address,
    pub class: String,
    pub monitor: String,
    pub floating: bool,
    pub fullscreen: bool,
}

#[derive(Deserialize, Clone)]
pub struct Margins {
    pub left:   i16,
    pub top:    i16,
    pub bottom: i16,
    pub right:  i16,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub padding: Margins,
    pub size: Descartes,
    pub detect_padding: bool,
    pub classic_resize: bool,
    pub stick_to_borders: bool,
    pub invert_keys_in_stick_mode: bool,
    pub resize_through_borders: bool,
}



#[derive(Deserialize, Clone)]
pub struct Count {
    pub max_pos: Descartes,
    pub window_center: Descartes,
}




lazy_static! {
    static ref HOME: PathBuf = home_dir().unwrap();
    static ref CONFIG_PATH: String = "/.config/hypr/hf.toml".to_string();
    static ref CONFIG_FILE: String = format!("{}{}", HOME.to_str().unwrap(), CONFIG_PATH.as_str());

    static ref CONFIG_RAW_DATA: String = check_config_file(CONFIG_FILE.as_str());
    pub static ref CONFIG_DATA: Config = config_data();
}



fn check_config_file(arg: &str ) -> String {

    match fs::read_to_string(arg).is_ok() {
        true =>  fs::read_to_string(arg).unwrap(),
        false => {
            notify_error(
                format!("No Config in {}", arg).as_str()
            );
            exit(0x0100)
        }
    }
}


fn notify_error(message: &str )  {
    let _ = hyprland::ctl::notify::call(
        Icon::Error,
        time::Duration::from_secs(10),
        hyprland::ctl::Color::new(100, 50, 50 ,50) ,
        format!(" Hyprfloat: {}", message.to_string())

    );
}


fn check_config_content(string: String) -> Result<Config, toml::de::Error> {
    let result: Result<Config, _>  = toml::from_str(&string);

    match result.clone() {
        Ok(x) => result,
        Err(e) => Err(e)
    }
}


fn get_table(section: &str) -> toml::value::Value{

    let mut full_table: toml::Table;
    match  toml::from_str::<toml::Table>(&CONFIG_RAW_DATA).is_ok() {
        true => {
            full_table = toml::from_str(&CONFIG_RAW_DATA).unwrap()
        }
        false => {
            notify_error("Config Error: Wrong parameter value");
            exit(0x0100)
        }
    }

    let table: toml::Value;
    match full_table.get(section){
        Some(x) =>  table = full_table[section].clone(),
        None => {
            notify_error(
                format!("Config Error: No section \"{}\" in {}", section, CONFIG_FILE.as_str() ).as_str()
            );
            exit(0x0100)

        }
    };

    table
}

pub fn config_data() -> Config {

    let table = get_table("monitors");

    let mut string: String = "empty".to_string();
    let section = Monitor::get_active().unwrap().id.to_string();

    match table.get(section.clone()) {
        Some(x) => {
            string =  toml::to_string(&table[&section]).unwrap();
        },
         None => {
             match table.get("any".to_string()){
                 Some(x) => {
                     string = toml::to_string(&table[&"any".to_string()]).unwrap()
                 },

                 None => {
                     notify_error("Config Error: no section \"any\"");
                     exit(0x0100)
                 }
             }
         }
    }


    if let  Err(e) = check_config_content(string.clone()) {
        notify_error(
            format!("Config Error: missing parameter in section: monitors.{}", section).as_str());
        exit(0x0100);

    }

    let config: Config = check_config_content(string.clone()).unwrap();

    config
}


pub fn count_data() -> Count {
    let loc_cli = client_data();
    
    let count = Count {
        max_pos: Descartes {
            x: loc_cli.screen_max.x - loc_cli.window_size.x - CONFIG_DATA.padding.right,
            y: loc_cli.screen_max.y - loc_cli.window_size.y - CONFIG_DATA.padding.bottom,
        },
        window_center: Descartes {
            x: loc_cli.cursor_pos.x - (loc_cli.window_size.x / 2),
            y: loc_cli.cursor_pos.y - (loc_cli.window_size.y / 2),
        }
    };

    count
}


pub fn parse_fullscreen(arg:hyprland::data::FullscreenMode) -> bool {
    if arg == hyprland::data::FullscreenMode::None {
        false
    } else {
        true
    }
}


pub fn empty_client() -> Client {
    Client {
        address: Address::new(
            "0x1a1a1a1a1a1a".to_string(),
        ),
        at: (500, 500),
        size: (1920,1080),
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
}


pub fn client_data() -> FromClient {
    let active_window = Client::get_active()
        .unwrap()
        .unwrap_or(
            empty_client()
        );
    let active_monitor = Monitor::get_active().unwrap();
    let cursor_position = CursorPosition::get().unwrap();

    let client = FromClient {
        window_pos: Descartes {
            x: active_window.at.0,
            y: active_window.at.1,
        },
        window_size: Descartes {
            x: active_window.size.0,
            y: active_window.size.1,
        },
        screen_min: Descartes {
            x: active_monitor.x as i16,
            y: active_monitor.y as i16,
        },
        screen_max: Descartes {
            x: active_monitor.x as i16 + active_monitor.width as i16,
            y: active_monitor.y as i16 + active_monitor.height as i16,
        },
        cursor_pos: Descartes{
            x: cursor_position.x as i16,
            y: cursor_position.y as i16,
        },

        class: active_window.class,
        monitor: active_window.monitor.to_string(),
        address: active_window.address,
        floating: active_window.floating,
        fullscreen: parse_fullscreen(active_window.fullscreen),
    };


    client
}


lazy_static! {
    static ref args:  Vec<String> = env::args().collect();
}

pub fn define_args(param: &str) {

    /////// Arguments ///////
    let mut dispatcher_var: &str = &"any";
    let mut  do_float: bool = false;
    let mut toggle_float: bool = false;
    let mut resize: bool = false;
    let mut width: i16  = crate::CONFIG_DATA.size.x;
    let mut height: i16 = crate::CONFIG_DATA.size.y;
    let mut tiled: bool = false;


    for (i, arg) in args[1..args.len()].iter().enumerate() {
        match arg.as_str() {
            "--position" | "-p" => dispatcher_var = args[i + 2].as_str(),
            "--resize"   | "-r" => resize = true,
            "--tiled"    | "-t" => tiled = true,
            "--width"    | "-w" => width =  args[i + 2].parse::<i16>().unwrap(),
            "--height"   | "-h" => height = args[i + 2].parse::<i16>().unwrap(),
            _ => continue,
        }
    }


    /////// Command ///////
    let start_addr = Client::get_active()
        .unwrap()
        .unwrap_or(
            empty_client()
        ).address;

    if param == "togglefloating" {
        toggle_float = true;
    } else if param == "open" {
        do_float = true;

        let _  = Command::new("hyprctl")
            .arg("dispatch")
            .arg("exec")
            .arg("[float] ")
            .arg(args[args.len() - 1].as_str())
            .spawn();
    }


    /////// Cycle ///////
    for _i in 0 ..= 200 {

        let mid_addr = Client::get_active()
            .unwrap()
            .unwrap_or(
                empty_client()
            ).address;

        if (mid_addr != start_addr && do_float == true)
             || (mid_addr == start_addr && toggle_float == true) {

            dispatch_client(resize, do_float, toggle_float, width, height, dispatcher_var, tiled);
            break
        }

        sleep(time::Duration::from_millis(50));
    }
}


pub fn dispatch_client(resize: bool, do_float: bool, toggle_float: bool, width: i16, height: i16, dispatcher_var: &str, tiled: bool) {

    let loc_cli = client_data();

    if do_float == true && loc_cli.floating == false ||
        toggle_float == true ||
        tiled == true && loc_cli.floating == true {

        let _ = Dispatch::call(ToggleFloating(None));
    }

    if client_data().floating == false {
        exit(0x0100)
    }


    if resize == true || width != CONFIG_DATA.size.x || height != CONFIG_DATA.size.y {
        let _ = Dispatch::call(ResizeActive(Exact(width, height)));

    } else {

        let monitor_id = loc_cli.monitor;
        let table = get_table("windows");
        let class = loc_cli.class;
        let list: toml::Table = table.as_table().unwrap().clone();


        if list.keys().collect::<Vec<_>>().contains(&&class.clone()) {
            let class_section = list[&class].clone();

            let value = format!("{}{}", "monitor_", monitor_id);



            if class_section
                .as_table()
                .unwrap()
                .keys()
                .collect::<Vec<_>>()
                .contains(&&value) {
                let param_vec = &class_section.as_table().unwrap()[&value];

                let new_width = param_vec[0].as_integer().unwrap() as i16;
                let new_height = param_vec[1].as_integer().unwrap() as i16;

                let _ = Dispatch::call(ResizeActive(Exact(new_width, new_height)));

            } else if  class_section
                .as_table()
                .unwrap()
                .keys()
                .collect::<Vec<_>>()
                .contains(&&"monitor_any".to_string()) {
                let param_vec = &class_section.as_table().unwrap()[&"monitor_any".to_string()];

                let new_width = param_vec[0].as_integer().unwrap() as i16;
                let new_height = param_vec[1].as_integer().unwrap() as i16;

                let _ = Dispatch::call(ResizeActive(Exact(new_width, new_height)));
            }

        }
    }

    let loc_cli = client_data();
    let loc_count = count_data();

    let cord = move_window(dispatcher_var,  loc_cli.clone(), loc_count);
    let _ = Dispatch::call(cord);
}


fn move_corner(pos: i16, max_pos: i16, win_size: i16) -> i16{
    if pos >= max_pos / 2
    { 0 }
    else
    { max_pos - win_size }
}


pub fn detect_padding(window_pos: i16, min_pos: i16, max_pos: i16) -> i16 {
    let mut output = window_pos;

    if CONFIG_DATA.detect_padding == true {
        if window_pos <= min_pos {
            output = min_pos
        } else if window_pos >= max_pos {
            output = max_pos
        }
    }

    output
}


pub fn move_window(param: &str, loc_cli: FromClient, loc_count: Count) ->  DispatchType<'static> {

    let window_pos_x: i16;
    let window_pos_y: i16;

    let mut def_pos_x: i16 = loc_cli.window_pos.x;
    let mut def_pos_y: i16 = loc_cli.window_pos.y;



    if param == "cursor" {
        def_pos_x = loc_count.window_center.x;
        def_pos_y = loc_count.window_center.y;

    } else if param == "random" {
        let mut rng = rand::thread_rng();

        def_pos_x = rng.gen_range(1..=(loc_cli.screen_max.x - loc_cli.window_size.x));
        def_pos_y = rng.gen_range(1..=(loc_cli.screen_max.y - loc_cli.window_size.y));

    } else if param == "opposite" {
        def_pos_x = loc_cli.screen_max.x - loc_cli.cursor_pos.x - loc_cli.window_size.x / 2;
        def_pos_y = loc_cli.screen_max.y - loc_cli.cursor_pos.y - loc_cli.window_size.y / 2;

    } else if param == "corner" {
        def_pos_x = move_corner(loc_cli.cursor_pos.x, loc_cli.screen_max.x, loc_cli.window_size.x);
        def_pos_y = move_corner(loc_cli.cursor_pos.y, loc_cli.screen_max.y, loc_cli.window_size.y);


    } else if param != "default" {
        exit(0x0100)
    }


    window_pos_x = detect_padding(
        def_pos_x,
        loc_cli.screen_min.x + CONFIG_DATA.padding.left,
        loc_count.max_pos.x
    );

    window_pos_y = detect_padding(
        def_pos_y,
        loc_cli.screen_min.y + CONFIG_DATA.padding.top,
        loc_count.max_pos.y,
    );

    DispatchType::MoveActive(Exact(window_pos_x, window_pos_y))
}
