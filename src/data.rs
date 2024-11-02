use simple_home_dir::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use std::{
    fs,    
    collections::HashMap,
    process::exit
};
use hyprland::{
    prelude::*,
    data::{
        Client, 
        CursorPosition,
        Monitor,
        WorkspaceBasic
    },
    shared::Address,
    
};
use crate::{
    get_parameter,
    notify_error,
    POSITION_PARAMETERS,
    SIZE_PARAMETERS,
};


#[derive(Deserialize, Clone)]
pub struct ConfigAxisData {
    pub padding_min: i16,
    pub padding_max: i16,
    pub default_size: i16,
    pub margin: i16,
}

#[derive(Deserialize, Clone)]
pub struct CountAxisData {
    pub max_pos: i16,
    pub window_center: i16,
    pub monitor_resolution: i16,
}

#[derive(Deserialize, Clone)]
pub struct ClientAxisData {
    pub window_pos:  i16,
    pub window_size: i16,
    pub monitor_min_point: i16,
    pub monitor_max_point: i16,
    pub cursor_pos:  i16,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub axis_data:  HashMap<String, ConfigAxisData>,
    pub detect_padding: bool,
    pub standard_resize: bool,
    pub stick_to_borders: bool,
    pub invert_keys_in_stick_mode: bool,
    pub resize_through_borders: bool,
}

#[derive(Deserialize, Clone)]
pub struct FromClient {
    pub axis_data: HashMap<String, ClientAxisData>,
    pub address: Address,
    pub class: String,
    pub monitor: String,
    pub floating: bool,
    pub fullscreen: bool,
}

#[derive(Deserialize, Clone)]
pub struct PreConfig {
    pub padding: (i16, i16, i16, i16),
    pub default_size: (u32, u32),
    pub margin: (u32, u32),
    pub detect_padding: bool,
    pub standard_resize: bool,
    pub stick_to_borders: bool,
    pub invert_keys_in_stick_mode: bool,
    pub resize_through_borders: bool,
}


lazy_static!(
    static ref HOME: PathBuf = home_dir().unwrap();
    pub static ref XDG_PATH: String = "/.config/hyprfloat/hf.toml".to_string();
    pub static ref CONFIG_FILE: String = format!("{}{}", HOME.to_str().unwrap(), XDG_PATH.as_str());

    pub static ref CONFIG_DATA: Arc<RwLock<Config>> = Arc::new(RwLock::new(config_data(CONFIG_FILE.clone())));
    pub static ref CLIENT_DATA:  Arc<RwLock<FromClient>> = Arc::new(RwLock::new(client_data()));
    pub static ref COUNT_DATA: Arc<RwLock<HashMap<String, CountAxisData>>> = {
       Arc::new(RwLock::new(
            count_data(
                CLIENT_DATA.read().unwrap().clone()
            )
       ))
    };
);


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
        size: (777,666),
        workspace: WorkspaceBasic {
            id: 4,
            name: "Empty".to_string(),
        },
        floating: false,
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


pub fn update_data() {
    *CLIENT_DATA.write().unwrap() = client_data();
    *COUNT_DATA.write().unwrap() = count_data(
        client_data()
    );
}


pub fn check_config_file(file: &str) -> String {
    match fs::read_to_string(file).is_ok() {
        true =>  fs::read_to_string(file).unwrap(),
        false => {
            notify_error(
                format!("No Config in {}", file).as_str()
            );
            exit(0x0100)
        }
    }
}


pub fn check_config_content(string: String) -> Result<PreConfig, toml::de::Error> {
    let result: Result<PreConfig, _>  = toml::from_str(&string);

    match result.clone() {
        Ok(_) => result,
        Err(e) => Err(e)
    }
}


pub fn get_table(section: &str, config_path: &str) -> toml::value::Value{
    let config_raw_data: String = check_config_file(config_path);
    let full_table: toml::Table;
    match  toml::from_str::<toml::Table>(&config_raw_data).is_ok() {
        true => {
            full_table = toml::from_str(&config_raw_data).unwrap()
        }
        false => {
            notify_error("Config Error: Wrong parameter value");
            exit(0x0100)
        }
    }

    let table: toml::Value;
    match full_table.get(section){
        Some(_) =>  table = full_table[section].clone(),
        None => {
            notify_error(
                format!("Config Error: No section \"{}\" in {}", section, CONFIG_FILE.as_str()).as_str()
            );
            exit(0x0100)

        }
    };

    table
}


