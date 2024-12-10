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
    AT_PARAMETERS,
    SIZE_PARAMETERS,
};


#[derive(Deserialize, Clone, Debug)]
pub struct ConfigAxisData {
    pub padding_min: i16,
    pub padding_max: i16,
    pub default_size: i16,
    pub margin: i16,
}

#[derive(Deserialize, Clone)]
pub struct CountAxisData {
    pub max_position: i16,
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
pub struct FromClient {
    pub axis_data: HashMap<String, ClientAxisData>,
    pub address: Address,
    pub class: String,
    pub monitor: String,
    pub floating: bool,
    pub fullscreen: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub axis_data:  HashMap<String, ConfigAxisData>,
    pub detect_padding: bool,
    pub standard_resize: bool,
    pub stick_to_borders: bool,
    pub invert_resize_in_stick_mode: bool,
    pub resize_through_borders: bool,
}


#[derive(Deserialize, Clone)]
pub struct PreConfig {
    pub padding: Option<(i16, i16, i16, i16)>,
    pub default_size: Option<(u32, u32)>,
    pub margin: Option<(u32, u32)>,
    pub detect_padding: Option<bool>,
    pub standard_resize: Option<bool>,
    pub stick_to_borders: Option<bool>,
    pub invert_resize_in_stick_mode: Option<bool>,
    pub resize_through_borders: Option<bool>,
}

impl PreConfig {
    fn replace_none_values(&mut self, other: &PreConfig) {
        if self.padding.is_none() {
            self.padding = other.padding;
        }
        if self.default_size.is_none() {
            self.default_size = other.default_size;
        }
        if self.margin.is_none() {
            self.margin = other.margin;
        }
        if self.detect_padding.is_none() {
            self.detect_padding = other.detect_padding;
        }
        if self.standard_resize.is_none() {
            self.standard_resize = other.standard_resize;
        }
        if self.stick_to_borders.is_none() {
            self.stick_to_borders = other.stick_to_borders;
        }
        if self.invert_resize_in_stick_mode.is_none() {
            self.invert_resize_in_stick_mode = other.invert_resize_in_stick_mode;
        }
        if self.resize_through_borders.is_none() {
            self.resize_through_borders = other.resize_through_borders;
        }
    }
}

trait MyTrait {
    fn check_parameter(self, parameter: &str) -> Self;
}

impl<T> MyTrait for Option<T> {
    fn check_parameter(self, parameter: &str) -> Self {
        match self {
            Some(x) => Some(x),
            None => {
                notify_error(format!(
                    "Config Error: missing parameter '{}'. Subsection{}",
                    parameter,
                    SUBSECTION_INFO.read().unwrap()
                ));
                
                exit(0x0100)
            }
        }
    }
}

lazy_static!(
    static ref HOME: PathBuf = home_dir().unwrap();
    pub static ref CACHE_DIR: String = format!("{}{}", HOME.to_str().unwrap(), "/.cache/hyprfloat/");
    pub static ref CLASSES_CACHE_FLIE: String = "classes.toml".to_string();
    pub static ref CACHE_FILE: String =  format!("{}{}", CACHE_DIR.clone(), CLASSES_CACHE_FLIE.clone()); 
    pub static ref CONFIG_PATH: String = "/.config/hyprfloat/hf.toml".to_string();
    pub static ref CONFIG_FILE: String = format!("{}{}", HOME.to_str().unwrap(), CONFIG_PATH.as_str());
    
    pub static ref CONFIG_DATA: Arc<RwLock<Config>> = Arc::new(RwLock::new(
         Config {
            axis_data: HashMap::new(),
            detect_padding: false,
            standard_resize: false,
            stick_to_borders: false,
            invert_resize_in_stick_mode: false,
            resize_through_borders: false,
        }
    ));
    pub static ref CLIENT_DATA: Arc<RwLock<FromClient>> = Arc::new(RwLock::new(
        FromClient {
            axis_data: HashMap::new(),
            address: Address::new(""),
            class: String::from(""),
            monitor: String::from(""),
            floating: false,
            fullscreen: false,
        }
    ));
    pub static ref COUNT_DATA: Arc<RwLock<HashMap<String, CountAxisData>>> = {
       Arc::new(RwLock::new(
           HashMap::new()
       ))
    };

    pub static ref SUBSECTION_INFO: Arc<RwLock<String>> = Arc::new(RwLock::new(
        String::new()
    ));
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
        CLIENT_DATA.read().unwrap().clone()
    );
}


pub fn check_config_file(file: &str) -> String {
    match fs::read_to_string(file).is_ok() {
        true  => fs::read_to_string(file).unwrap(),
        false => {
            notify_error(format!(
                "No Config in {}", file
            ));
            exit(0x0100)
        }
    }
}


