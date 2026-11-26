# kalapa verification map

This directory is the maintained source for verifying what a kalapa user sees and does. Read this index first, then use the matching feature file as the recipe.

## Baseline preconditions

- `create_fixture_repository.sh` built the fixture, and `launch_kalapa.sh` opened `$RUN_DIR/fixture-repo` on a display this run started.
- `doctor_kalapa.sh "$RUN_DIR"` exits 0.
- `kalapa_driver.sh` is sourced in the current shell.

The fresh fixture looks like this.

- Worktrees, in panel order: `fixture-repo` (0), `fixture-repo-detached` (1), `fixture-repo-worktree-review` (2). `fixture-repo` is the one `launch_kalapa.sh` opens, so its row has ●. `fixture-repo-detached` checks out the head of `main` detached. `fixture-repo-worktree-review` checks out `worktree-review` and has one untracked file, `sub/only-in-worktree-review.txt`.
- Branches, in panel order: `feature-a` (0), `feature-b` (1), `hotfix` (2), `main` (3), `worktree-review` (4). `main` is current and tracks `origin/main` in sync. `feature-b` and `worktree-review` have no upstream. `worktree-review` points at the head of `main`.
- The bare remote `$RUN_DIR/remote.git` has only `main`.
- History has 7 commits. Dates and authors are fixed, so the screen is the same every run.
- CHANGES rows, in order, are as follows.

| Row | Path | Status | Checkbox |
| --- | --- | --- | --- |
| 0 | `CHANGELOG.md` | `MM` | ⊟ |
| 1 | `README.md` | ` M` | ☐ |
| 2 | `notes.txt` | ` D` | ☐ |
| 3 | `日本語メモ.txt` | `??` | ☐ |

## Driving conventions

- Start every recipe from a fresh fixture unless its preconditions say otherwise. A fresh fixture is `cleanup_kalapa.sh`, then Launch again.
- Use the driver functions. Do not compute coordinates by hand.
- After a click, wait on git state with `wait_until`. A screenshot alone is not proof.
- Treat every command as literal.

## Proof and skip reporting

- Capture the user action and the resulting state, not only the final screen.
- UI proof is a screenshot from `capture_screenshot`. State proof is `record_git_state` or the exact git command and its output.
- Record the feature ID and entry point with every artifact.
- Report an unreachable path with the attempted command and the unmet precondition.
- Do not report a skipped entry point as verified through a different path.

## Feature entry contract

Each feature file starts with an H1 title and one paragraph describing the user-visible behavior. It then uses exactly four H2 sections in this order.

1. `Sub-features` lists short IDs with one line for each behavior.
2. `How to get to it (user POV)` lists every user entry point.
3. `Driving it with kalapa_driver.sh` starts with `Preconditions:` and uses labeled bullets that pair each user action with an exact command and observable result.
4. `Gotchas` lists traps that can waste or invalidate a verification run.

## Features

- [View worktrees](./view-worktrees.md) covers the WORKTREES list, the ● on the opened worktree, and opening a linked worktree from a subdirectory.
- [View history](./view-history.md) covers the commit list, the branch graph, and the branch colors.
- [Switch branch](./switch-branch.md) covers switching from the branch panel and git refusing a switch.
- [Stage changed files](./stage-changed-files.md) covers the CHANGES list, the three checkbox states, and re-reading on return to the window.
- [Commit staged changes](./commit-staged-changes.md) covers the message input, Japanese text, Commit, and the error banner.
- [Push the current branch](./push-current-branch.md) covers Publish, `Push ↑N ↓M`, `Pushing…`, and push failures.
