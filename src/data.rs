use std::{
    fs,    
    collections::HashMap,
    process::exit
};
use std::io::Read;
use std::os::unix::net::UnixStream;
use hyprland::{
    prelude::*,
    data::{
        Layers,
        LayerDisplay,
        LayerClient,
        Client, 
        CursorPosition,
        Monitor,
    },
    
};

use crate::{ notify_error, IfPercent};

use crate::globals::*;
use crate::structs::*;


impl PreConfig {
    fn replace_none_values(&mut self, other: &PreConfig) {
        if self.padding.is_none() {
            self.padding = other.clone().padding;
        }
        if self.default_size.is_none() {
            self.default_size = other.clone().default_size;
        }
        if self.margin.is_none() {
            self.margin = other.clone().margin;
        }
        if self.detect_padding.is_none() {
            self.detect_padding = other.detect_padding;
        }
        if self.detect_bars.is_none() {
            self.detect_bars = other.detect_bars;
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
        if self.toggle_to_open_size.is_none() {
            self.toggle_to_open_size = other.toggle_to_open_size;
        }
    }
}

trait ConfParametersTrait {
    fn check_parameter(self, parameter: &str) -> Self;
}


impl<T> ConfParametersTrait for Option<T> {
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


// pub fn get_session_hash() -> String {
//     let command = std::process::Command::new("hyprctl")
//         .arg("version")
//         .output()
//         .expect("failed to execute process");
// 
//     let output = String::from_utf8(command.stdout).unwrap();
//     let date = output.split("\n").collect::<Vec<&str>>()[1];
//     let hash  =  format!("{:x}", md5::compute(date.as_bytes()));
// 
//     hash
// }


pub fn update_data() {
    *CLIENT_DATA.write().unwrap() = client_data();
}


fn remove_zeros(vec: [u8; BUF_SIZE]) -> Vec<u8> {
    let new_vec = vec.clone().iter().filter(|&&x| x != 0).cloned().collect::<Vec<u8>>();
    new_vec
}


pub fn get_daemon_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect(SOCKET_FILE.as_str())?;
    
    let mut buf = [0u8; BUF_SIZE];
    let _ = stream.read(&mut buf)?; //.unwrap() stream.read(&mut buf)?; //.read_exact(&mut buf)?;

    let new_buf = remove_zeros(buf);
    
    // println!("New buf: {:?}", new_buf.len().clone());

    let work_buf: DaemonData = serde_json::from_slice(new_buf.as_slice())?; //from_str(&buf)?;
    *MONITOR_DATA.write().unwrap() = work_buf.daemon_monitor.clone();
    *WINDOWS_CONFIG_DATA.write().unwrap()  = work_buf.daemon_windows_config.clone();
    *CONFIG_DATA.write().unwrap() = work_buf.daemon_config.clone();
    *LAYERS_DATA.write().expect("Failed to update layers data") = work_buf.daemon_layers.clone();

    Ok(())
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
                CONFIG_FILE.read().unwrap().clone().as_str()
                ));
            exit(0x0100)
        }
    };

    table
}


