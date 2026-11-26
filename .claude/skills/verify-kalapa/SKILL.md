---
name: verify-kalapa
description: Drive the real kalapa desktop window (the Rust + GPUI git GUI in this repo) on Xvfb with xdotool clicks and key presses, and prove behavior with screenshots plus git state. Use when verifying any change to what a kalapa user sees or does, such as the history graph, branch switching, staging changed files, committing, pulling, or pushing.
---

# Verify kalapa

kalapa is a GPUI desktop app with no accessibility tree. Drive it by window-relative coordinates. `kalapa_driver.sh` computes them from `src/appearance/repository_window_sizes.rs`, so a layout change in that file moves the clicks with it. Prove every action with git state, not only pixels.

Run every command from the repository root. The feature recipes are in [features/README.md](features/README.md). Read it before driving a feature.

## Launch

Needs Xvfb, xdotool, ImageMagick `import`, a Vulkan driver (`mesa-vulkan-drivers`, lavapipe is enough), and on Linux the build packages `libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev libfreetype-dev libfontconfig-dev`.

Pick a fresh run directory (inside your scratchpad if you have one) and a free display number from 90 to 98. `scripts/verify_on_xvfb.sh` owns `:99`.

```sh
RUN_DIR=/tmp/kalapa-verify-$(date +%s)
DISPLAY_NUMBER=91
SCRIPTS=.claude/skills/verify-kalapa/scripts
"$SCRIPTS/create_fixture_repository.sh" "$RUN_DIR"
"$SCRIPTS/launch_kalapa.sh" "$RUN_DIR" "$DISPLAY_NUMBER" "$RUN_DIR/fixture-repo"
```

The app is ready when the last line starts with `ready:`. The first `cargo build` takes about six minutes. `launch_kalapa.sh` does the following.

- It refuses a display that is already in use. Pick another number. Never drive an X server this run did not start.
- It starts its own Xvfb and records the PIDs in `$RUN_DIR/state.env`.
- It unmaps and maps the window once, because GPUI 0.2.2 never starts drawing without a window manager otherwise.
- It waits until the window has non-black pixels.

Every click writes to the repository the window has open. Open only the fixture or a throwaway copy, never a repository someone cares about.

## Doctor

```sh
"$SCRIPTS/doctor_kalapa.sh" "$RUN_DIR"
```

It runs six read-only checks. They confirm that the Xvfb and the app are the ones this run started, that the window title matches the repository, that the binary is newer than every source file, that stderr has no panic, and that the window is drawn. Exit code 0 means the instance is worth driving. Run it first whenever anything looks off. The title check compares against the worktree `launch_kalapa.sh` opened, so it fails after `click_worktree_row` opens another worktree.

## Drive

```sh
. "$SCRIPTS/kalapa_driver.sh" "$RUN_DIR"
```

This defines the following functions in the current shell.

| Function | Action |
| --- | --- |
| `click_worktree_row <i>` | Click the i-th worktree in WORKTREES (0-based, `git worktree list` order, main worktree first). The window opens that worktree. |
| `click_branch_row <i>` | Click the i-th branch in BRANCHES (0-based, sorted by name). |
| `click_changed_file_row <i>` | Click the i-th row in CHANGES (0-based, `git status --porcelain` order, untracked last). |
| `click_commit_message_input` | Click the message input. This focuses it. |
| `type_text <text>` | Type into the focused input. Japanese works. |
| `press_keys <keys...>` | Press xdotool keys, for example `ctrl+a BackSpace`. |
| `click_commit_button` | Click **Commit**. |
| `click_pull_button` | Click **Pull** (also labeled `Pulling…`). |
| `click_push_button` | Click **Push** (also labeled `Publish` or `Pushing…`). |
| `leave_and_return_to_window` | Focus away and back. kalapa re-reads the repository on return. |
| `wait_until '<shell condition>'` | Poll up to 30 s. |
| `capture_screenshot <name>` | Save `$EVIDENCE_DIR/<name>.png`. |
| `record_git_state <name>` | Save status, the last 3 commits, and `branch -vv` to `$EVIDENCE_DIR/<name>.git.txt`. |
| `git_in_repository <args>` | Run git in the worktree `launch_kalapa.sh` opened, without taking `index.lock`. |

`git_in_repository` and `record_git_state` run git in `REPOSITORY` from `state.env`, the worktree `launch_kalapa.sh` opened. They do not follow `click_worktree_row`. Read another worktree with `git -C <path>`.

After each click, wait on git with `wait_until`, not a fixed sleep. Sleep about 1 s before a screenshot so the repaint lands.

## Evidence

Everything goes to `$RUN_DIR/evidence/`. The app's stdout and stderr are there as `app-stdout.log` and `app-stderr.log`.

- Drive the real user path. Click and type. Do not run the git command yourself to set up the state you are trying to prove.
- Capture the state before the action and after it, with `capture_screenshot` and `record_git_state` for each.
- Prove side effects in git: the index, `git log`, the bare remote at `$RUN_DIR/remote.git`.
- Report the feature ID and the evidence paths for each claim.

## Cleanup

```sh
"$SCRIPTS/cleanup_kalapa.sh" "$RUN_DIR"
```

It stops only the kalapa and Xvfb PIDs recorded in `state.env`. It deletes the fixture, its linked worktrees, and the bare remote, and keeps `$RUN_DIR/evidence/`. Run it after every failed attempt too, so no process or display is left behind.
