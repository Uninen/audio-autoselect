#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate anyhow;

mod audio;
use audio::{get_default_output_device, JsonError};

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

#[tauri::command]
async fn get_default_output_name() -> Result<serde_json::Value, JsonError> {
    get_default_output_device()
}

fn run_applescript(script: &str) -> Result<String, String> {
    let output = Command::new("osascript").args(&["-e", script]).output();

    match output {
        Ok(output) => Ok(output.stdout),

        Err(_) => Err("Applescript execution failed".to_string()),
    }
}

#[tauri::command]
async fn set_system_audio_output(name: &str) -> Result<String, String> {
    let script = format!(
        r#"
        set myDevice to "{}"

        tell application "System Settings"
            quit
            delay 0.2
            activate
            -- set visible of application process "System Settings" to false
            delay 0.2
            
            tell application "System Events"
                
                delay 0.2
                keystroke "Sound"
                delay 1
                
                set theRows to (every row of table 1 of scroll area 1 of group 2 of scroll area 1 of group 1 of group 2 of splitter group 1 of group 1 of window "Sound" of application process "System Settings")
                
                repeat with aRow in theRows
                    try
                        if name of first item of static text of group 1 of UI element 1 of aRow is equal to myDevice then
                            set selected of aRow to true
                            exit repeat
                        end if
                    on error
                        display dialog "Error setting output sound to " & device
                    end try
                end repeat
            end tell
            delay 2
            quit
        end tell
        "#,
        name
    );

    run_applescript(&script)
}

fn main() {
    get_default_output_device().unwrap();

    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_output_devices,
            list_input_devices,
            set_system_audio_output,
            get_default_output_name
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
