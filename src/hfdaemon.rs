extern crate core;

use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::thread;
use crate::{config_data, CONFIG_FILE, SOCKET_FILE, monitor_data, DaemonData, CONFIG_DATA, MONITOR_DATA, BUF_SIZE, layers_data, LAYERS_DATA, GIVE_ARGS, CLIENT_DATA, empty_client, windows_config_data, WINDOWS_CONFIG_DATA};



use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::{SystemTime, Duration};
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use signal_hook::{consts::SIGTERM, iterator::Signals};
use crate::DaemonArgs;

fn get_last_modified_time(str_path: String) -> Result<SystemTime, std::io::Error> {
    let path = Path::new(&str_path);
    let metadata = fs::metadata(path)?;
    Ok(metadata.modified()?)
}


fn mod_time() -> Result<f64, Box<dyn std::error::Error>> {
    let path: String = CONFIG_FILE.read().unwrap().clone(); // Replace with your file path
    
    match get_last_modified_time(path) {
        Ok(time) => {
            let duration_since_epoch = time.duration_since(SystemTime::UNIX_EPOCH)?;
            // println!("Last modified: {} seconds since the epoch", duration_since_epoch.as_secs_f64());

            // *CURR_DUR.write().expect("Cannot write") = duration_since_epoch.as_secs_f64();
            Ok(duration_since_epoch.as_secs_f64())

        }
        Err(e) => {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
        }
    }

}

use std::sync::{Arc, RwLock};
use hyprland::data::Client;
use hyprland::shared::*;

lazy_static!(
    static ref LAST_DUR:  Arc<RwLock<f64>> = Arc::new(RwLock::new(0f64));
    static ref CURR_DUR:  Arc<RwLock<f64>> = Arc::new(RwLock::new(0f64));
    static ref MON:  Arc<RwLock<i16>> = Arc::new(RwLock::new(0));
    // static ref LOC_CONFIG_FILE: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("")));
);


fn remove_socket_file(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}


pub fn daemon(daemon_args: crate::Modes) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_args = match daemon_args {
        crate::Modes::Daemon {
            config
        } => crate::DaemonArgs {
            config
        },
        _ => panic!("Cannot parse Daemon enum into DaemonArgs")
    };  

    *CONFIG_FILE.write().unwrap() = parsed_args.config.clone();

    
    if let Ok(_) = fs::remove_file(SOCKET_FILE.clone()) {
        println!("Removed existing socket file.");
    }

    // Create a Unix domain socket
    let listener = match UnixListener::bind(SOCKET_FILE.clone()) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error creating socket: {}", e);
            return Err(Box::new(e));
        }
    };

    // Capture termination signal
    let mut signals = match Signals::new(&[SIGTERM]) {
        Ok(signals) => signals,
        Err(e) => {
            eprintln!("Error setting up signal handler: {}", e);
            return Err(Box::new(e));
        }
    };

    std::thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal: {:?}", sig);
            if let Err(e) = remove_socket_file(&SOCKET_FILE.clone()) {
                eprintln!("Error removing socket file: {}", e);
            }
            exit(0); // Exit cleanly after removing the socket.
        }
    });
    
    
    let mut handles: Vec<_> = vec![];

    *MONITOR_DATA.write().unwrap() = monitor_data();
    *CONFIG_DATA.write().unwrap() = config_data()?;

    let handle = thread::spawn(move || {
        loop {
            let _ = match layers_data() {
                Ok(x) => {println!("Layers success")},
                Err(e) => {eprintln!("Layers data: {}", e)},
            };
            
            let _ = match windows_config_data() {
                Ok(x) => {println!("Windows config success")},
                Err(e) => {eprintln!("Windows config data: {}", e)},
            };
            
            thread::sleep(Duration::from_millis(1500));
        }

    });
    handles.push(handle);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let new_stream = thread::spawn(move || {
                    let last_duration = LAST_DUR.read().expect("Cannot read last dur").clone();
                    let current_duration = mod_time().expect("Cannot read time");

                    let last_monitor = MON.read().expect("Cannot read last monitor").clone();
                    let current_monitor = hyprland::data::Monitor::get_active().unwrap().id as i16;
                                        
            
                    println!("Last dur: {}, current dur: {}", last_duration, current_duration);
                    
                    if  current_duration != last_duration || last_monitor != current_monitor {
                        println!("Rewrite");
                        *LAST_DUR.write().expect("Cannot write last dur") = current_duration;
                        *MON.write().expect("Cannot write last monitor") = current_monitor;
                        *CONFIG_DATA.write().expect("Cannot write config") = config_data().expect("Cannot find config");
                    } else {
                        println!("Not Rewrite");
                    }

                    let send_config = CONFIG_DATA.read().expect("Cannot find config").clone();
                    *MONITOR_DATA.write().expect("Cannot write monitor") = monitor_data().clone();
                                        
                        let send_data = DaemonData {
                            daemon_config: send_config.clone(),
                            daemon_windows_config: WINDOWS_CONFIG_DATA.read().expect("Cannot read windows config").clone(),
                            daemon_monitor: MONITOR_DATA.read().expect("Cannot read monitor").clone(),
                            daemon_layers: LAYERS_DATA.read().expect("Cannot read monitor").clone(),
                        };

                        let send_string = serde_json::to_string(&send_data).unwrap();
                        let send_bytes  = send_string.as_bytes();
                        let mut add_buf = [1u8 ; BUF_SIZE];
                        add_buf[0..send_bytes.len()].copy_from_slice(&send_bytes);

                        println!("Len:  {}", add_buf.len());


                        if let Err(e) = stream.write_all(send_bytes) {
                            eprintln!("Error writing to socket: {}", e);
                        }
                });
                handles.push(new_stream);

            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }


    for h in handles {
        h.join().unwrap();
    }


    Ok(())
}
