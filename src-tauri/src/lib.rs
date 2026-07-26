mod commands;
mod geometry;
mod instances;
mod profile;

use instances::InstanceManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(InstanceManager::default())
        .invoke_handler(tauri::generate_handler![
            commands::open_service_instance,
            commands::focus_service_instance,
            commands::close_service_instance,
            commands::resize_service_instance,
            commands::debug_read_instance_marker,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
