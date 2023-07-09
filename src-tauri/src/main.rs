#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #[cfg(debug_assertions)]
// #[cfg(target_os = "macos")]
// embed_plist::embed_info_plist!("../Info.plist");

use tauri::api::process::Command;
use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

#[tauri::command]
async fn list_devices(input_type: &str) -> Result<serde_json::Value, String> {
    let output = Command::new("SwitchAudioSource")
        .args(&["-a", "-t", input_type, "-f", "json"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let lines: Vec<String> = output.stdout.split('\n').map(|s| s.to_string()).collect();

                let mut json_vec: Vec<serde_json::Value> = Vec::new();
                for line in lines {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let json: serde_json::Value = serde_json::from_str(&trimmed)
                        .map_err(|err| format!("Failed to parse JSON: {}", err))?;
                    json_vec.push(json);
                }

                Ok(serde_json::json!(json_vec))
            } else {
                Err(output.stderr)
            }
        }
        Err(_) => Err("Command not found".to_string()),
    }
}

#[tauri::command]
async fn list_output_devices() -> Result<serde_json::Value, String> {
    list_devices("output").await
}

#[tauri::command]
async fn list_input_devices() -> Result<serde_json::Value, String> {
    list_devices("input").await
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_output_devices,
            list_input_devices
        ])
        .menu(
            tauri::Menu::os_default("Tauri Vue Template").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new(
                    "Online Documentation",
                    "Online Documentation",
                )
                .into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/Uninen/tauri-vue-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
