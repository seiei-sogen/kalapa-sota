# Pull the current branch

The button between **Commit** and **Push** pulls the current branch's upstream into the opened worktree in the background, fast-forward only. It reads `Pull`, and a dimmed `Pulling…` while running. While a pull or a push runs, both **Pull** and **Push** are dimmed and ignore clicks, and the rest of the window still works. When git cannot fast-forward, for example because the branch has diverged, has no upstream, or is not checked out, the red banner shows git's reason and HEAD does not move.

## Sub-features

- `pull-fast-forward` brings in the upstream's new commits. The newest is the top history row.
- `pull-in-progress` shows a dimmed `Pulling…` next to a dimmed **Push**. A click on **Push** starts nothing, and CHANGES still stages.
- `pull-diverged` shows `fatal: Not possible to fast-forward, aborting.` and leaves HEAD where it was.
- `pull-linked-worktree` moves only the opened linked worktree's branch.

## How to get to it (user POV)

- Click the second button from the right at the bottom of the window.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md). `main` tracks `origin/main` in sync. Run the first three steps in order in one launch. Each starts from the state the previous one left.
- Move the remote from a second clone. Run `SECOND_CLONE="$RUN_DIR/second-clone"`, `rm -rf "$SECOND_CLONE"`, and `git clone -q -b main "$RUN_DIR/remote.git" "$SECOND_CLONE"`. Commit there with `git -C "$SECOND_CLONE" -c user.name=Bob -c user.email=bob@example.com commit -q --allow-empty -m <subject>`, then `git -C "$SECOND_CLONE" push -q`.

- **Fast-forward.** Commit `Remote work` in the second clone and push it. Run `click_pull_button` and `wait_until '[ "$(git_in_repository rev-parse HEAD)" = "$(git -C "$RUN_DIR/remote.git" rev-parse main)" ]'`. The condition holds. Run `sleep 1; capture_screenshot pull-fast-forward-after`. The top history row is `Remote work` by Bob, and the button reads `Pull` again.
- **In progress.** Commit `Slow remote work` in the second clone and push it. Make the fetch take 3 s with `git_in_repository config remote.origin.uploadpack 'sleep 3; git-upload-pack'`. Count pushes with `printf '#!/bin/sh\necho called >> "%s"\n' "$EVIDENCE_DIR/pre-push-calls" > "$REPOSITORY/.git/hooks/pre-push"` and `chmod +x "$REPOSITORY/.git/hooks/pre-push"`. Then run `click_pull_button`, `sleep 0.5`, `capture_screenshot pull-in-progress`, `click_push_button`, `click_changed_file_row 1`, and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- README.md)" = "M  README.md" ]'`. The screenshot shows a dimmed `Pulling…` and a dimmed `Push ↑0`. README.md is staged while `git_in_repository rev-parse HEAD` still differs from `git -C "$RUN_DIR/remote.git" rev-parse main`. Wait until they are equal, then `sleep 2`. `cat "$EVIDENCE_DIR/pre-push-calls" 2>/dev/null | wc -l` prints `0`. Undo with `git_in_repository config --unset remote.origin.uploadpack` and `rm "$REPOSITORY/.git/hooks/pre-push"`.
- **Diverged.** Commit `Remote only` in the second clone and push it. Commit locally with `git_in_repository commit -q --allow-empty -m "Local only"`, then run `HEAD_BEFORE_PULL=$(git_in_repository rev-parse HEAD)`, `leave_and_return_to_window`, `sleep 1`, `click_pull_button`, `wait_until '[ "$(git_in_repository rev-parse origin/main)" = "$(git -C "$RUN_DIR/remote.git" rev-parse main)" ]'`, `sleep 2`, and `capture_screenshot pull-diverged`. The banner reads `fatal: Not possible to fast-forward, aborting.`, `git_in_repository rev-parse HEAD` still prints `$HEAD_BEFORE_PULL`, and `ls "$REPOSITORY/.git"` has no `rebase-merge`. **Push** reads `Push ↑1 ↓1`.
- **Linked worktree.** Start from a fresh fixture launched on the review worktree: `"$SCRIPTS/launch_kalapa.sh" "$RUN_DIR" "$DISPLAY_NUMBER" "$RUN_DIR/fixture-repo-worktree-review"`. `git_in_repository` then reads that worktree. Publish it with `click_push_button` and `wait_until 'git -C "$RUN_DIR/remote.git" rev-parse --verify -q worktree-review >/dev/null'`. Make the second clone with `-b worktree-review` instead of `-b main`, commit `Review feedback` there, and push it. Run `MAIN_HEAD_BEFORE=$(git -C "$RUN_DIR/fixture-repo" rev-parse HEAD)`, `click_pull_button`, and `wait_until '[ "$(git_in_repository rev-parse HEAD)" = "$(git -C "$RUN_DIR/remote.git" rev-parse worktree-review)" ]'`. The condition holds, and `git -C "$RUN_DIR/fixture-repo" rev-parse HEAD` still prints `$MAIN_HEAD_BEFORE`. Run `sleep 1; capture_screenshot pull-linked-worktree-after`. The top history row is `Review feedback`.

## Gotchas

- Make the second clone with `-b main` (or `-b worktree-review`). Here `git init --bare` leaves the remote's HEAD at `master`, so a plain clone checks out nothing, and its commit lands on a new `master`.
- Keep remote commits empty. The fixture has uncommitted changes in `README.md`, `CHANGELOG.md`, and `notes.txt`, and git refuses a fast-forward that touches them.
- `remote.origin.uploadpack` and `.git/hooks/pre-push` live in the shared `.git`, so they also slow and count the linked worktrees' pulls and pushes. Undo them after the in-progress step.
- kalapa does not fetch on return to the window. The pull's own fetch is what updates `↓M` on **Push**.
- `click_pull_button` assumes the Push label fits in 96 px. `Push ↑1 ↓1` already widens it and moves **Pull** and **Commit** about 12 px left, which still lands. Check a screenshot before clicking after longer labels.
- `rm -rf "$RUN_DIR/second-clone"` after the recipe. `cleanup_kalapa.sh` removes only the fixture, its linked worktrees, and the bare remote.
