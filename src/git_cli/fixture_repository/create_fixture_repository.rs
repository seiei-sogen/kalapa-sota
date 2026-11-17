use std::path::PathBuf;

use super::run_git_in_fixture;

pub fn create_fixture_repository(test_name: &str) -> PathBuf {
    let fixture = std::env::temp_dir().join(format!("kalapa-{test_name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&fixture);
    std::fs::create_dir_all(&fixture).unwrap();
    run_git_in_fixture(&fixture, &["init", "-q", "-b", "main"]);
    // 利用者の git 設定がない CI でも、テストが run_git_command で直接打つ git commit が通るようにする。
    for (key, value) in [
        ("user.name", "Fixture Author"),
        ("user.email", "fixture@example.com"),
        ("commit.gpgsign", "false"),
    ] {
        run_git_in_fixture(&fixture, &["config", key, value]);
    }
    fixture
}
