# Push the current branch

The button right of **Commit** pushes the current branch to its upstream in the background. Its label shows the branch's distance from the upstream: `Publish` with no upstream, `Push ↑N`, `Push ↑N ↓M` when behind, and a dimmed `Pushing…` while running. A second click during a push is ignored. Failures show git's reason in the red banner.

## Sub-features

- `push-ahead` pushes local commits and the label returns to `Push ↑0`.
- `push-publish` publishes a branch with no upstream under the same name and sets tracking.
- `push-behind-label` shows `↓M` after the remote moved ahead and was fetched.
- `push-in-progress` dims the button as `Pushing…` and ignores further clicks.
- `push-failure` shows git's reason and returns the label.

## How to get to it (user POV)

- Click the button at the bottom right of the window.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md). `main` tracks `origin/main` in sync, so the label reads `Push ↑0`.

- **Ahead.** Commit once through the app (see [commit-staged-changes.md](./commit-staged-changes.md)). Run `sleep 1; capture_screenshot push-ahead-before`. The label reads `Push ↑1`.
- **Push.** Run `click_push_button` and `wait_until '[ "$(git -C "$RUN_DIR/remote.git" rev-parse main)" = "$(git_in_repository rev-parse HEAD)" ]'`. Then run `sleep 1; capture_screenshot push-ahead-after`. The label reads `Push ↑0`.
- **Publish.** Switch to `feature-b`, which has no upstream. Run `click_branch_row 1`, wait for `git_in_repository branch --show-current` to print `feature-b`, then run `capture_screenshot push-publish-before`. The label reads `Publish`. Run `click_push_button` and `wait_until 'git -C "$RUN_DIR/remote.git" rev-parse --verify -q feature-b'`. Then `git_in_repository rev-parse --abbrev-ref feature-b@{upstream}` prints `origin/feature-b`.
- **In progress.** Make the push slow, then click twice. Run `printf '#!/bin/sh\necho called >> "%s"\nsleep 2\n' "$EVIDENCE_DIR/pre-push-calls" > "$REPOSITORY/.git/hooks/pre-push"` and `chmod +x "$REPOSITORY/.git/hooks/pre-push"`, commit once through the app, then run `click_push_button`, `sleep 0.5`, `capture_screenshot push-in-progress`, and `click_push_button`. The screenshot shows a dimmed `Pushing…`. After the push ends, `wc -l < "$EVIDENCE_DIR/pre-push-calls"` prints `1`.
- **Behind.** Move the remote ahead from a second clone, then fetch. Run `git clone -q "$RUN_DIR/remote.git" "$RUN_DIR/second-clone"`, commit and push there with `git -C "$RUN_DIR/second-clone" -c user.name=Bob -c user.email=bob@example.com commit -q --allow-empty -m remote-work` and `git -C "$RUN_DIR/second-clone" push -q`, then `git_in_repository fetch -q`, `leave_and_return_to_window`, `sleep 1`, and `capture_screenshot push-behind`. The label shows `↓1`.
- **Failure.** Point origin at a missing path and push. Run `git_in_repository remote set-url origin "$RUN_DIR/missing.git"`, commit once through the app, `click_push_button`, `sleep 3`, and `capture_screenshot push-failure`. The banner shows git's `does not appear to be a git repository`, and the label is back to `Push ↑1`.

## Gotchas

- The push always uses `push.default=upstream`, whatever the repository's config says. A branch tracking a differently named upstream pushes to that upstream.
- `Publish` needs git 2.37 or newer.
- ssh and HTTPS remotes are out of reach on Xvfb here. Git runs without a terminal, so an ssh passphrase prompt fails instead of waiting. That path is covered only by `cargo test run_git_command` under `script -qec`.
- The behind count comes from the last fetch or push. Fetch first, then return to the window, or the label keeps the old count.
- Remove the pre-push hook or start from a fresh fixture after the in-progress step. Every later push waits 2 s otherwise.
- `rm -rf "$RUN_DIR/second-clone"` after the behind step. `cleanup_kalapa.sh` removes only the fixture, its linked worktrees, and the bare remote.
