use std::collections::HashMap;
use hyprland::shared::Address;
use serde::{Deserialize, Serialize};
use crate::WINDOWS_CONFIG_DATA;

#[derive(Deserialize, Clone, Debug, Serialize)]
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


#[derive(Deserialize, Debug, Clone)]
pub struct ClientAxisData {
    pub window_pos:  i16,
    pub window_size: i16,
    pub cursor_pos:  i16,
}


#[derive(Deserialize, Debug, Clone)]
pub struct FromClient {
    pub axis_data: HashMap<String, ClientAxisData>,
    pub tags: Vec<String>,
    pub address: Address,
    pub class: String,
    pub monitor: String,
    pub floating: bool,
    pub fullscreen: bool,
}


#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Config {
    pub axis_data:  HashMap<String, ConfigAxisData>,
    pub detect_padding: bool,
    pub detect_bars: bool,
    pub standard_resize: bool,
    pub stick_to_borders: bool,
    pub invert_resize_in_stick_mode: bool,
    pub resize_through_borders: bool,   
    pub toggle_to_open_size: bool,
}


#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Paddings {
    pub padding_min: i16,
    pub padding_max: i16,
}


#[derive(Deserialize, Clone, Debug,  Serialize)]
pub struct MonitorAxisData {
    pub min_point: i16,
    pub max_point: i16,
    pub resolution: u16,
}


#[derive(Deserialize, Clone, Debug,  Serialize)]
pub struct MonitorData {
    pub axis_data: HashMap<String, MonitorAxisData>,
    pub name: String,
}


#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct DaemonData {
    pub daemon_config: Config,
    pub daemon_windows_config: toml::Table,
    pub daemon_monitor: MonitorData,
    pub daemon_layers: HashMap<String, Paddings>,
}


#[derive(Deserialize, Clone)]
pub struct PreConfig {
    pub padding: Option<(toml::value::Value, toml::value::Value, toml::value::Value, toml::value::Value)>,
    pub default_size: Option<(toml::value::Value, toml::value::Value)>,
    pub margin: Option<(toml::value::Value, toml::value::Value)>,
    pub detect_padding: Option<bool>,
    pub detect_bars: Option<bool>,
    pub standard_resize: Option<bool>,
    pub stick_to_borders: Option<bool>,
    pub invert_resize_in_stick_mode: Option<bool>,
    pub resize_through_borders: Option<bool>,
    pub toggle_to_open_size: Option<bool>,
}


#[derive(Debug, Clone)]
pub struct Parameters {
    pub size_from_args: bool,
    pub default_size: bool,
    pub resize_x: bool,
    pub resize_y: bool,
    pub tile: bool,
    pub origin_size: bool,
    pub binary: String,
    pub count_system: String,
    pub position: String,
    pub size: Option<HashMap<String, i16>>,
    pub at: Option<HashMap<String, i16>>,
}


pub fn empty_parameters() -> Parameters {
    Parameters {
        size_from_args: false,
        default_size: false,
        resize_x: false,
        resize_y: false,
        tile: false,
        origin_size: false,
        binary: "".to_string(),
        count_system: "origin".to_string(),
        position: "any".to_string(),
        size: None,
        at: None
    }
}
