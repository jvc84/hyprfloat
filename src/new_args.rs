use std::ffi::OsStr;
use clap::{Arg, Command, Parser, Subcommand};
use crate::{CONFIG_FILE, POSITION_VALUES};

pub fn test_args() {
    let matches = Command::new("nfuni")
        .subcommand(
            Command::new("daemon")
                .about("Daemon")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("PATH")
                        .help("Path co config file 'hf.toml'")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("exec")
                .about("Exec")
                .arg(
                    Arg::new("executable")
                        .value_name("EXECUTABLE")
                        .help("Program to execute (Example: \"nautilus --new-window\")")
                        .required(false)
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Do not detect padding, even if 'detect_padding' option in config equals 'true'")
                        .required(false)

                )
                .arg(
                    Arg::new("default_size")
                        .short('d')
                        .long("default-size")
                        .help("Resize window according to config parameter 'default_size'")
                        .required(false)
                )
                .arg(
                    Arg::new("origin_size")
                        .short('o')
                        .long("origin-size")
                        .help("Open small window and then resize it")
                        .required(false)
                )
                .arg(
                    Arg::new("tile")
                        .short('t')
                        .long("tile")
                        .help("Open window floating, then tile")
                        .required(false)
                )
                .arg(
                    Arg::new("size")
                        .short('s')
                        .long("size")
                        .help("Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>")
                        .number_of_values(2)
                        .value_names(["SIZE_X", "SIZE_Y"])
                        .required(false)
                )
                .arg(
                    Arg::new("at")
                        .short('a')
                        .long("at")
                        .help("Set window open position by x-axis to <AT_X>, by y-axis to <AT_Y>")
                        .number_of_values(2)
                        .value_names(["AT_X", "AT_Y"])
                        .required(false)
                )
                .arg(
                    Arg::new("position")
                        .short('p')
                        .long("position")
                        .help("Open window according to <POSITION> value")
                        .value_name("POSITION")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("togglefloating")
                .about("Togglefloating")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Do not detect padding, even if 'detect_padding' option in config equals 'true'")
                        .required(false)

                )
                .arg(
                    Arg::new("default_size")
                        .short('d')
                        .long("default-size")
                        .help("Resize window according to config parameter 'default_size'")
                        .required(false)
                )
                .arg(
                    Arg::new("size")
                        .short('s')
                        .long("size")
                        .help("Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>")
                        .number_of_values(2)
                        .value_names(["SIZE_X", "SIZE_Y"])
                        .required(false)
                )
                .arg(
                    Arg::new("at")
                        .short('a')
                        .long("at")
                        .help("Set window open position by x-axis to <AT_X>, by y-axis to <AT_Y>")
                        .number_of_values(2)
                        .value_names(["AT_X", "AT_Y"])
                        .required(false)
                        // .value_type(Vec<i16>)
                )
                .arg(
                    Arg::new("position")
                        .short('p')
                        .long("position")
                        .help("Open window according to <POSITION> value")
                        .value_name("POSITION")
                        .required(false)
                        // .value_type(String)
                )
        )
        .subcommand(
            Command::new("resizeactive")
                .about("Togglefloating")
                .arg(
                    Arg::new("resize_x")
                        .help("Resize window by x-axis on <RESIZE_X> pixels according to config parameters")
                        .allow_hyphen_values(true)
                        .allow_negative_numbers(true)
                        .required(true)
                        // .value_type(u16)
                )
                .arg(
                    Arg::new("resize_y")
                        .help("Resize window by y-axis on <RESIZE_Y> pixels according to config parameters")
                        .allow_hyphen_values(true)
                        .allow_negative_numbers(true)
                        .required(true)
                        // .value_type(u16)
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Do not detect padding, even if 'detect_padding' option in config equals 'true'")
                        .required(false)

                )
                // .arg(
                //     Arg::new("default_size")
                //         .short('d")
                //         .long("default-size")
                //         .help("Resize window according to config parameter 'default_size'")
                //         .required(false)
                // )
                .arg(
                    Arg::new("no_invert")
                        .short('n')
                        .long("no-invert")
                        .help("Do not invert resize in stick mode, even if 'invert_resize_in_stick_mode' option in config equals 'true'")
                        // .default_value(OsStr::new(false))
                        .required(false)
                )
                .arg(
                    Arg::new("exact")
                        .short('e')
                        .long("exact")
                        .help("Set size of floating window exactly <RESIZE_X> pixels on x-axis, <RESIZE_Y> pixels on y-axis")
                        // .default_value(false)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("movewindow")
                .about("Movewindow")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Do not detect padding, even if 'detect_padding' option in config equals 'true'")
                        .required(false)
                )
                .arg(
                    Arg::new("direction")
                        .short('d')
                        .long("direction")
                        .help("Direction to move window to")
                        .default_value("l")
                        .hide_default_value(true)
                        .value_parser(["l", "r", "u", "d"])
                        .required(false)
                        )
                .arg(
                    Arg::new("position")
                        .short('p')
                        .long("position")
                        .help("Open window according to <POSITION> value")
                        .value_name("POSITION")
                        .required(false)
                        // .value_type(String)
                )
        )
        .get_matches();
}


//     Movewindow {
//         /// Open window according to <POSITION> value
//         #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
//         position: String,
//     },
// }
//