fn check_any(table: toml::Value) -> bool {
    table.get("any".to_string()).is_some()
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


pub fn windows_config_data() -> Result<(), Box<dyn std::error::Error>> {
    let value = get_table("windows", CONFIG_FILE.read().unwrap().clone().as_str());
    *WINDOWS_CONFIG_DATA.write().unwrap() =  value.as_table().unwrap().clone();
    
    Ok(())
    
}


pub fn layers_data() -> Result<(), Box<dyn std::error::Error>> {
    let conf = CONFIG_DATA.read()?.clone();
    let monitor = MONITOR_DATA.read()?.clone();
    
    let mut x_edges = (0, 0);
    let mut y_edges = (0, 0);

    let conf_x = conf.axis_data.get("x").unwrap();
    let conf_y = conf.axis_data.get("y").unwrap();

    if conf.detect_bars && conf.detect_padding {
        let mut bars_vec: Vec<LayerClient> = Vec::new();

        let layers = Layers::get().unwrap();
        let layers_displays = layers
            .into_iter()
            .collect::<HashMap<String, LayerDisplay>>();

        let layers_monitor = layers_displays
            .get(monitor.name.as_str())
            .expect("Cannot get monitor name")
            .levels
            .clone();

        let screen_w = monitor.axis_data.get("x").unwrap().resolution as i32;
        let screen_h = monitor.axis_data.get("y").unwrap().resolution as i32;


        for (_key, value) in layers_monitor.clone().into_iter() {
            for client in value {
                if client.clone().namespace.to_lowercase().contains("bar") &&
                    (client.w > (screen_w as f32 * 0.1) as i16 ||
                        client.h > (screen_h as f32 * 0.1) as i16) {
                    bars_vec.push(client.clone())
                }
            }
        }
        
        for client in bars_vec {
            match (client.w < client.h, client.w > client.h) {
                (true, false) => x_edges = compare_layers(screen_w as i16, x_edges, client.w, client.x as i16),
                (false, true) => y_edges = compare_layers(screen_h as i16, y_edges, client.h, client.y as i16), 
                _ => {} 
            }
        }
    }
    
    LAYERS_DATA.write()?.insert(
        "x".to_string(),
        Paddings {
            padding_min: x_edges.0 + conf_x.padding_min,
            padding_max: x_edges.1 + conf_x.padding_max,
        }
    );
    LAYERS_DATA.write()?.insert(
        "y".to_string(),
        Paddings {
            padding_min: y_edges.0 + conf_y.padding_min,
            padding_max: y_edges.1 + conf_y.padding_max,
        }
    );

    Ok(())
}


fn compare_layers(
    screen_param: i16,
    mut edges: (i16, i16),
    param: i16,
    pos: i16,
    ) -> (i16, i16) {
    if screen_param - pos > screen_param / 2 && (pos + param) > edges.0 {
        edges.0 = param + pos
    } else if screen_param - pos > edges.1 {
        edges.1 = screen_param - pos
    }

    edges
}


pub fn config_data() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = CONFIG_FILE.read().unwrap().clone();
    let table = get_table("monitors", config_path.as_str());
    let mut subsection = Monitor::get_active()?.id.to_string();


    if table.get(subsection.clone()).is_none() {
        match check_any(table.clone()) {
            true => subsection = String::from("any"),
            false => {
                notify_error(format!(
                    "Config Error: no subsection '[monitors.any]' or '[monitors.{}]''",
                    subsection
                ));
                exit(0x0100)
            }
        }
    }

    let mut pre_config: PreConfig = check_config_content(
        toml::to_string(&table[&subsection])?,
        subsection.clone()
    );

    let mut subsection_info = format!(
        " '[monitors.{}]'",
        subsection.clone()
    );

    if subsection.clone() != *"any".to_string() && check_any(table.clone()) {
        let section_any_pre_config = check_config_content(
            toml::to_string(&table[&"any"])?,
            "any".to_string()
        );
        pre_config.replace_none_values(&section_any_pre_config);
        subsection_info = format!(
                "s{} and '[monitors.any]'",
                subsection_info.clone()
            );
    }

    *SUBSECTION_INFO.write()? = subsection_info.clone();
    

    let mut axis_map: HashMap<String, ConfigAxisData> = HashMap::new();
    let padding = pre_config.padding.check_parameter("padding").unwrap();
    let default_size = pre_config.default_size.check_parameter("default_size").unwrap();
    let margin = pre_config.margin.check_parameter("default_size").unwrap();
    
    
    axis_map.insert(
        "x".to_string(),
        ConfigAxisData {
            padding_min: padding.3.check_percent("x"),
            padding_max: padding.1.check_percent("x"),
            default_size: default_size.0.check_percent("x") as u16 as i16,
            margin: margin.0.check_percent("x") as u16 as i16
        }
    );

    axis_map.insert(
        "y".to_string(),
        ConfigAxisData {
            padding_min: padding.0.check_percent("y"),
            padding_max: padding.2.check_percent("y"),
            default_size: default_size.1.check_percent("y") as u32 as i16,
            margin: margin.1.check_percent("y") as u16 as i16
        }
    );


    let config = Config {
        axis_data: axis_map,
        detect_padding: pre_config.detect_padding.check_parameter("detect_padding").expect("Cannot get detect padding"),
        detect_bars: pre_config.detect_bars.check_parameter("detect_bars").expect("Cannot detect bars"),
        standard_resize: pre_config.standard_resize.check_parameter("standard_resize").unwrap(),
        stick_to_borders: pre_config.stick_to_borders.check_parameter("stick_to_borders").unwrap(),
        invert_resize_in_stick_mode: pre_config.invert_resize_in_stick_mode.check_parameter("invert_resize_in_stick_mode").unwrap(),
        resize_through_borders: pre_config.resize_through_borders.check_parameter("resize_through_borders").unwrap(),
        toggle_to_open_size: pre_config.toggle_to_open_size.check_parameter("toggle_to_open_size").unwrap(),
    };

    Ok(config)
}


