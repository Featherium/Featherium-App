use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::instances::InstanceId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedInstance {
    pub id: InstanceId,
    pub recipe_id: String,
    pub label: String,
    pub native_user_agent: bool,
}

fn workspace_path(app_config_dir: &Path) -> PathBuf {
    app_config_dir.join("workspace.json")
}

pub fn load_workspace(app_config_dir: &Path) -> Vec<PersistedInstance> {
    let path = workspace_path(app_config_dir);
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

pub fn save_workspace(app_config_dir: &Path, instances: &[PersistedInstance]) -> std::io::Result<()> {
    std::fs::create_dir_all(app_config_dir)?;
    let contents =
        serde_json::to_string_pretty(instances).expect("PersistedInstance always serializes");
    std::fs::write(workspace_path(app_config_dir), contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn temp_dir() -> PathBuf {
        std::env::temp_dir().join(format!("featherium-workspace-test-{}", Uuid::new_v4()))
    }

    #[test]
    fn missing_file_loads_as_empty() {
        let dir = temp_dir();
        assert_eq!(load_workspace(&dir), Vec::new());
    }

    #[test]
    fn round_trips_through_disk() {
        let dir = temp_dir();
        let instances = vec![PersistedInstance {
            id: InstanceId(Uuid::new_v4()),
            recipe_id: "whatsapp".into(),
            label: "WhatsApp".into(),
            native_user_agent: false,
        }];
        save_workspace(&dir, &instances).unwrap();
        assert_eq!(load_workspace(&dir), instances);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn corrupted_file_loads_as_empty() {
        let dir = temp_dir();
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("workspace.json"), "not valid json").unwrap();
        assert_eq!(load_workspace(&dir), Vec::new());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