pub fn get_table(section: &str, config_path: &str) -> toml::value::Value {
    let config_data_as_string: String = check_config_file(config_path);
    let full_table: toml::Table;
    match toml::from_str::<toml::Table>(&config_data_as_string).is_ok() {
        true => {
            full_table = toml::from_str(&config_data_as_string).unwrap()
        }
        false => {
            notify_error("Config Error: Fatal config error".to_string());
            exit(0x0100)
        }
    }

    let table: toml::Value;
    match full_table.get(section) {
        Some(_) => table = full_table[section].clone(),
        None => {
            notify_error(format!(
                "Config Error: No section \"{}\" in {}",
                section,
                CONFIG_FILE.as_str()
                ));
            exit(0x0100)
        }
    };

    table
}


fn check_any(table: toml::Value) -> bool {
    match table.get("any".to_string()) {
        Some(_) => true,
        None => false
    }
}


pub fn check_config_content(config_data_string: String, subsection: String) -> PreConfig {
    let result: Result<PreConfig, _>  = toml::from_str(&config_data_string);

    match result.clone() {
        Ok(_) => result.unwrap(),
        Err(_) => {
            notify_error(format!(
                "Config Error: Wrong parameter value in subsection 'monitors.{}'", subsection
            ));
            exit(0x0100);
        }
    }
}


pub fn config_data(config_path: String) -> Config {
    let table = get_table("monitors", config_path.as_str());
    let mut subsection = Monitor::get_active().unwrap().id.to_string();


    if table.get(subsection.clone()).is_none() {
        if check_any(table.clone()) == true {
            subsection = String::from("any");
        } else {
            notify_error(format!(
                "Config Error: no subsection '[monitors.any]' or '[monitors.{}]''",
                subsection
            ));
            exit(0x0100)
        }
    }

    let mut pre_config: PreConfig = check_config_content(
        toml::to_string(&table[&subsection]).unwrap(),
        subsection.clone()
    );

    let mut subsection_info = format!(
        " '[monitors.{}]'",
        subsection.clone()
    );

    if subsection.clone() != "any".to_string() && check_any(table.clone()) == true {
        let section_any_pre_config = check_config_content(
            toml::to_string(&table[&"any"]).unwrap(),
            "any".to_string()
        );
        pre_config.replace_none_values(&section_any_pre_config);
        subsection_info = String::from(
            format!(
                "s{} and '[monitors.any]'",
                subsection_info.clone()
            )
        );
    }

    *SUBSECTION_INFO.write().unwrap() = subsection_info.clone();


    let mut axis_map: HashMap<String, ConfigAxisData> = HashMap::new();
    let padding = pre_config.padding.check_parameter("padding").unwrap();
    let default_size = pre_config.default_size.check_parameter("default_size").unwrap();
    let margin = pre_config.margin.check_parameter("default_size").unwrap();
    
    
    axis_map.insert(
        "x".to_string(),
        ConfigAxisData {
            padding_min: padding.3,
            padding_max: padding.1,
            default_size: default_size.0 as i16,
            margin: margin.0 as i16
        }
    );

    axis_map.insert(
        "y".to_string(),
        ConfigAxisData {
            padding_min: padding.0,
            padding_max: padding.2,
            default_size: default_size.1 as i16,
            margin: margin.1 as i16
        }
    );

    let config = Config {
        axis_data: axis_map,
        detect_padding: pre_config.detect_padding.check_parameter("detect_padding").unwrap(),
        standard_resize: pre_config.standard_resize.check_parameter("standard_resize").unwrap(),
        stick_to_borders: pre_config.stick_to_borders.check_parameter("stick_to_borders").unwrap(),
        invert_resize_in_stick_mode: pre_config.invert_resize_in_stick_mode.check_parameter("invert_resize_in_stick_mode").unwrap(),
        resize_through_borders: pre_config.resize_through_borders.check_parameter("resize_through_borders").unwrap(),
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
            window_pos: get_parameter("x", AT_PARAMETERS.clone(), active_window.at.0),
            window_size:  get_parameter("x", SIZE_PARAMETERS.clone(), active_window.size.0),
            monitor_min_point: active_monitor.x as i16,
            monitor_max_point: active_monitor.x as i16 + active_monitor.width as i16,
            cursor_pos: cursor_position.x as i16
        }
    );
    axis_map.insert(
        "y".to_string(),
        ClientAxisData {
            window_pos: get_parameter("y", AT_PARAMETERS.clone(), active_window.at.1),
            window_size: get_parameter("y", SIZE_PARAMETERS.clone(), active_window.size.1),
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
            max_position: cli_axis_data.monitor_max_point - loc_conf.axis_data.get(item).unwrap().padding_max - cli_axis_data.window_size,
            window_center: cli_axis_data.cursor_pos - (cli_axis_data.window_size / 2),
            monitor_resolution: cli_axis_data.monitor_max_point - cli_axis_data.monitor_min_point,
        };

        axis_map.insert(item.to_string(),data);
    }
    axis_map
}
