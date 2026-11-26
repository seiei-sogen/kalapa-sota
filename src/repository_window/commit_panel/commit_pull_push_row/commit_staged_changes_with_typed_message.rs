use gpui::Context;

use crate::git_cli::commit_staged_changes;
use crate::repository_window::RepositoryWindow;

impl RepositoryWindow {
    pub fn commit_staged_changes_with_typed_message(&mut self, cx: &mut Context<Self>) {
        let commit_message = self.commit_message_input.read(cx).text().to_string();
        let committed = self.run_git_operation_and_reload(|worktree_root_path| {
            commit_staged_changes(worktree_root_path, &commit_message)
        });
        if committed {
            self.commit_message_input
                .update(cx, |commit_message_input, cx| {
                    commit_message_input.clear(cx)
                });
        }
        cx.notify();
    }
}
