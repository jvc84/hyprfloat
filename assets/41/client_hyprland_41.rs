use hyprland::shared::Address;
use crate::FromClient;
use hyprland::shared::*;
use std::boxed::Box;
use hyprland::data::WorkspaceBasic;
use hyprland::data::{CursorPosition, Client, Monitor};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Descartes {
    pub x: i16,
    pub y: i16,
}


pub fn client_data() -> FromClient {
    let active_window = Client::get_active()
        .unwrap()
        .unwrap_or(
            Client {
                address: Address::new(
                    "0x1a1a1a1a1a1a".to_string(),
                ),
                at: (500, 500),
                size: (10,10),
                workspace: WorkspaceBasic {
                    id: 4,
                    name: "Empty".to_string(),
                },
                floating: false,
                fullscreen: false,
                fullscreen_mode: 0,
                // fullscreen: hyprland::data::FullscreenMode::None,
                // fullscreen_client: hyprland::data::FullscreenMode::None,
                monitor: 0,
                initial_class: "Empty".to_string(),
                class: "Empty".to_string(),
                initial_title: "Empty".to_string(),
                title: "Empty".to_string(),
                pid: 28823,
                xwayland: true,
                pinned: false,
                grouped: vec![],
                mapped: true,
                focus_history_id: 1,
                swallowing: Some(
                    Box::<Address>::new(Address::new("0x0"))
                )

            }
        );

    let client = FromClient {
        window_pos: Descartes {
            x: active_window.at.0,
            y: active_window.at.1,
        },
        window_size: Descartes {
            x: active_window.size.0,
            y: active_window.size.1,
        },
        screen_size: Descartes {
            x: Monitor::get_active().unwrap().width  as i16,
            y: Monitor::get_active().unwrap().height as i16,
        },
        cursor_pos: Descartes{
            x: CursorPosition::get().unwrap().x as i16,
            y: CursorPosition::get().unwrap().y as i16,
        },
        address: active_window.address,
        floating: active_window.floating,
        fullscreen: active_window.fullscreen
        // fullscreen: parse_fullscreen(active_window.fullscreen),
    };


    return client

}