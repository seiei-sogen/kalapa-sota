use super::run_git_command::run_git_command;

#[test]
fn git_runs_without_a_terminal_so_ssh_prompts_fail_instead_of_waiting() {
    let terminal_reachable = run_git_command(
        &std::env::temp_dir(),
        &[
            "-c",
            "alias.can-open-terminal=!(: </dev/tty) 2>/dev/null && echo yes || echo no",
            "can-open-terminal",
        ],
    )
    .unwrap();
    assert_eq!(terminal_reachable.trim(), "no");
}
