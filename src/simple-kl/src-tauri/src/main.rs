// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod extensions;
mod pages;
mod structs;
mod themes;
mod utils;

use enigo::MouseControllable;
use extensions::*;
use pages::dialogs::*;
use pages::search::*;
use pages::settings::*;
use std::env;
use tauri::{
    Manager, PhysicalPosition, RunEvent, WindowBuilder, WindowEvent, WindowUrl,
};
use utils::*;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_results,
            get_current_settings,
            update_settings,
            run_action,
            get_extensions_json,
            get_os,
            update_extension_setting,
            update_extension_keyword,
            add_search_engine,
            export_theme,
            import_theme,
            debug_message,
            import_extension,
            delete_extension,
            get_extension_default_keyword,
            get_extension_default_setting,
            get_community_themes,
            apply_community_theme,
            get_community_extensions,
            install_community_extension,
            get_dialog_action,
            write_dialog_result,
            open_settings,
            update_auto_start,
            get_whitelist_apps,
            get_blacklist_apps,
            add_to_blacklist,
            remove_from_blacklist
        ])
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments.iter().any(|e| e == "--settings");

            //Opens settings if the argument is found
            if open_settings {
                WindowBuilder::new(app, "settings", WindowUrl::App("settings".into()))
                    .title("Settings")
                    .build()
                    .expect("Error creating settings window");

                let main_window = app.get_window("main").unwrap();
                main_window.close().expect("Error closing search window");

                return Ok(());
            }

            let (cursor_x, cursor_y) = enigo::Enigo::new().mouse_location();

            let main_window = app.get_window("main").unwrap();

            main_window
                .set_position(PhysicalPosition {
                    x: cursor_x,
                    y: cursor_y,
                })
                .unwrap();

            main_window.set_always_on_top(true).unwrap();
            main_window.show().unwrap();
            main_window.set_focus().unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .build(tauri::generate_context!())
        .unwrap()
        .run(|app, e| match e {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::Focused(focused) => {
                            //Hides the window when user clicks outside
                            if !focused {
                                let window = app.get_window("main").unwrap();
                                window.close().unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
}
