use hyprland::dispatch::{
    Dispatch,
    DispatchType::{
        Exec,
    },
};

use std::{
    fs,
    fs::File,
    path::Path,
    io::{Read, Write}
};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, RwLock};
use clap::{Parser, Subcommand};
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use serde::{Deserialize, Serialize};
use toml::ser;
use crate::{empty_parameters, check_config_file, change_window_state, update_data, get_daemon_data, origin_position, GIVE_ARGS, CONFIG_DATA, PARAMETERS, BIN, CACHE_DIR, CLASS_CACHE_FILE, SIZE_CACHE_FILE, CLASS, POSITION_VALUES, CLIENT_DATA, Parameters, get_table, CONFIG_FILE, IfPercent, WINDOWS_CONFIG_DATA, get_parameter, Modes, notify_error};
use crate::ExecArgs;




// #[derive(Parser, Debug, Clone)]
// #[command(about, long_about = None, ignore_errors = false)]
// pub struct ExecArgs {
//     /// Program to run (Example: "nautilus --new-window")
//     #[arg()]
//     pub executable: String,
//     /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
//     #[arg(short, long, default_value_t = false)]
//     pub force: bool,
//     /// Resize window according to config parameter 'default_size'
//     #[arg(short, long, default_value_t = false)]
//     pub default_size: bool,
//     /// Open small window and then resize it
//     #[arg(short, long, default_value_t = false)]
//     pub origin_size: bool,
//     /// Open window floating, then tile
//     #[arg(short, long, default_value_t = false)]
//     pub tile: bool,
//     /// Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>
//     #[arg(short, long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
//     pub size: Vec<String>,
//     /// Set window open position by x-axis to <POS_X>, by y-axis to <POS_Y>
//     #[arg(short, long, num_args = 2, value_names = ["AT_X", "AT_Y"])]
//     pub at: Vec<i16>,
//     /// Open window according to <POSITION> value
//     #[arg(short, long, default_value_t = String::from("any"), hide_default_value = false, value_parser = POSITION_VALUES.clone())]
//     pub position: String,
//     // /// Path to config file
//     // #[arg(short, long, default_value_t = CONFIG_FILE.clone())]
//     // pub config: String,
// }


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClassSize {
    pub size: (i16, i16)
}


pub fn get_toml_data_from_cache(key: &str, file_path: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
    let dir_path = Path::new(CACHE_DIR.as_str());
    fs::create_dir_all(dir_path).unwrap();

    let mut file = match File::open(file_path) {
        Ok(data) => data,
        Err(_) => {
            let path_buf = dir_path.join(file_path);
            let file = File::create(path_buf.clone()).unwrap();
            file
        }
    };

    let mut contents = String::new();


    file.read_to_string(&mut contents)?;

    let toml_value: serde_json::Value = serde_json::from_str(&contents).unwrap();

    println!("toml tablce: {:?}", toml_value.get(key));

    match toml_value.get(key){
        Some(data) => {
            println!("Data: {:?}", data);
            Ok(Some(data.clone()))
        },
        None => Ok(None)
    }
}


fn check_file(dir_path: String, file_path: String) -> Result<File, Box<dyn std::error::Error>> {
    let cache_dir_path = Path::new(dir_path.as_str());
    fs::create_dir_all(cache_dir_path)?;


    let  file = match File::open(file_path.clone()) {
        Ok(data) => data,
        Err(_) => {
            let path_buf = cache_dir_path.join(file_path.clone());
            let file = File::create(path_buf.clone())?;
            // fs::write(
            //     path_buf.clone(),
            //     "{}"
            // ).unwrap();
            file
        }
    };

    Ok(file)
}


pub fn write_class_data_into_cache(bin: String, mut class: String) -> Result<(), Box<dyn std::error::Error>> {

    let cli = CLIENT_DATA.read()?.clone();

    if class.is_empty() {
        println!("Origin size goo");
        notify_error("Origin size goo".to_string());
        PARAMETERS.write().unwrap().origin_size = true;

        class = cli.class;

        let _ = check_file(CACHE_DIR.clone(), CLASS_CACHE_FILE.clone())?;


        let class_str = fs::read_to_string(CLASS_CACHE_FILE.clone().as_str())?;
        let mut class_map: HashMap<String, serde_json::Value> = serde_json::from_str(&class_str)
            .unwrap_or_else(
                |e| {
                    fs::write(
                        CLASS_CACHE_FILE.clone(),
                        "{}"
                    ).unwrap();

                    let class_str = fs::read_to_string(CLASS_CACHE_FILE.clone().as_str()).unwrap();
                    serde_json::from_str(class_str.as_str()).unwrap()

                }
            );

        class_map.insert(
            bin,
            serde_json::Value::String(class.clone())
        );

        fs::write(
            CLASS_CACHE_FILE.clone(),
            serde_json::to_string_pretty(&class_map)?
        )?;
    }


    check_file(CACHE_DIR.clone(), SIZE_CACHE_FILE.clone())?;
    let size_str = fs::read_to_string(SIZE_CACHE_FILE.clone().as_str())?;

    let mut existing_data: serde_json::Value = serde_json::from_str(size_str.as_str()).unwrap_or_else(|e| {
        fs::write(
            SIZE_CACHE_FILE.clone(),
            "{}"
        ).unwrap();

        let size_str = fs::read_to_string(SIZE_CACHE_FILE.clone().as_str()).unwrap();
        serde_json::from_str(size_str.as_str()).unwrap()
    });

    let new_data = ClassSize {
        size: (
            cli.axis_data.get("x").expect(format!("Cannot get {}-axis from client data", "x").as_str()).window_size,
            cli.axis_data.get("y").expect(format!("Cannot get {}-axis from client data", "y").as_str()).window_size,
        )
    };


    let new_str_data = serde_json::to_string(&new_data)?;
    let new_value_data: serde_json::Value = serde_json::from_str(&new_str_data)?;
    existing_data[class.as_str()] = new_value_data;

    let updated_json = serde_json::to_string_pretty(&existing_data)?;
    fs::write(SIZE_CACHE_FILE.clone(), &updated_json)?;

    Ok(())
}



