use gpui::Context;

use crate::git_cli::{pull_current_branch_fast_forward_only, push_current_branch};
use crate::repository_window::{RemoteOperation, RepositoryWindow};

impl RepositoryWindow {
    pub fn run_remote_operation_in_background(
        &mut self,
        remote_operation: RemoteOperation,
        cx: &mut Context<Self>,
    ) {
        if self.remote_operation_in_progress.is_some() {
            return;
        }
        self.remote_operation_in_progress = Some(remote_operation);
        cx.notify();
        let opened_worktree_root_path = self.opened_worktree_root_path.clone();
        let run_remote_git_command = match remote_operation {
            RemoteOperation::Push => push_current_branch,
            RemoteOperation::Pull => pull_current_branch_fast_forward_only,
        };
        cx.spawn(async move |repository_window, cx| {
            let remote_operation_result = cx
                .background_executor()
                .spawn(async move { run_remote_git_command(&opened_worktree_root_path.0) })
                .await;
            repository_window.update(cx, |repository_window, cx| {
                repository_window.remote_operation_in_progress = None;
                repository_window.run_git_operation_and_reload(|_| remote_operation_result);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}
