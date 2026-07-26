use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Position, Rect as TauriRect, Size, State,
    WebviewBuilder, WebviewUrl,
};

use crate::geometry::ViewportRect;
use crate::instances::{InstanceId, InstanceManager, InstanceRecord};
use crate::profile::profile_dir;

#[tauri::command]
pub fn open_service_instance(
    app: AppHandle,
    state: State<InstanceManager>,
    recipe_id: String,
    label: String,
) -> Result<InstanceId, String> {
    let window = app.get_window("main").ok_or("main window not found")?;
    let id = InstanceId::new();
    let webview_label = format!("instance-{}", id);

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let data_dir = profile_dir(&app_data_dir, id.0);

    let builder = WebviewBuilder::new(
        webview_label.clone(),
        WebviewUrl::App("isolation-test.html".into()),
    )
    .data_directory(data_dir);

    let webview = window
        .add_child(builder, LogicalPosition::new(0.0, 0.0), LogicalSize::new(1.0, 1.0))
        .map_err(|e| e.to_string())?;
    webview.hide().map_err(|e| e.to_string())?;

    let mut instances = state.0.lock().map_err(|e| e.to_string())?;
    instances.insert(id, InstanceRecord { webview_label, recipe_id, label });

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
    let mut instances = state.0.lock().map_err(|e| e.to_string())?;
    let record = instances
        .remove(&id)
        .ok_or_else(|| "instance not found".to_string())?;
    if let Some(webview) = app.get_webview(&record.webview_label) {
        webview.close().map_err(|e| e.to_string())?;
    }
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
