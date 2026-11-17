# Stage changed files

The CHANGES list under the history shows one row per changed path. The checkbox is the git index itself. Clicking a row stages the whole file, or unstages it when it is already fully staged. Returning to the window re-reads the repository, so edits made elsewhere appear.

## Sub-features

- `stage-file` stages an unstaged, untracked, or deleted file.
- `stage-partial` stages the rest of a partly staged file (⊟ becomes ☑).
- `unstage-file` unstages a fully staged file (☑ becomes ☐).
- `stage-reread-on-return` shows files changed outside kalapa after the user comes back to the window.

## How to get to it (user POV)

- Click a row in CHANGES at the bottom of the right column.
- Edit a file in another program, then return to the kalapa window.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md).

- **Stage a modified file.** Click `README.md`. Run `click_changed_file_row 1` and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- README.md)" = "M  README.md" ]'`. The condition holds and the row shows ☑.
- **Unstage it.** Click the same row again. Run `sleep 0.5; click_changed_file_row 1` and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- README.md)" = " M README.md" ]'`. The row shows ☐.
- **Stage the rest of a partial file.** Click `CHANGELOG.md`. Run `click_changed_file_row 0` and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- CHANGELOG.md)" = "M  CHANGELOG.md" ]'`. ⊟ becomes ☑.
- **Stage a deletion.** Click `notes.txt`. Run `click_changed_file_row 2` and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- notes.txt)" = "D  notes.txt" ]'`.
- **Stage an untracked Japanese path.** Click `日本語メモ.txt`. Run `click_changed_file_row 3` and `wait_until 'git_in_repository status --porcelain=v1 | grep -q "^A  "'`.
- **Re-read on return.** Create a file outside kalapa and come back. Run `echo idea > "$REPOSITORY/new-idea.txt"`, `leave_and_return_to_window`, `sleep 1`, and `capture_screenshot stage-reread-on-return`. The header reads `CHANGES (5)` and `new-idea.txt` is the last row.
- **Proof.** Run `capture_screenshot stage-final` and `record_git_state stage-final` after the steps you claim.

## Gotchas

- Wait about 0.5 s between two clicks on the same row. A click before the repaint reuses the row's previous state and stages again instead of unstaging.
- Clicking ⊟ stages the whole file. Clicking ☑ then unstages all of it. The earlier split between staged and unstaged parts is lost.
- Clicking a conflicted row (`!`) runs `git add`, which marks the file resolved even with conflict markers in it. The fresh fixture has no conflict. Build one with a conflicting merge first if you need this path.
- Untracked files sort after tracked ones. `git status --porcelain` prints non-ASCII paths quoted, while the app shows them as UTF-8.
- Paths are relative to the repository root even when kalapa was opened at a subdirectory.
