use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::{CACHE_DIR, CLASS_CACHE_FILE};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cache {
    pub class: String,
    pub size: Vec<u16>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyVec {
    pub vec: Vec<i16>
}




// fn get_class_from_request(request: &str ) -> Result<String, Box<dyn std::error::Error>> {
//     let request_vec = request.split(" ").collect::<Vec<&str>>(); 
//     let binary= request_vec[0];
//     let mut output_class  = String::from("");
//     
//     for (i, word) in  request_vec[1..].iter().enumerate() {
//         if *word == "--class" || *word == "--app-id" {
//             output_class = request_vec[i + 1].to_string();
//             return Ok(output_class)
//         }
//     }
//     
//     
//     let cache_dir_path = Path::new(CACHE_DIR.as_str());
//     fs::create_dir_all(cache_dir_path)?;
//     
//     let file_path = CLASS_CACHE_FILE.clone();
//     
//     let mut file = match File::open(file_path.clone()) {
//         Ok(data) => data,
//         Err(_) => {
//             let path_buf = cache_dir_path.join(file_path.clone());
//             let file = File::create(path_buf.clone())?;
//             file
//         }
//     };
//     
//     let mut contents = String::new();
//     
//     file.read_to_string(&mut contents)?;
//     
//     let toml_data = toml::Table::try_from(contents)?;
//     
//     output_class = match toml_data.get(binary) {
//         Some(data) => data.to_string(),
//         None => {
//             "".to_string()
//         }
//     };
//     
//     
//     Ok(output_class)
// }



// pub fn get_cache_data_from_cache(key: &str, file_path: &str) -> Result<Option<toml::value::Value>, Box<dyn std::error::Error>> {
//     let dir_path = Path::new(CACHE_DIR.as_str());
//     fs::create_dir_all(dir_path).unwrap();
// 
//     let mut file = match File::open(file_path) {
//         Ok(data) => data,
//         Err(_) => {
//             let path_buf = dir_path.join(file_path.clone());
//             let file = File::create(path_buf.clone()).unwrap();
//             file
//         }
//     };
// 
//     let mut contents = String::new();
// 
// 
//     file.read_to_string(&mut contents)?;
// 
//     let toml_value: toml::Table = toml::from_str(&contents).unwrap();
// 
//     println!("toml table: c{:?}", toml_value.get(key));
// 
//     match toml_value.get(key){
//         Some(data) => {
//             println!("Data: {:?}", data);
//             Ok(Some(data.clone()))
//         },
//         None => Ok(None)
//     }
// }
// 
// 
// pub fn write_class_data_into_cache() {
//     let class_cache_file = CLASS_CACHE_FILE.clone();
//     let size_cache_file = SIZE_CACHE_FILE.clone();
//     let mut class = CLASS.read().unwrap().clone();
//     let bin = BIN.read().unwrap().clone();
//     let cli = CLIENT_DATA.read().unwrap().clone();
//     let cli_x = cli.axis_data.get("x").unwrap().clone();
//     let cli_y = cli.axis_data.get("y").unwrap().clone();
// 
//     println!("Class: {},  Bin: {}", class, bin);
// 
//     if class.is_empty() && !bin.is_empty() {
//         class = CLIENT_DATA.read().unwrap().clone().class;
//         let classes_file_str = check_config_file(class_cache_file.clone().as_str());
//         let mut classes_toml_data = toml::from_str::<toml::Table>(&classes_file_str).unwrap();
// 
//         println!("Class to insert: {}", class.clone());
//         println!("Toml data: {}", classes_toml_data.clone().to_string());
//         classes_toml_data.insert(
//             bin,
//             toml::Value::String(class.clone())
//         );
// 
// 
//         let toml_string = toml::to_string(&classes_toml_data).unwrap();
// 
//         let mut file = OpenOptions::new().read(true).write(true).open(class_cache_file).unwrap();
// 
//         file.set_len(0).unwrap();
//         file.write_all(toml_string.as_bytes()).unwrap();
//     }
// 
//     let sizes_file_str = check_config_file(size_cache_file.clone().as_str());
//     let mut sizes_toml_data = toml::from_str::<toml::Table>(&sizes_file_str).unwrap();
// 
// 
//     let insert_vec   = MyVec {
//         vec: vec![
//             cli_x.window_size,
//             cli_y.window_size
//         ]
//     };
// 
//     println!("Class before insert: {}", class);
// 
//     sizes_toml_data.insert(
//         class.clone(),
//         toml::from_str(
//             toml::to_string(&insert_vec).unwrap().as_ref()
//         ).unwrap()
//     );
// 
//     println!("Toml data: {}", toml::to_string(&sizes_toml_data).unwrap());
//     let sizes_toml_string = toml::to_string(&sizes_toml_data).unwrap();
// 
//     let mut file = OpenOptions::new().read(true).write(true).open(size_cache_file).unwrap();
// 
//     file.set_len(0).unwrap();
//     file.write_all(sizes_toml_string.as_bytes()).unwrap();
// }
// 
// 
// fn cut_first_last_iter(s: &str) -> String {
//     if s.len() <= 2 {
//         String::new()
//     } else {
//         s.chars().skip(1).take(s.len() - 2).collect()
//     }
// }
// 
// 
// fn get_class_of_executable(parsed_args: crate::hfopen::Args) -> Result<(), Box<dyn std::error::Error>> {
//     let exec_list = parsed_args.executable.split_whitespace().collect::<Vec<&str>>();
//     let bin = exec_list[0];
//     let mut class = "".to_string();
//     for (i, arg) in exec_list.clone().iter().enumerate() {
//         match *arg {
//             "--class" | "--app-id" => {
//                 class = exec_list[i+1].to_string();
//                 break
//             },
//             _ => continue
//         }
//     }
// 
// 
//     if class.clone().is_empty() {
//         class = match get_cache_data_from_cache(bin, CLASS_CACHE_FILE.clone().as_str())? {
//             Some(data) => {
//                 let pre_class: toml::Value = data.clone();
// 
//                 pre_class.to_string()
//             },
//             None => {"".to_string()}
//         };
//     } else {
//         *CLASS.write()? = class.clone();
//         println!("AFter Write class: {:?}", CLASS.read().unwrap());
//         PARAMETERS.write()?.window_pre_size = true;
//         set_origin_size_from_parameters("".to_string(), class.clone());
//     }
// 
//     println!("After cache file: {}", class);
// 
//     if !class.clone().is_empty() && parsed_args.size.len() == 0 && !parsed_args.default_size {
//         println!("CLASS NOT EMPTY");
//         *CLASS.write().unwrap() = class.clone();
//         *BIN.write().unwrap() = bin.to_string();
// 
// 
//         PARAMETERS.write()?.window_pre_size = true;
//         set_origin_size_from_parameters("".to_string(), class.clone());
//         println!("Size parametes before open : {:#?}", SIZE_PARAMETERS.read()?.clone())
//     } else {
//         println!("CLASS EMPTY");
//         *BIN.write().unwrap() = bin.to_string();
//         // *CLASS.write()? = class.clone();
//     }
//     println!("Bin after write: {}", bin.to_string());
//     Ok(())
// }