fn cut_first_last_iter(s: &str) -> String {
    if s.len() <= 2 {
        String::new()
    } else {
        s.chars().skip(1).take(s.len() - 2).collect()
    }
}


////////////////////////


pub fn get_origin_size(axis: &str) -> i16 {
    println!("Get origin size");
    let params = PARAMETERS.read().unwrap().clone();
    let size_params = params.size.clone().unwrap_or_else(|| HashMap::new());

    println!("{:?}", size_params);

    if size_params.contains_key(axis) {
        size_params.get(axis).unwrap().clone()
    } else if PARAMETERS.read().unwrap().clone().default_size == true {
        CONFIG_DATA.read().unwrap().axis_data.get(axis).unwrap().default_size
    } else {
        println!("No default size for axis. Size params: {:?}", size_params.clone());

        get_parameter(
            format!("origin_{}", axis).as_str(),
            Arc::new(RwLock::new(size_params.clone())),
            20
        )
    }
}


fn define_parameters(parsed_args: ExecArgs) -> Result<(), Box<dyn std::error::Error>> {
    get_daemon_data()?;

    let mut output_params = empty_parameters();

    output_params.binary = "hfopen".to_string();
    output_params.position = parsed_args.position;
    output_params.tile = parsed_args.tile;
    output_params.origin_size = parsed_args.origin_size;
    output_params.default_size = parsed_args.default_size;

    if parsed_args.default_size {
        output_params.origin_size = true
    }
    if parsed_args.force {
        CONFIG_DATA.write()?.detect_padding = false
    }

    if parsed_args.size.len() > 0 {
        let list =[("x", parsed_args.size[0].as_str()), ("y", parsed_args.size[1].as_str())];
        output_params.size_from_args = true;
        let mut map: HashMap<String, i16> = HashMap::new();

        for i in list {
            map.insert(i.0.to_string(), i.1.to_string().check_percent(i.0));
        }
        output_params.size = Some(map);
    }
    if parsed_args.at.len() > 0 {
        let list = [("x", parsed_args.at[0]), ("y", parsed_args.at[1])];
        let mut map: HashMap<String, i16> = HashMap::new();
                
        for i in list {
            map.insert(i.0.to_string(), i.1);
        }
        
        output_params.at = Some(map);
    }

    *PARAMETERS.write()? = output_params;

    Ok(())
}




fn get_class_from_request(request: Vec<String> ) -> Result<String, Box<dyn std::error::Error>> {
    println!("Get class from request");
    let executable = request[ request.len() -1].as_str();
    let executable_vec = executable.split_whitespace().collect::<Vec<&str>>();
    let binary = executable_vec[0];

    *BIN.write()? = binary.to_string();

    println!("Binary: {}", binary);
    let mut output_class: String ; //  = String::from("");

    for (i, word) in  executable_vec[1..].iter().enumerate() {
        println!("Word: {}", word);
        if *word == "--class" || *word == "--app-id" {
            output_class = executable_vec[i + 2].to_string().clone();
            return Ok(output_class)
        }
    }

    let mut file = check_file(CACHE_DIR.clone(), CLASS_CACHE_FILE.clone())?;

    println!("After file");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(x) => {},
        Err(e) => {
            println!("Error reading to string: {}", e);
            contents = "{}".to_string();
        }
    };
    // .unwrap_or_else(
    //     |e| {
    //
    //     }
    // );

    print!("After content");

    let toml_data: serde_json::Value = serde_json::from_str(&contents).unwrap_or_else(
        |e| {
            fs::write(
                CLASS_CACHE_FILE.clone(),
                "{}"
            ).unwrap();

            let class_str = fs::read_to_string(CLASS_CACHE_FILE.clone().as_str()).unwrap();
            serde_json::from_str(class_str.as_str()).unwrap()
        }
    );
    output_class = match toml_data.get(binary) {
        Some(data) => data.as_str().unwrap().to_string(),
        None => {
            "".to_string()
        }
    };

    Ok(output_class)
}


