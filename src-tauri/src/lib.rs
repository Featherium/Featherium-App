mod commands;
mod geometry;
mod instances;
mod profile;
mod recipes;
mod workspace;

use instances::{InstanceManager, InstanceRecord};

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
            commands::set_instance_user_agent,
            commands::list_service_instances,
        ])
        .setup(|app| {
            use tauri::Manager;
            let app_handle = app.handle().clone();
            let window = app.get_window("main").expect("main window must exist");
            let app_config_dir = app_handle
                .path()
                .app_config_dir()
                .expect("app config dir must resolve");

            let persisted = workspace::load_workspace(&app_config_dir);
            let mut restored = Vec::new();
            {
                let state = app_handle.state::<InstanceManager>();
                let mut instances = state.0.lock().unwrap();
                for entry in persisted {
                    let Some(recipe) = recipes::find_recipe(&entry.recipe_id) else {
                        continue;
                    };
                    if let Ok(webview_label) = commands::spawn_instance_webview(
                        &app_handle,
                        &window,
                        entry.id,
                        recipe,
                        entry.native_user_agent,
                    ) {
                        instances.insert(
                            entry.id,
                            InstanceRecord {
                                webview_label,
                                recipe_id: entry.recipe_id.clone(),
                                label: entry.label.clone(),
                                native_user_agent: entry.native_user_agent,
                            },
                        );
                        restored.push(entry);
                    }
                }
            }
            let _ = workspace::save_workspace(&app_config_dir, &restored);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
