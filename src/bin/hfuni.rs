use std::process::exit;
use clap::Parser;
use clap::Command;
use hyprfloat::{notify_error, DaemonArgs, GIVE_ARGS};
use hyprfloat:: {
    daemon,
    exec,
    togglefloating,
    resizeactive,
    movewindow
};

use hyprfloat::Modes::{self, Daemon, Exec, Togglefloating, Resizeactive, Movewindow};
use hyprfloat::args::Args;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    /////////////
    use clap::CommandFactory;
    use clap_complete::generate_to;
    use clap_complete::Shell::{Bash, Zsh, Fish};
    use std::path::PathBuf;
    use std::fs;

    let mut cmd = crate::Args::command();
    let comp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/completions");

    fs::create_dir_all(&comp_dir).unwrap();

    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, "hfuni", &comp_dir).unwrap();
    }

    //////////////
    
 
     
    
    let parsed_args = Args::parse();
    
    match parsed_args.mode {
        Daemon{ .. }  => daemon(parsed_args.mode),
        Exec { .. } => Ok(exec(parsed_args.mode)?),
        Togglefloating { .. } => Ok(togglefloating(parsed_args.mode)),
        Resizeactive { .. }  => Ok(resizeactive(parsed_args.mode)),
        Movewindow { .. } => Ok(movewindow(parsed_args.mode)),
        _  => {
            notify_error(format!("No such mode: {:?}", parsed_args.mode));
            exit(1);
        }

    }
}