pub fn get_size_data_by_class(class: String) -> Result<Option<(i16, i16)>, Box<dyn std::error::Error>> {
    let mut file = check_file(CACHE_DIR.clone(), SIZE_CACHE_FILE.clone())?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let toml_data: serde_json::Value  = serde_json::from_str(&contents)?;

    match toml_data.get(class.as_str()) {
        Some(data) => {
            let output_size: ClassSize = serde_json::from_str(
                &serde_json::to_string(data)?
            )?;

            Ok(Some(output_size.size))
        },
        None => Ok(None)
    }
}


pub fn predefine_size_by_class(mut class: String) -> Result<(i16, i16), Box<dyn std::error::Error>> {
    let cli = CLIENT_DATA.read().unwrap().clone();
    let conf = CONFIG_DATA.read().unwrap().clone();
    let params = PARAMETERS.read()?.clone();
    let monitor_id = cli.clone().monitor;
    // let windows_section = get_table("windows", CONFIG_FILE.read().unwrap().clone().as_str());
    // let config_table: toml::Table = windows_section.as_table().unwrap().clone();
    let config_table = WINDOWS_CONFIG_DATA.read()?.clone();
    
    if config_table.keys().collect::<Vec<_>>().iter().any(|&x| x == &class.clone())
        && ((conf.toggle_to_open_size && params.binary == "hftogglefloating") || params.binary == "hfopen") {
        println!("Config contains class");

        let class_section = config_table[&class].clone();
        let current_monitor_value = format!("{}{}", "monitor_", monitor_id);
        let class_section_keys: Vec<&str> = vec![
            current_monitor_value.as_str(),
            "monitor_any"
        ];

        println!("class_section_keys: {:?}", class_section_keys);

        for key in class_section_keys {
            if class_section
                .as_table()
                .unwrap()
                .keys()
                .collect::<Vec<_>>()
                .contains(&&key.to_string()) {
                println!("In");
                use std::str::FromStr;

                let param_vec = &class_section
                    .as_table().
                    unwrap()[key]
                    .as_array()
                    .unwrap();
                // let val_param_vec = toml::Value::Array(param_vec);

                let output_tuple = (
                    param_vec[0].clone().check_percent("x").clone() as u16 as i16,
                    param_vec[1].clone().check_percent("y").clone() as u16 as i16
                );

                println!("Set size from config");
                
                return Ok(output_tuple);
            }
        }
    } else {
        println!("Class not in config");
    }

    
    if params.count_system == "origin" || params.binary == "hftogglefloating" {
        println!("Getting from cache");
        
        match get_size_data_by_class(class.clone())? {
            Some(tuple) => {
                PARAMETERS.write()?.size_from_args = true;
                Ok(tuple)
            },
            None => {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No data in cache or config"
                )))
            }
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not getting data from cache"
        )))
    }
}


pub fn exec(exec_args: Modes) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args: ExecArgs = match exec_args {
        Modes::Exec {
            executable ,
            force,
            default_size,
            origin_size,
            tile,
            size,
            at,
            position
        } => ExecArgs {
                executable ,
                force,
                default_size,
                origin_size,
                tile,
                size,
                at,
                position
            },
        _ => panic!("Cannot parse Exec enum into ExecArgs")
    };


    println!("Exec Parsed args: {:?}", parsed_args);

    update_data();

    let _ = match define_parameters(parsed_args.clone()) {
        Ok(x) =>   x,
        Err(e) => {
            eprintln!("Cannot define parameters: {}", e);
            exit(1)
        }
    };

    let request = parsed_args.executable.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();

    let class = get_class_from_request(request)?;
    *CLASS.write()? = class.clone();

       if !parsed_args.default_size && parsed_args.size.len() == 0 {
        match predefine_size_by_class(class) {
            Ok(insert_tuple) => {
                let mut map: HashMap<String, i16> = HashMap::new();
                
                map.insert("origin_x".to_string(), insert_tuple.0);
                map.insert("origin_y".to_string(), insert_tuple.1);
                
                PARAMETERS.write()?.size = Some(map)
            },
            Err(e) => {
                println!("No data in cache or config {}", e);
            }
        }
    }




    let params = PARAMETERS.read()?.clone();

    let mut origin_size = "".to_string();
    if params.origin_size ||
        (!params.size_from_args &&
            !CLASS.read()?.clone().is_empty()
        )  
    {
        origin_size = format!(
            "size {} {}",
            get_origin_size("x"),
            get_origin_size("y"),
        );
    }

    println!("Origin size: {}", origin_size);


    let origin_position = format!(
        "move {} {}",
        origin_position("x"),
        origin_position("y"),
    );

    println!("Position outside: {}", origin_position);

    let exec_content =  format!(
        "[{};{};{};] {}",
        "float",
        origin_position,
        origin_size,
        parsed_args.executable
    );

    println!("Exec content: {}", exec_content);

    let _ = Dispatch::call(Exec(
       exec_content.as_str()
    ));

    change_window_state();
    
    Ok(())
}
