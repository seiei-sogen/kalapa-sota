use gpui::Context;

use crate::git_cli::push_current_branch;
use crate::repository_window::RepositoryWindow;

impl RepositoryWindow {
    pub fn push_current_branch_in_background(&mut self, cx: &mut Context<Self>) {
        if self.push_in_progress {
            return;
        }
        self.push_in_progress = true;
        cx.notify();
        let repository_path = self.repository_path.clone();
        cx.spawn(async move |repository_window, cx| {
            let push_result = cx
                .background_executor()
                .spawn(async move { push_current_branch(&repository_path) })
                .await;
            repository_window.update(cx, |repository_window, cx| {
                repository_window.push_in_progress = false;
                repository_window.run_git_operation_and_reload(|_| push_result);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}
