use crate::repository::WorktreeRootPath;

// 先頭の "kalapa - " は、検証スクリプトがウィンドウを探す目印なので変えない。
pub fn repository_window_title(opened_worktree_root_path: &WorktreeRootPath) -> String {
    format!("kalapa - {}", opened_worktree_root_path.0.display())
}
