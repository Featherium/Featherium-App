use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn profile_dir(app_data_dir: &Path, instance_id: Uuid) -> PathBuf {
    app_data_dir.join("profiles").join(instance_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_path_under_profiles_subdirectory() {
        let base = Path::new("app-data");
        let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let result = profile_dir(base, id);
        assert_eq!(
            result,
            Path::new("app-data/profiles/11111111-1111-1111-1111-111111111111")
        );
    }

    #[test]
    fn different_ids_produce_different_paths() {
        let base = Path::new("app-data");
        let id_a = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let id_b = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
        assert_ne!(profile_dir(base, id_a), profile_dir(base, id_b));
    }
}
