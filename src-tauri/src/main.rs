extern crate anyhow;
mod audio;
use audio::{get_default_output_device, get_output_devices, JsonError};

use tauri::api::process::Command;
use tauri::api::shell;
use tauri::{
    ActivationPolicy, App, AppHandle, CustomMenuItem, Manager, Menu, PhysicalPosition, Submenu,
    SystemTray, SystemTrayEvent,
};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

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
async fn list_output_devices() -> Result<serde_json::Value, JsonError> {
    get_output_devices()
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
    //             -- set visible of application process "System Settings" to false

    let script = format!(
        r#"
        set myDevice to "{}"

        tell application "System Settings"
            quit
            delay 0.3
            activate
            delay 0.3
            
            tell application "System Events"
                keystroke "Sound"
                delay 0.5
                
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

            delay 1
            activate
            quit
        end tell
        "#,
        name
    );

    run_applescript(&script)
}

fn app_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").expect("Main window not found");
    // let _ = win.set_always_on_top(true);

    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(ActivationPolicy::Accessory);
        apply_vibrancy(
            &main_window,
            NSVisualEffectMaterial::HudWindow,
            Some(NSVisualEffectState::Active),
            Some(8.0),
        )
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    }

    #[cfg(debug_assertions)]
    {
        main_window.open_devtools();
    }

    Ok(())
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();

    if let SystemTrayEvent::LeftClick { position, .. } = event {
        let win_outer_size = window.outer_size().unwrap();

        if window.is_visible().unwrap() {
            window.hide().unwrap();
            window.emit("window:hidden", false).unwrap();
        } else {
            window.show().unwrap();
            window.set_focus().unwrap();
        }

        window
            .set_position(PhysicalPosition {
                x: position.x,
                y: position.y,
            })
            .unwrap();

        let current_monitor = window.current_monitor().unwrap().unwrap();
        let screen_size = current_monitor.size();
        let screen_position = current_monitor.position();

        let y = if position.y > screen_size.height as f64 / 2.0 {
            position.y - win_outer_size.height as f64
        } else {
            position.y as f64
        };

        window
            .set_position(PhysicalPosition {
                x: f64::min(
                    position.x - win_outer_size.width as f64 / 2.0,
                    (screen_position.x as f64 + screen_size.width as f64)
                        - win_outer_size.width as f64,
                ),
                y,
            })
            .unwrap()
    }
}
fn main() {
    get_default_output_device().unwrap();

    let ctx = tauri::generate_context!();
    let tray = SystemTray::new();

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
        .system_tray(tray)
        .on_system_tray_event(handle_tray_event)
        .setup(app_setup)
        .run(ctx)
        .expect("error while running tauri application");
}
