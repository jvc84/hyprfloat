use std::env;
use std::process::exit;
use hyprfloat::{config_data as conf, client_data as cli, count_data as count};
use hyprland::dispatch::{Dispatch,DispatchType};
use hyprland::dispatch::Position::{Delta};
use hyprland::Result;
use hyprland::shared::CommandContent;
use std::process::Command;

fn check_invert(window_pos : i16, window_size: i16, screen_const: i16, margin_min: i16,
                margin_max: i16, resize: i16, floating_status: bool) -> i16 {
    if floating_status == true && conf().detect_borders == true &&
        conf().invert_keys_in_stick_mode == true && conf().resize_through_borders == false &&
        conf().stick_to_borders == true &&
        ((window_pos + window_size >= screen_const - margin_max) && window_pos > margin_min ) {
            -resize

    } else {

            resize
    }
}

fn stick_window(window_pos : i16, window_size: i16, margin_min: i16, i_margin_max:i16, resize: i16, screen_const: i16) -> i16 {

    let border_pos_min = window_pos;
    let border_pos_max = window_pos + window_size;

    let border_pos_min_final = border_pos_min - resize;
    let border_pos_max_final = border_pos_max + resize;

    let margin_max = screen_const - i_margin_max;

    if  (-resize > window_size) ||
        (conf().detect_borders == true && conf().resize_through_borders == false &&
            ( border_pos_min_final < margin_min && border_pos_max_final > margin_max &&
            border_pos_max - border_pos_min < margin_max - margin_min )) {
            println!("{:?}", resize);
        // println!("{:?}", resize);
            exit(0x0100);

    } else if conf().resize_through_borders == false && conf().detect_borders == true {

        if (conf().stick_to_borders == true && border_pos_min <= margin_min) ||
        (border_pos_min_final <= margin_min) {
            margin_min - border_pos_min

        } else if (conf().stick_to_borders == true && border_pos_max >= margin_max) ||
        (border_pos_max_final >= margin_max) {
            margin_max - resize - border_pos_max
        } else {
            if conf().classic_resize == false {
                resize / -2
            } else {
                0
            }
        }
    } else {
        if conf().classic_resize == false {
            resize / -2
        } else {
            0
        }
    }
}


fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut int_args: Vec<i16>  = vec![];

    for i in 1..=2 {
        let mut arg: i16  = args[i].parse::<i16>().unwrap();
        if arg % 2 != 0 {
            arg = arg - 1;
        }

        int_args.push(arg);
    }
    let mut resize_x: i16 = int_args[0];
    let mut resize_y: i16 = int_args[1];


    // let args: Vec<String> = vec!["me".to_string(), "-80".to_string(), "0".to_string()];
    // let mut resize_x: i16 = args[1].as_str().parse::<i16>().unwrap();
    // let mut resize_y: i16 = args[2].as_str().parse::<i16>().unwrap();


    // println!("{:?}", cli().floating);

    let loc_cli = cli();

    if loc_cli.floating == true {

        let mut move_x: i16 = 0;
        let mut move_y: i16 = 0;

        resize_x = check_invert(
            loc_cli.window_pos.x,
            loc_cli.window_size.x,
            loc_cli.screen_size.x,
            conf().margins.left,
            conf().margins.right,
            resize_x,
            loc_cli.floating,
        );

        resize_y = check_invert(
            loc_cli.window_pos.y,
            loc_cli.window_size.y,
            loc_cli.screen_size.y,
            conf().margins.top,
            conf().margins.bottom,
            resize_y,
            loc_cli.floating
        );



        if resize_x != 0 {
            move_x = stick_window(
                loc_cli.window_pos.x,
                loc_cli.window_size.x,
                conf().margins.left,
                conf().margins.right,
                resize_x,
                loc_cli.screen_size.x
            )
        }

        if resize_y != 0 {
            move_y = stick_window(
                loc_cli.window_pos.y,
                loc_cli.window_size.y,
                conf().margins.top,
                conf().margins.bottom,
                resize_y,
                loc_cli.screen_size.y
            )
        }


        // println!("{:?}, {:?}", resize_x, resize_y);
        let _ = Dispatch::call(
            DispatchType::MoveActive(
                Delta(move_x, move_y)
            )
        );

        // Command::new("hyprctl")
        //     .arg("dispatch")
        //     .arg("moveactive")
        //     .arg(move_x)
        //     .arg(move_y)
        //     .spawn();


        let _ = Dispatch::call(
            DispatchType::ResizeActive(
                Delta(resize_x, resize_y))
        );

        // Command::new("hyprctl")
        //     .arg("dispatch")
        //     .arg("resizeactive")
        //     .arg(resize_x)
        //     .arg(resize_y)
        //     .spawn();



    } else {
        // println!("{:?}, {:?}", resize_x, resize_y);
        let _ = Dispatch::call(
            DispatchType::ResizeActive(
                Delta(resize_x, resize_y))
        );

        // Command::new("hyprctl")
        //     .arg("dispatch")
        //     .arg("resizeactive")
        //     .arg(resize_x)
        //     .arg(resize_y)
        //     .spawn();
    }


    Ok(())
}





