mod commands;
mod geometry;
mod instances;
mod profile;
mod recipes;
mod workspace;

use tauri::{AppHandle, Manager, Window};

use instances::{InstanceManager, InstanceRecord};

fn restore_workspace(app_handle: &AppHandle, window: &Window) {
    let app_config_dir = match app_handle.path().app_config_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!(
                "featherium: could not resolve app config dir, starting with empty workspace: {err}"
            );
            return;
        }
    };

    let persisted = workspace::load_workspace(&app_config_dir);
    let mut restored = Vec::new();
    let mut to_insert = Vec::new();

    for entry in persisted {
        let Some(recipe) = recipes::find_recipe(&entry.recipe_id) else {
            eprintln!(
                "featherium: dropping persisted instance {} — unknown recipe '{}'",
                entry.id, entry.recipe_id
            );
            continue;
        };
        match commands::spawn_instance_webview(
            app_handle,
            window,
            entry.id,
            recipe,
            entry.native_user_agent,
        ) {
            Ok(webview_label) => {
                to_insert.push((
                    entry.id,
                    InstanceRecord {
                        webview_label,
                        recipe_id: entry.recipe_id.clone(),
                        label: entry.label.clone(),
                        native_user_agent: entry.native_user_agent,
                    },
                ));
                restored.push(entry);
            }
            Err(err) => {
                eprintln!(
                    "featherium: dropping persisted instance {} ('{}') — failed to restore webview: {err}",
                    entry.id, entry.recipe_id
                );
            }
        }
    }

    {
        let state = app_handle.state::<InstanceManager>();
        let mut instances = state.0.lock().unwrap();
        for (id, record) in to_insert {
            instances.insert(id, record);
        }
    }

    if let Err(err) = workspace::save_workspace(&app_config_dir, &restored) {
        eprintln!("featherium: failed to persist restored workspace: {err}");
    }
}

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
            let app_handle = app.handle().clone();
            let window = app.get_window("main").ok_or("main window not found")?;
            restore_workspace(&app_handle, &window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