pub fn config_data(config_path: String) -> Config {
    let table = get_table("monitors", config_path.as_str());
    let mut section_data_string: String = "empty".to_string();
    let mut section = Monitor::get_active().unwrap().id.to_string();

    match table.get(section.clone()) {
        Some(_) => {
            section_data_string = toml::to_string(&table[&section]).unwrap();
        },
        None => {
            match table.get("any".to_string()){
                Some(_) => {
                    section = "any".to_string();
                    section_data_string = toml::to_string(&table[&"any".to_string()]).unwrap()
                },
                None => {
                    notify_error("Config Error: no section \"[monitors.any]\"");
                    exit(0x0100)
                }
            }
        }
    }

    if let  Err(_) = check_config_content(section_data_string.clone()) {
        notify_error(
            format!("Config Error: missing parameter in section: monitors.{}", section).as_str()
        );
        exit(0x0100);
    }

    let pre_config: PreConfig = check_config_content(section_data_string).unwrap();
    let mut axis_map: HashMap<String, ConfigAxisData> = HashMap::new();

    axis_map.insert(
        "x".to_string(),
        ConfigAxisData {
            padding_min: pre_config.padding.0,
            padding_max: pre_config.padding.3,
            default_size: pre_config.default_size.0 as i16,
            margin: pre_config.margin.0 as i16
        }
    );

    axis_map.insert(
        "y".to_string(),
        ConfigAxisData {
            padding_min: pre_config.padding.1,
            padding_max: pre_config.padding.2,
            default_size: pre_config.default_size.1 as i16,
            margin: pre_config.margin.1 as i16
        }
    );

    let config = Config {
        axis_data: axis_map,
        detect_padding: pre_config.detect_padding,
        standard_resize: pre_config.standard_resize,
        stick_to_borders: pre_config.stick_to_borders,
        invert_keys_in_stick_mode: pre_config.invert_keys_in_stick_mode,
        resize_through_borders: pre_config.resize_through_borders,
    };

    config
}


pub fn client_data() -> FromClient {
    let active_window = Client::get_active()
        .unwrap()
        .unwrap_or(empty_client());
    let active_monitor = Monitor::get_active().unwrap();
    let cursor_position = CursorPosition::get().unwrap();

    let mut axis_map: HashMap<String, ClientAxisData> = HashMap::new();

    axis_map.insert(
        "x".to_string(),
        ClientAxisData {
            window_pos: get_parameter("x", POSITION_PARAMETERS.clone(), active_window.at.0),
            window_size:  get_parameter("x", SIZE_PARAMETERS.clone(), active_window.size.0),
            monitor_min_point: active_monitor.x as i16,
            monitor_max_point: active_monitor.x as i16 + active_monitor.width as i16,
            cursor_pos: cursor_position.x as i16
        }
    );
    axis_map.insert(
        "y".to_string(),
        ClientAxisData {
            window_pos: get_parameter("y", POSITION_PARAMETERS.clone(), active_window.at.1),
            window_size:  get_parameter("y", SIZE_PARAMETERS.clone(), active_window.size.1),
            monitor_min_point: active_monitor.y as i16,
            monitor_max_point: active_monitor.y as i16 + active_monitor.height as i16,
            cursor_pos: cursor_position.y as i16
        }
    );

    let client = FromClient {
        axis_data: axis_map,
        class: active_window.class,
        monitor: active_window.monitor.to_string(),
        address: active_window.address,
        floating: active_window.floating,
        fullscreen: parse_fullscreen(active_window.fullscreen),
    };
    client
}


pub fn count_data(cli_data: FromClient) -> HashMap<String, CountAxisData> {
    let list = ["x", "y"];

    let loc_conf = CONFIG_DATA.read().unwrap().clone();
    let mut axis_map : HashMap<String, CountAxisData> = HashMap::new();

    for item in list {
        let cli_axis_data = cli_data.axis_data.get(item).unwrap();
        let data = CountAxisData {
            max_pos: cli_axis_data.monitor_max_point - loc_conf.axis_data.get(item).unwrap().padding_max - cli_axis_data.window_size,
            window_center: cli_axis_data.cursor_pos - (cli_axis_data.window_size / 2),
            monitor_resolution: cli_axis_data.monitor_max_point - cli_axis_data.monitor_min_point,
        };

        axis_map.insert(item.to_string(),data);
    }
    axis_map
}
