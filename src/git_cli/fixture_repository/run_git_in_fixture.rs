use std::path::Path;

use crate::git_cli::run_git_command::run_git_command;

pub fn run_git_in_fixture(fixture: &Path, arguments: &[&str]) {
    run_git_command(fixture, arguments).expect("fixture git command");
}
