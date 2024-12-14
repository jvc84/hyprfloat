use hyprfloat::{
    change_window_state,
    config_data,
    update_data,
    CONFIG_FILE,
    POSITION_VALUES,
    SIZE_PARAMETERS,
    AT_PARAMETERS,
    PARAMETERS,
    CONFIG_DATA,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, ignore_errors = false)]
struct Args {
    /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Resize window according to config parameter 'default_size'
    #[arg(short, long, default_value_t = false)]
    default_size: bool,
    /// Set window size by x axis to <SIZE_X>, by y axis to <SIZE_Y>
    #[arg(short, long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
    size: Vec<u16>,
    /// Set window open position by x axis to <POS_X>, by y axis to <POS_Y>
    #[arg(short, long, num_args = 2, value_names = ["AT_X", "AT_Y"])]
    at: Vec<i16>,
    /// Open window according to <POSITION> value
    #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
    position: String,
    /// Path to config file
    #[arg(short, long, default_value_t = CONFIG_FILE.clone())]
    config: String,
}

fn main() {

    let parsed_args = Args::parse();
    PARAMETERS.write().unwrap().binary = "hftogglefloating".to_string();


    *CONFIG_DATA.write().unwrap() = config_data(parsed_args.config);
    PARAMETERS.write().unwrap().dispatcher_arg = parsed_args.position;
    PARAMETERS.write().unwrap().default_size = parsed_args.default_size;
    if parsed_args.default_size {
        PARAMETERS.write().unwrap().origin_size = true
    }

    if parsed_args.force {
        CONFIG_DATA.write().unwrap().detect_padding = false
    }

    if parsed_args.size.len() > 0 {
        let list = [("x", parsed_args.size[0]), ("y", parsed_args.size[1])];
        PARAMETERS.write().unwrap().origin_size = true;

        for i in list {
            SIZE_PARAMETERS.write().unwrap().insert(i.0.to_string(), i.1 as i16);
        }
    }

    if parsed_args.at.len() > 0 {
        let list = [("x", parsed_args.at[0]), ("y", parsed_args.at[1])];
    
        for i in list {
            AT_PARAMETERS.write().unwrap().insert(i.0.to_string(), i.1);
        }
    }

    update_data();
    change_window_state();
}