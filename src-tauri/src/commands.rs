use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Position, Rect as TauriRect, Size, State,
    WebviewBuilder, WebviewUrl, Window,
};

use crate::geometry::ViewportRect;
use crate::instances::{InstanceId, InstanceManager, InstanceRecord};
use crate::profile::profile_dir;
use crate::recipes::{find_recipe, Recipe};
use crate::workspace::{save_workspace, PersistedInstance};

pub(crate) fn spawn_instance_webview(
    app: &AppHandle,
    window: &Window,
    id: InstanceId,
    recipe: &Recipe,
    native_user_agent: bool,
) -> Result<String, String> {
    let webview_label = format!("instance-{}", id);
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let data_dir = profile_dir(&app_data_dir, id.0);

    let mut builder = WebviewBuilder::new(
        webview_label.clone(),
        WebviewUrl::External(recipe.url.parse().expect("recipe url must be a valid URL")),
    )
    .data_directory(data_dir);

    if !native_user_agent {
        builder = builder.user_agent(recipe.default_user_agent);
    }

    let webview = window
        .add_child(
            builder,
            LogicalPosition::new(0.0, 0.0),
            LogicalSize::new(1.0, 1.0),
        )
        .map_err(|e| e.to_string())?;
    webview.hide().map_err(|e| e.to_string())?;

    Ok(webview_label)
}

