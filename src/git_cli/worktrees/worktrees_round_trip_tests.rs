use std::path::PathBuf;

use super::resolve_worktree_root_path;
use crate::git_cli::fixture_repository::{create_fixture_repository, run_git_in_fixture};
use crate::repository::WorktreeRootPath;

fn path_outside_fixture(test_name: &str, purpose: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "kalapa-{test_name}-{purpose}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    path
}

// macOS の一時ディレクトリは /var の下にあり、git は /private/var で始まるパスを返すので、正規化して比べる。
fn canonical_worktree_root_path(path: &PathBuf) -> WorktreeRootPath {
    WorktreeRootPath(path.canonicalize().unwrap())
}

#[test]
fn a_subdirectory_resolves_to_its_worktree_root() {
    let fixture = create_fixture_repository("resolve-subdirectory");
    let subdirectory = fixture.join("sub");
    std::fs::create_dir(&subdirectory).unwrap();

    assert_eq!(
        resolve_worktree_root_path(&subdirectory).unwrap(),
        canonical_worktree_root_path(&fixture)
    );

    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn a_bare_repository_resolves_to_itself() {
    let bare_repository = path_outside_fixture("resolve", "bare");
    run_git_in_fixture(
        &std::env::temp_dir(),
        &["init", "-q", "--bare", bare_repository.to_str().unwrap()],
    );

    assert_eq!(
        resolve_worktree_root_path(&bare_repository).unwrap(),
        canonical_worktree_root_path(&bare_repository)
    );

    std::fs::remove_dir_all(&bare_repository).unwrap();
}
