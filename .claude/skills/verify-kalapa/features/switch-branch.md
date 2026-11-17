# Switch branch

Clicking a branch in the BRANCHES panel runs `git switch` on it. The panel, history, and CHANGES re-read afterwards. When git refuses, its reason appears in the red banner and the current branch stays.

## Sub-features

- `switch-branch-click` switches to the clicked branch.
- `switch-branch-refused` shows git's reason when local changes would be overwritten.

## How to get to it (user POV)

- Click a branch name in the BRANCHES panel on the left.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md). On a fresh fixture `CHANGELOG.md` is the same on `main` and `feature-b`, so the local changes carry over.

- **Switch.** Click `feature-b`. Run `click_branch_row 1` and `wait_until '[ "$(git_in_repository branch --show-current)" = feature-b ]'`. The condition holds.
- **Proof.** Run `sleep 1; capture_screenshot switch-branch-feature-b; record_git_state switch-branch-feature-b`. The screenshot shows ● on `feature-b`, and CHANGES still lists the 4 rows.
- **Refused switch.** Start from a fresh fixture. Stage and commit `CHANGELOG.md` on `main` through the app (see [commit-staged-changes.md](./commit-staged-changes.md)), so `main` and `feature-b` now differ in a file that still has unstaged edits. Then run `click_branch_row 1`, `sleep 2`, and `capture_screenshot switch-branch-refused`. The red banner starts with `error: Your local changes to the following files would be overwritten by checkout:`, and `git_in_repository branch --show-current` still prints `main`.

## Gotchas

- A refused switch is git working correctly. Do not report it as an app failure. Read the banner before deciding.
- The panel lists local branches only. Remote branches are not shown.
- The banner stays until the next git operation succeeds. Returning to the window does not clear it.
