# View worktrees

The WORKTREES section at the top of the left column lists every worktree of the repository, one row each, in `git worktree list` order. A row shows a mark, the directory name, and what the worktree checks out: a branch name, `detached` and the first 7 characters of the commit ID, or `bare`. The worktree the window has open has ●, a highlighted row, and bold text. BRANCHES sits below it.

## Sub-features

- `worktrees-list` shows one row per worktree with its directory name and checkout.
- `worktrees-opened-mark` marks the opened worktree with ● and a highlighted row.
- `worktrees-open-from-subdirectory` resolves a path inside a worktree to that worktree's root, for the ● and the window title.

## How to get to it (user POV)

- Open a repository. The section shows at once.
- Launch kalapa with a path inside a linked worktree, such as a subdirectory. That worktree becomes the opened one.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md).

- **Read the list.** Run `sleep 1; capture_screenshot worktrees-initial`. WORKTREES shows 3 rows: `fixture-repo` with `main` and ●, `fixture-repo-detached` with `detached` and a commit ID, and `fixture-repo-worktree-review` with `worktree-review`. `git_in_repository worktree list` prints the same 3 worktrees in the same order.
- **Branches moved down.** Run `click_branch_row 0` and `wait_until '[ "$(git_in_repository branch --show-current)" = feature-a ]'`. The condition holds.
- **Open a linked worktree from a subdirectory.** Run `"$SCRIPTS/cleanup_kalapa.sh" "$RUN_DIR"`, `"$SCRIPTS/create_fixture_repository.sh" "$RUN_DIR"`, and `"$SCRIPTS/launch_kalapa.sh" "$RUN_DIR" "$DISPLAY_NUMBER" "$RUN_DIR/fixture-repo-worktree-review/sub"`, then source `kalapa_driver.sh` again. `"$SCRIPTS/doctor_kalapa.sh" "$RUN_DIR"` exits 0, and its title line names `$RUN_DIR/fixture-repo-worktree-review`, not `sub`. Run `sleep 1; capture_screenshot worktrees-opened-from-subdirectory`. ● sits on the `fixture-repo-worktree-review` row and on `worktree-review` in BRANCHES. CHANGES lists only `sub/only-in-worktree-review.txt`.

## Gotchas

- Clicking a worktree row does nothing yet.
- A name wider than the column is clipped without an ellipsis. Tell the fixture rows apart by the checkout label.
- `REPOSITORY` in `state.env` is the resolved worktree root, not the path passed to `launch_kalapa.sh`.
- `click_branch_row` counts worktrees with `git worktree list` at click time. After adding or removing a worktree, return to the window first, or the click lands one row off.
- Opening the `.git` directory of a non-bare repository marks no worktree row, because its path matches none of them.
