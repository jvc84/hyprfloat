use clap::{Arg, Command, Parser, Subcommand};
use crate::{CONFIG_FILE, POSITION_VALUES};




#[derive(Debug, Clone)]
pub struct DaemonArgs {
    pub config: String,
}

#[derive(Debug, Clone)]
pub struct ExecArgs {
    pub executable: String,
    pub force: bool,
    pub default_size: bool,
    pub origin_size: bool,
    pub tile: bool,
    pub size: Vec<String>,
    pub at: Vec<i16>,
    pub position: String,
}

#[derive(Debug, Clone)]
pub struct TogglefloatingArgs {
    pub force: bool,
    pub default_size: bool,
    pub size: Vec<u16>,
    pub at: Vec<i16>,
    pub position: String,
}

#[derive(Debug, Clone)]
pub struct ResizeactiveArgs {
    pub resize_x: String,
    pub resize_y: String,
    pub force: bool,
    pub no_invert: bool,
    pub exact: bool,
}

#[derive(Debug, Clone)]
pub struct MovewindowArgs {
    pub force: bool,
    pub direction: String,
    pub position: String,
}


#[derive(Debug, Clone, Subcommand)]
// #[subcommand( about, long_about = None, ignore_errors = false)]
pub enum Modes {
    Daemon {
        #[arg(short, long, default_value_t = CONFIG_FILE.read().unwrap().clone())]
        config: String,
    },
    Exec {
        /// Program to execute (Example: "nautilus --new-window")
        #[arg()]
        executable: String,
        /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
        #[arg(short, long, default_value_t = false)]
        force: bool,
        /// Resize window according to config parameter 'default_size'
        #[arg(short, long, default_value_t = false)]
        default_size: bool,
        /// Open small window and then resize it
        #[arg(short, long, default_value_t = false)]
        origin_size: bool,
        /// Open window floating, then tile
        #[arg(short, long, default_value_t = false)]
        tile: bool,
        /// Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>
        #[arg(short, long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
        size: Vec<String>,
        /// Set window open position by x-axis to <AT_X>, by y-axis to <AT_Y>
        #[arg(short, long, num_args = 2, value_names = ["AT_X", "AT_Y"])]
        at: Vec<i16>,
        /// Open window according to <POSITION> value
        #[arg(short, long, default_value_t = String::from("any"), hide_default_value = false, value_parser = POSITION_VALUES.clone())]
        position: String,
    },
    Togglefloating {
        /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
        #[arg(short, long, default_value_t = false)]
        force: bool,
        /// Resize window according to config parameter 'default_size'
        #[arg(short, long, default_value_t = false)]
        default_size: bool,
        /// Set window size by x-axis to <SIZE_X>, by y-axis to <SIZE_Y>
        #[arg(short, long, num_args = 2, value_names = ["SIZE_X", "SIZE_Y"])]
        size: Vec<u16>,
        /// Set window open position by x-axis to <AT_X>, by y-axis to <AT_Y>
        #[arg(short, long, num_args = 2, value_names = ["AT_X", "AT_Y"])]
        at: Vec<i16>,
        /// Open window according to <POSITION> value
        #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
        position: String,
    },
    Resizeactive {
        /// resize window by x-axis on <RESIZE_X> pixels according to config parameters
        #[arg(allow_negative_numbers = true, allow_hyphen_values = true)]
        resize_x: String,
        /// resize window by y-axis on <RESIZE_Y> pixels according to config parameters
        #[arg(allow_negative_numbers = true, allow_hyphen_values = true)]
        resize_y: String,
        /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
        #[arg(short, long, default_value_t = false)]
        force: bool,
        /// Do not invert resize in stick mode, even if 'invert_resize_in_stick_mode' option in config equals 'true'
        #[arg(short, long, default_value_t = false)]
        no_invert: bool,
        /// Set size of floating window exactly <RESIZE_X> pixels on x-axis, <RESIZE_Y> pixels on y-axis   
        #[arg(short, long, default_value_t = false)]
        exact: bool,
    },
    Movewindow {
        /// Do not detect padding, even if 'detect_padding' option in config equals 'true'
        #[arg(short, long, default_value_t = false)]
        force: bool,
        /// Direction to move window to
        #[arg(default_value_t = String::from("l"), hide_default_value = true, value_parser = ["l", "r", "u", "d"])]
        direction: String,
        /// Open window according to <POSITION> value
        #[arg(short, long, default_value_t = String::from("any"), hide_default_value = true, value_parser = POSITION_VALUES.clone())]
        position: String,
    },
}


#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None, ignore_errors = false)]
pub struct Args {
    /// Test subcommand
    #[command(subcommand)]
    pub mode: Modes,
}


// 
// pub fn test_args() {
//     let args = Args::parse();
//     
//     println!("{:?}", args);
// }