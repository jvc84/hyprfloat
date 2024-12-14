use std::{
    fs,
    fs::{OpenOptions, File},
    path::Path,
    io::{Read, Write}
};
use clap::Parser;
use crate::{
    check_config_file,
    add_size_values,
    BIN,
    CACHE_DIR,
    CACHE_FILE,
    CLASS,
    CONFIG_FILE,
    POSITION_VALUES,
    PARAMETERS, CLIENT_DATA,
};


#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, ignore_errors = false)]
pub struct Args {
    /// Program to run (Example: "nautilus --new-window")
    #[arg()]
    pub executable: String,
    /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
    /// Resize window according to config parameter 'default_size'
    #[arg(short, long, default_value_t = false)]
    pub default_size: bool,
    /// Open small window and then resize it
    #[arg(short, long, default_value_t = false)]
    pub origin_size: bool,
    /// Open window floating, then tile
    #[arg(short, long, default_value_t = false)]
    pub tiled: bool,
    /// Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>
    #[arg(short, long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
    pub size: Vec<u16>,
    /// Set window open position by x-axis to <POS_X>, by y-axis to <POS_Y>
    #[arg(short, long, num_args = 2, value_names = ["AT_X", "AT_Y"])]
    pub at: Vec<i16>,
    /// Open window according to <POSITION> value
    #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
    pub position: String,
    /// Path to config file
    #[arg(short, long, default_value_t = CONFIG_FILE.clone())]
    pub config: String,
}


pub fn get_cached_class(bin: &str, file_path: &str) -> String {
    let dir_path = Path::new(CACHE_DIR.as_str());
    fs::create_dir_all(dir_path).unwrap();

    let mut file = match File::open(file_path) {
        Ok(data) => data,
        Err(_) => {
            let file_path = dir_path.join(crate::CLASSES_CACHE_FLIE.as_str());
            let file = File::create(file_path.clone()).unwrap();
            file
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => file.read_to_string(&mut contents).unwrap(),
        Err(_) => return "".to_string()
    };
    let toml_value: toml::Value = toml::from_str(&contents).unwrap();

    let output: String = match toml_value.get(bin) {
        Some(x) => x.as_str().unwrap().to_string(),
        None => "".to_string()
    };

    match output.as_str() {
        "null" => "".to_string(),
        _ => output
    }
}


pub fn check_class() {
    let cache_file = CACHE_FILE.clone();
    let class = CLASS.read().unwrap().clone();
    let bin = BIN.read().unwrap().clone();
    
    if  class.is_empty() && !bin.is_empty() {
        let file_str = check_config_file(cache_file.clone().as_str());
        let mut toml_data= toml::from_str::<toml::Table>(&file_str).unwrap();
        toml_data.insert(bin, toml::Value::String(CLIENT_DATA.read().unwrap().clone().class));
        let toml_string = toml::to_string(&toml_data).unwrap();

        let mut file = OpenOptions::new().read(true).write(true).open(cache_file).unwrap();

        file.set_len(0).unwrap(); 
        file.write_all(toml_string.as_bytes()).unwrap();
    }
}

pub fn parse_class(parsed_args: Args)  {
    let exec_list = parsed_args.executable.split_whitespace().collect::<Vec<&str>>();
    let bin = exec_list[0];
    let mut class = "".to_string();
    for (i, arg) in exec_list.clone().iter().enumerate() {
        match *arg {
            "--class" | "--app-id" => {
                class = exec_list[i+1].to_string();
                break
            },
            _ => continue
        }
    }

    if class.clone().is_empty() {
        class = get_cached_class(bin, CACHE_FILE.clone().as_str()).to_string();
    }

    if !class.clone().is_empty() && parsed_args.size.len() == 0 {
        PARAMETERS.write().unwrap().config_pre_size = true;
        add_size_values("".to_string(), class.clone());
    }

    *CLASS.write().unwrap() = class.clone();
    *BIN.write().unwrap() = bin.to_string();
}
