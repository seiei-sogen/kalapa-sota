use std::path::Path;

use crate::git_cli::run_git_command::run_git_command;

pub fn run_git_in_fixture(fixture: &Path, arguments: &[&str]) {
    let identity = [
        "-c",
        "user.name=Fixture Author",
        "-c",
        "user.email=fixture@example.com",
        "-c",
        "commit.gpgsign=false",
    ];
    run_git_command(fixture, &[&identity[..], arguments].concat()).expect("fixture git command");
}
