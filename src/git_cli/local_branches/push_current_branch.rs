use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;

/// 現在のブランチだけを上流ブランチへ送る。上流ブランチがなければ、同じ名前でリモートに公開し、追跡を設定する。
// shortcut: push.autoSetupRemote は git 2.37 から。古い git では Publish が失敗する。古い git に対応するなら、上流がないときにリモートと HEAD を明示する。
pub fn push_current_branch(repository_path: &Path) -> Result<(), GitCommandError> {
    run_git_command(
        repository_path,
        &[
            // 利用者の push.default が matching だとほかのブランチまで送り、nothing だと何も送らないので、ここで決める。
            "-c",
            "push.default=upstream",
            "-c",
            "push.autoSetupRemote=true",
            "push",
            "--quiet",
        ],
    )
    .map(|_| ())
}
