# Open a worktree

Clicking a row in WORKTREES opens that worktree in the window. ● moves to the clicked row, and the history, BRANCHES, CHANGES, and window title become that worktree's. Branch switching, staging, committing, and pushing then act on the opened worktree. When git cannot read the clicked worktree, for example because its directory was deleted, the red banner shows git's reason and the window keeps the worktree it had open.

## Sub-features

- `open-worktree-click` opens the clicked worktree. ●, history, BRANCHES, CHANGES, and the title follow it.
- `open-worktree-switch-branch` switches the branch of the opened worktree only.
- `open-worktree-branch-in-use` shows git's refusal when the clicked branch is checked out in another worktree.
- `open-worktree-unreadable` keeps the opened worktree and shows git's error when the clicked worktree cannot be read.

## How to get to it (user POV)

- Click a row in WORKTREES at the top of the left column.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md). Run the four steps in order in one launch. Each step starts from the state the previous one left.
- Run `REVIEW="$RUN_DIR/fixture-repo-worktree-review"`. `git_in_repository` keeps reading `fixture-repo`, so read the opened worktree with `git -C "$REVIEW"`.

- **Open.** Run `click_worktree_row 2` and `wait_until '[ "$(xdotool getwindowname "$WINDOW")" = "kalapa - $REVIEW" ]'`. The condition holds. Run `sleep 1; capture_screenshot open-worktree-review`. ● sits on the `fixture-repo-worktree-review` row and on `worktree-review` in BRANCHES. CHANGES lists only `sub/only-in-worktree-review.txt`.
- **Switch the opened worktree's branch.** Run `click_branch_row 2` and `wait_until '[ "$(git -C "$REVIEW" branch --show-current)" = hotfix ]'`. The condition holds, and `git_in_repository branch --show-current` still prints `main`. Run `sleep 1; capture_screenshot open-worktree-switch-to-hotfix`. ● sits on `hotfix` in BRANCHES.
- **Branch in use by another worktree.** Run `click_branch_row 3`, `sleep 2`, and `capture_screenshot open-worktree-main-in-use`. The red banner starts with `fatal: 'main' is already used by worktree at`. `git -C "$REVIEW" branch --show-current` still prints `hotfix`, and `git_in_repository branch --show-current` still prints `main`.
- **Unreadable worktree.** Run `rm -rf "$RUN_DIR/fixture-repo-detached"`, `click_worktree_row 1`, `sleep 2`, and `capture_screenshot open-worktree-deleted-directory`. The red banner starts with `fatal: cannot change to` and names `fixture-repo-detached`. ● stays on the `fixture-repo-worktree-review` row, and `xdotool getwindowname "$WINDOW"` still prints `kalapa - $REVIEW`.

## Gotchas

- `doctor_kalapa.sh` compares the title with `REPOSITORY` in `state.env`, so its title check fails once another worktree is open. The other five checks still apply.
- The banner text is git's own. The wording above is from git 2.43. Other git versions may word it differently.
- `fixture-repo-detached` stays in WORKTREES after its directory is deleted, because git still lists it. `click_branch_row` still counts 3 worktrees, so the branch rows do not shift.
