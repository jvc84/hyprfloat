use hyprland::data::{Client, WorkspaceBasic};
use hyprland::shared::Address;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use std::env;
use std::collections::HashMap;
use simple_home_dir::*;
use std::path::PathBuf;
use clap::Parser;
use crate::{Config, FromClient, Parameters, empty_parameters, get_tags, MonitorData, CountAxisData, Paddings,};


pub const BUF_SIZE: usize = 8192;



lazy_static! (
    pub static ref SOCKET_FILE: String = String::from(
            "/tmp/hyprfloat.socket"
    );
    pub static ref TAGS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new())); 
    static ref ARGS: Vec<String> = env::args().collect();
    pub static ref PARAMETERS: Arc<RwLock<Parameters>> = {
       Arc::new(RwLock::new(
            empty_parameters()
       ))
    };
    pub static ref EXACT: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
    pub static ref POSITION_VALUES: Vec<&'static str> = vec![
        "cursor",
        "center",
        "random",
        "far",
        "close",
        "opposite",
        "any",
        "l" , "left",
        "r" , "right",
        "t" , "top",
        "b" , "bottom",
        "tl", "top-left",
        "tr", "top-right",
        "bl", "bottom-left",
        "br", "bottom-right",
    ];
    pub static ref CLASS: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("")));
    pub static ref BIN: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("")));

    pub static ref GIVE_ARGS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));

    static ref HOME: PathBuf = home_dir().unwrap();
    pub static ref CACHE_DIR: String = format!("{}{}", HOME.to_str().unwrap(), "/.cache/hyprfloat/");
    pub static ref CLASSES_FILE: String = String::from("classes.json");
    pub static ref SIZES_FLIE: String = String::from("sizes.json");
    pub static ref CLASS_CACHE_FILE: String = format!("{}{}", CACHE_DIR.clone(), CLASSES_FILE.clone());
    pub static ref SIZE_CACHE_FILE: String = format!("{}{}", CACHE_DIR.clone(), SIZES_FLIE.clone());
    pub static ref CONFIG_PATH: String = "/.config/hyprfloat/hf.toml".to_string();
    pub static ref CONFIG_FILE: Arc<RwLock<String>> = Arc::new(RwLock::new(
        format!("{}{}", HOME.to_str().unwrap(), CONFIG_PATH.as_str())
    ));
    pub static ref CONFIG_DATA: Arc<RwLock<Config>> = Arc::new(RwLock::new(
         Config {
            axis_data: HashMap::new(),
            detect_padding: false,
            detect_bars: false,
            standard_resize: false,
            stick_to_borders: false,
            invert_resize_in_stick_mode: false,
            resize_through_borders: false,
            toggle_to_open_size: false,
        }
    ));
    pub static ref WINDOWS_CONFIG_DATA: Arc<RwLock<toml::Table>> = 
        Arc::new(RwLock::new(
            toml::Table::new()
        ));
    pub static ref LAYERS_DATA: Arc<RwLock<HashMap<String, Paddings>>> =
        Arc::new(RwLock::new(
           HashMap::new()
    ));
    pub static ref MONITOR_DATA: Arc<RwLock<MonitorData>> = Arc::new(RwLock::new(
         MonitorData {
            axis_data: HashMap::new(),
            name: "".to_string()
        }
    ));
    pub static ref CLIENT_DATA: Arc<RwLock<FromClient>> = Arc::new(RwLock::new(
        FromClient {
            tags: Vec::new(),
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
        monitor: Some(0),
        initial_class: "Empty".to_string(),
        class: "Empty".to_string(),
        initial_title: "Empty".to_string(),
        title: "Empty".to_string(),
        pid: 28823,
        xwayland: true,
        pinned: false,
        grouped: Vec::new(),
        tags: Vec::new(),
        mapped: true,
        focus_history_id: 1,
        swallowing: Some(
            Box::<Address>::new(Address::new("0x0"))
        )
    }
}
