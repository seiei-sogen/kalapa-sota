# Commit staged changes

A single-line input under CHANGES takes the commit message. Japanese input works. **Commit** commits what is staged. On success the input clears. On failure git's reason appears in the red banner and the typed message stays.

## Sub-features

- `commit-staged` commits only the staged files with the typed message.
- `commit-japanese-message` keeps a Japanese message intact in the commit subject.
- `commit-nothing-staged` shows git's one-line reason and keeps the message.
- `commit-empty-message` shows git's refusal for an empty message.

## How to get to it (user POV)

- Tick files in CHANGES, click the message input, type, then click **Commit**.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md).

- **Stage one file.** Click `README.md`. Run `click_changed_file_row 1` and `wait_until '[ "$(git_in_repository status --porcelain=v1 -- README.md)" = "M  README.md" ]'`.
- **Type the message.** Run `click_commit_message_input`, `type_text "docs: 使い方を追記"`, `sleep 1`, and `capture_screenshot commit-message-typed`. The input shows the full message with the cursor after it.
- **Commit.** Run `click_commit_button` and `wait_until '[ "$(git_in_repository log -1 --format=%s)" = "docs: 使い方を追記" ]'`. Then run `git_in_repository show --name-only --format= HEAD`. It prints `CHANGELOG.md` and `README.md`, because the fixture's `CHANGELOG.md` already had a staged part.
- **Input clears.** Run `sleep 1; capture_screenshot commit-done`. The input shows the grey placeholder `Commit message`, and the new commit is the top history row.
- **Nothing staged.** Run `click_commit_message_input`, `type_text "nothing staged"`, `click_commit_button`, `sleep 2`, and `capture_screenshot commit-nothing-staged`. The banner shows one line that starts with `no changes added to commit` or `nothing added to commit`, the input still holds `nothing staged`, and `git_in_repository log -1 --format=%s` is unchanged.
- **Empty message.** Stage a file, run `click_commit_message_input` and `press_keys ctrl+a BackSpace`, then `click_commit_button`. The banner shows `Aborting commit due to empty commit message.`

## Gotchas

- Type with `type_text`, never raw `xdotool type`. Without `LC_ALL=C.UTF-8` xdotool cannot read Japanese, and without the 300 ms delay GPUI drops characters while xdotool remaps keys.
- Long typing queues up in a debug build on lavapipe. Wait on `git log`, not on a fixed sleep, after a long message.
- The input does not scroll sideways. A message wider than the box hides its end on screen but is committed in full. Prove the message with `git log`, not the screenshot.
- Real IME composition (underlined text, candidate window) cannot be driven on Xvfb. `type_text` sends finished characters only.
- `click_commit_button` assumes the Push label fits in 96 px. A label like `Push ↑12 ↓3` widens the Push button and moves Commit left. Take a screenshot and check before clicking.
