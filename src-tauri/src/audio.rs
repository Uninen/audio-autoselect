extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::DeviceNameError;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonError {
    message: String,
}

impl From<anyhow::Error> for JsonError {
    fn from(error: anyhow::Error) -> Self {
        JsonError {
            message: format!("{:?}", error),
        }
    }
}

impl From<DeviceNameError> for JsonError {
    fn from(error: DeviceNameError) -> Self {
        JsonError {
            message: format!("{:?}", error),
        }
    }
}

pub fn get_default_output_device() -> Result<serde_json::Value, JsonError> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| JsonError {
        message: String::from("No output device available"),
    })?;
    let device_name = device.name().map_err(|e| JsonError::from(e))?;
    println!("Default output device: {}", device_name);
    Ok(json!({ "name": device_name }))
}

// pub fn enumerate_audio_devices() -> Result<(), anyhow::Error> {
//     println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
//     let available_hosts = cpal::available_hosts();
//     println!("Available hosts:\n  {:?}", available_hosts);

//     for host_id in available_hosts {
//         println!("{}", host_id.name());
//         let host = cpal::host_from_id(host_id)?;

//         let default_in = host.default_input_device().map(|e| e.name().unwrap());
//         let default_out = host.default_output_device().map(|e| e.name().unwrap());
//         println!("  Default Input Device:\n    {:?}", default_in);
//         println!("  Default Output Device:\n    {:?}", default_out);

//         let devices = host.devices()?;
//         println!("  Devices: ");
//         for (device_index, device) in devices.enumerate() {
//             println!("  {}. \"{}\"", device_index + 1, device.name()?);

//             // Input configs
//             if let Ok(conf) = device.default_input_config() {
//                 println!("    Default input stream config:\n      {:?}", conf);
//             }
//             let input_configs = match device.supported_input_configs() {
//                 Ok(f) => f.collect(),
//                 Err(e) => {
//                     println!("    Error getting supported input configs: {:?}", e);
//                     Vec::new()
//                 }
//             };
//             if !input_configs.is_empty() {
//                 println!("    All supported input stream configs:");
//                 for (config_index, config) in input_configs.into_iter().enumerate() {
//                     println!(
//                         "      {}.{}. {:?}",
//                         device_index + 1,
//                         config_index + 1,
//                         config
//                     );
//                 }
//             }

//             // Output configs
//             if let Ok(conf) = device.default_output_config() {
//                 println!("    Default output stream config:\n      {:?}", conf);
//             }
//             let output_configs = match device.supported_output_configs() {
//                 Ok(f) => f.collect(),
//                 Err(e) => {
//                     println!("    Error getting supported output configs: {:?}", e);
//                     Vec::new()
//                 }
//             };
//             if !output_configs.is_empty() {
//                 println!("    All supported output stream configs:");
//                 for (config_index, config) in output_configs.into_iter().enumerate() {
//                     println!(
//                         "      {}.{}. {:?}",
//                         device_index + 1,
//                         config_index + 1,
//                         config
//                     );
//                 }
//             }
//         }
//     }

//     Ok(())
// }
