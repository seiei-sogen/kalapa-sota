use std::path::PathBuf;

use super::run_git_in_fixture;

pub fn create_fixture_repository(test_name: &str) -> PathBuf {
    let fixture = std::env::temp_dir().join(format!("kalapa-{test_name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&fixture);
    std::fs::create_dir_all(&fixture).unwrap();
    run_git_in_fixture(&fixture, &["init", "-q", "-b", "main"]);
    fixture
}