fn persist_workspace(app: &AppHandle, state: &State<InstanceManager>) -> Result<(), String> {
    let instances = state.0.lock().map_err(|e| e.to_string())?;
    let persisted: Vec<PersistedInstance> = instances
        .iter()
        .map(|(id, record)| PersistedInstance {
            id: *id,
            recipe_id: record.recipe_id.clone(),
            label: record.label.clone(),
            native_user_agent: record.native_user_agent,
        })
        .collect();
    drop(instances);
    let app_config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    save_workspace(&app_config_dir, &persisted).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_service_instance(
    app: AppHandle,
    state: State<InstanceManager>,
    recipe_id: String,
    label: String,
) -> Result<InstanceId, String> {
    let recipe = find_recipe(&recipe_id).ok_or_else(|| format!("unknown recipe: {recipe_id}"))?;
    let window = app.get_window("main").ok_or("main window not found")?;
    let id = InstanceId::new();

    let webview_label = spawn_instance_webview(&app, &window, id, recipe, false)?;

    let mut instances = state.0.lock().map_err(|e| e.to_string())?;
    instances.insert(
        id,
        InstanceRecord {
            webview_label,
            recipe_id,
            label,
            native_user_agent: false,
        },
    );
    drop(instances);
    persist_workspace(&app, &state)?;

    Ok(id)
}

#[tauri::command]
pub fn focus_service_instance(
    app: AppHandle,
    state: State<InstanceManager>,
    id: InstanceId,
) -> Result<(), String> {
    let instances = state.0.lock().map_err(|e| e.to_string())?;
    for (other_id, record) in instances.iter() {
        if *other_id != id {
            if let Some(webview) = app.get_webview(&record.webview_label) {
                webview.hide().map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn close_service_instance(
    app: AppHandle,
    state: State<InstanceManager>,
    id: InstanceId,
) -> Result<(), String> {
    {
        let mut instances = state.0.lock().map_err(|e| e.to_string())?;
        let record = instances
            .remove(&id)
            .ok_or_else(|| "instance not found".to_string())?;
        if let Some(webview) = app.get_webview(&record.webview_label) {
            webview.close().map_err(|e| e.to_string())?;
        }
    }
    persist_workspace(&app, &state)?;
    Ok(())
}

#[tauri::command]
pub fn resize_service_instance(
    app: AppHandle,
    state: State<InstanceManager>,
    id: InstanceId,
    bounds: ViewportRect,
) -> Result<(), String> {
    let instances = state.0.lock().map_err(|e| e.to_string())?;
    let record = instances
        .get(&id)
        .ok_or_else(|| "instance not found".to_string())?;
    let webview = app
        .get_webview(&record.webview_label)
        .ok_or_else(|| "webview not found".to_string())?;
    let window = app
        .get_window("main")
        .ok_or_else(|| "main window not found".to_string())?;

    let scale_factor = window.scale_factor().map_err(|e| e.to_string())?;
    let physical_size = window.inner_size().map_err(|e| e.to_string())?;
    let logical_size = physical_size.to_logical::<f64>(scale_factor);
    let window_bounds = ViewportRect {
        x: 0.0,
        y: 0.0,
        width: logical_size.width,
        height: logical_size.height,
    };
    let clamped = bounds.clamped_to(window_bounds);

    webview
        .set_bounds(TauriRect {
            position: Position::Logical(LogicalPosition::new(clamped.x, clamped.y)),
            size: Size::Logical(LogicalSize::new(clamped.width, clamped.height)),
        })
        .map_err(|e| e.to_string())?;
    webview.show().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn debug_read_instance_marker(
    app: AppHandle,
    state: State<InstanceManager>,
    id: InstanceId,
) -> Result<Option<String>, String> {
    let instances = state.0.lock().map_err(|e| e.to_string())?;
    let record = instances
        .get(&id)
        .ok_or_else(|| "instance not found".to_string())?;
    let webview = app
        .get_webview(&record.webview_label)
        .ok_or_else(|| "webview not found".to_string())?;
    let url = webview.url().map_err(|e| e.to_string())?;
    let cookies = webview.cookies_for_url(url).map_err(|e| e.to_string())?;
    Ok(cookies
        .iter()
        .find(|c| c.name() == "instance_marker")
        .map(|c| c.value().to_string()))
}

#[tauri::command]
pub fn set_instance_user_agent(
    app: AppHandle,
    state: State<InstanceManager>,
    id: InstanceId,
    native: bool,
) -> Result<(), String> {
    let window = app.get_window("main").ok_or("main window not found")?;
    let (recipe_id, label, old_webview_label) = {
        let instances = state.0.lock().map_err(|e| e.to_string())?;
        let record = instances
            .get(&id)
            .ok_or_else(|| "instance not found".to_string())?;
        (
            record.recipe_id.clone(),
            record.label.clone(),
            record.webview_label.clone(),
        )
    };
    let recipe = find_recipe(&recipe_id).ok_or_else(|| format!("unknown recipe: {recipe_id}"))?;

    if let Some(webview) = app.get_webview(&old_webview_label) {
        webview.close().map_err(|e| e.to_string())?;
    }

    let webview_label = match spawn_instance_webview(&app, &window, id, recipe, native) {
        Ok(webview_label) => webview_label,
        Err(err) => {
            // The old webview is already closed and gone; without a fresh webview the
            // instance record would point at nothing, so drop it rather than leave a
            // ghost entry that a restart can't recover.
            let mut instances = state.0.lock().map_err(|e| e.to_string())?;
            instances.remove(&id);
            drop(instances);
            persist_workspace(&app, &state)?;
            return Err(err);
        }
    };

    let mut instances = state.0.lock().map_err(|e| e.to_string())?;
    // Single-window app: commands are dispatched sequentially from one UI's IPC calls, so
    // the window between the read-lock above and this re-insert is narrow and currently
    // accepted (a concurrent close_service_instance could theoretically race here).
    instances.insert(
        id,
        InstanceRecord {
            webview_label,
            recipe_id,
            label,
            native_user_agent: native,
        },
    );
    drop(instances);
    persist_workspace(&app, &state)?;
    Ok(())
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSummary {
    pub id: InstanceId,
    pub recipe_id: String,
    pub label: String,
    pub native_user_agent: bool,
}

#[tauri::command]
pub fn list_service_instances(
    state: State<InstanceManager>,
) -> Result<Vec<InstanceSummary>, String> {
    let instances = state.0.lock().map_err(|e| e.to_string())?;
    Ok(instances
        .iter()
        .map(|(id, record)| InstanceSummary {
            id: *id,
            recipe_id: record.recipe_id.clone(),
            label: record.label.clone(),
            native_user_agent: record.native_user_agent,
        })
        .collect())
}
