//! git コマンドを呼び出す境界。git の出力はここでドメインの型に変換し、外へは型だけを渡す。

mod changed_files;
mod git_command_error;
mod read_commit_history;
mod read_local_branches;
mod read_repository_snapshot;
mod run_git_command;
mod switch_local_branch;

#[cfg(test)]
mod fixture_repository;
#[cfg(test)]
mod git_cli_round_trip_tests;

pub use changed_files::toggle_changed_file_in_commit;
pub use git_command_error::GitCommandError;
pub use read_repository_snapshot::read_repository_snapshot;
pub use switch_local_branch::switch_local_branch;
