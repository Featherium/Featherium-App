fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(&[
            "open_service_instance",
            "focus_service_instance",
            "close_service_instance",
            "resize_service_instance",
            "debug_read_instance_marker",
            "set_instance_user_agent",
            "list_service_instances",
        ])),
    )
    .expect("failed to run tauri-build");
}
