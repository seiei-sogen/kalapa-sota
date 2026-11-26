use std::path::PathBuf;

pub fn path_outside_fixture(test_name: &str, purpose: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "kalapa-{test_name}-{purpose}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    path
}