pub fn monitor_data() -> MonitorData {
    let active_monitor = Monitor::get_active().unwrap();
    
    println!("Monitor name: {}", active_monitor.name);
    let mut axis_map: HashMap<String, MonitorAxisData> = HashMap::new();
        
    axis_map.insert(
        "x".to_string(),
        MonitorAxisData {
            min_point: active_monitor.x as i16,
            max_point: active_monitor.x as i16 + active_monitor.width as i16,
            resolution: active_monitor.width
        }
    );
    
    axis_map.insert(
        "y".to_string(),
        MonitorAxisData {
            min_point: active_monitor.y as i16,
            max_point: active_monitor.y as i16 + active_monitor.height as i16,
            resolution: active_monitor.height
        }
    );
    
    MonitorData {
        axis_data: axis_map,
        name: active_monitor.name
    }
}


pub fn client_data() -> FromClient {
    let active_window = Client::get_active()
        .unwrap()
        .unwrap_or(empty_client());
    let cursor_position = CursorPosition::get().unwrap();

    let mut axis_map: HashMap<String, ClientAxisData> = HashMap::new();
    
    axis_map.insert(
        "x".to_string(),
        ClientAxisData {
            window_pos: active_window.at.0,
            window_size: active_window.size.0,
            cursor_pos: cursor_position.x as i16
        }
    );
    axis_map.insert(
        "y".to_string(),
        ClientAxisData {
            window_pos: active_window.at.1,
            window_size:active_window.size.1,
            cursor_pos: cursor_position.y as i16
        }
    );

    FromClient {
        axis_data: axis_map,
        tags: active_window.tags,
        class: active_window.class,
        monitor: active_window.monitor.unwrap().to_string(),
        address: active_window.address,
        floating: active_window.floating,
        fullscreen: !matches!( active_window.fullscreen, hyprland::data::FullscreenMode::None),
    }
}


// pub fn count_data(cli_data: FromClient) -> Result<HashMap<String, CountAxisData>, Box<dyn std::error::Error>>  {
//     let list = ["x", "y"];
//     
//     // let conf = CONFIG_DATA.read()?.clone();
//     let monitor = MONITOR_DATA.read()?.clone();
//     let mut axis_map : HashMap<String, CountAxisData> = HashMap::new();
// 
//     for axis in list {
//         let cli_axis = cli_data.axis_data.get(axis).expect(format!("Cannot get axis: {}", axis).as_str());
//         let monitor_axis = monitor.axis_data.get(axis).expect(format!("Cannot get axis: {}", axis).as_str());
//         let layers = LAYERS_DATA.read()?.clone();
//         let paddings = layers.get(axis).expect(format!("Cannot get axis: {}", axis).as_str());
// 
//         let data = CountAxisData {
//             max_position: monitor_axis.max_point - paddings.padding_max - cli_axis.window_size,
//             window_center: cli_axis.cursor_pos - (cli_axis.window_size / 2),
//             monitor_resolution: monitor_axis.max_point - monitor_axis.min_point,
//         };
// 
//         axis_map.insert(axis.to_string(), data);
//     }
//     Ok(axis_map)
// }
