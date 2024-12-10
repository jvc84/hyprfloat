use lazy_static::lazy_static;
use hyprland::dispatch::{
    Dispatch,
    DispatchType::{
        Exec,
    },
};
use hyprfloat::{
    Args,
    parse_class,
    change_window_state,
    update_data,
    config_data,
    origin_position,
    get_origin_size,
    CLASS,
    PARAMETERS,
    CONFIG_DATA,
    SIZE_PARAMETERS,
    AT_PARAMETERS
};
use clap::Parser;



lazy_static! {
    static ref PARSED_ARGS: Args = Args::parse();
}


fn define_parameters() {
    let parsed_args = PARSED_ARGS.clone();

    *CONFIG_DATA.write().unwrap() = config_data(parsed_args.config);
    PARAMETERS.write().unwrap().binary = "hfopen".to_string();
    PARAMETERS.write().unwrap().dispatcher_arg = parsed_args.position;
    PARAMETERS.write().unwrap().tiled = parsed_args.tiled;
    PARAMETERS.write().unwrap().origin_size = parsed_args.origin_size;
    PARAMETERS.write().unwrap().default_size = parsed_args.default_size;
    
    if parsed_args.default_size {
        PARAMETERS.write().unwrap().origin_size = true
    }
    if parsed_args.force {
        CONFIG_DATA.write().unwrap().detect_padding = false
    }
    if parsed_args.size.len() > 0 {
        let list =[("x", parsed_args.size[0]), ("y", parsed_args.size[1])];
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
}


fn main() {
    let parsed_args = PARSED_ARGS.clone();
    define_parameters();
    update_data();
    parse_class(parsed_args.clone());
    
    let params = PARAMETERS.read().unwrap().clone();
    
    let mut origin_size = "".to_string();
    if params.origin_size ||
        (params.config_pre_size &&
            !CLASS.read().unwrap().clone().is_empty()
        )  
    {
        origin_size = format!(
            "size {} {}",
            get_origin_size("x"),
            get_origin_size("y"),
        );
    }

    let origin_position = format!(
        "move {} {}",
        origin_position("x"),
        origin_position("y"),
    );
    
    let _ = Dispatch::call(Exec(
        format!(
            "[{};{};{}] {}",
            "float",
            origin_position,
            origin_size,
            parsed_args.executable
        ).as_str()
    ));

    change_window_state();
}
