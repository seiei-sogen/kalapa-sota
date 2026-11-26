#!/usr/bin/env bash
# Xvfb 上でアプリを起動し、履歴グラフ、ブランチ切り替え、変更一覧のステージ切り替え、
# ウィンドウに戻ったときの読み直し、メッセージを打ってのコミット、コミットの失敗の表示、push を確かめる。
# 使い方: scripts/verify_on_xvfb.sh <作業ディレクトリ>
# 必要なもの: git, Xvfb, xdotool, ImageMagick (import), Vulkan ドライバ (lavapipe で可)
set -euo pipefail

work_dir=$(realpath -m "${1:?作業ディレクトリを指定する}")
fixture="$work_dir/fixture-repo"
bare_remote="$work_dir/remote.git"
export DISPLAY=:99
# 確認用の git status が index.lock を取り、同時に走るアプリの git add や git reset を失敗させないようにする。
export GIT_OPTIONAL_LOCKS=0
app_root=$(cd "$(dirname "$0")/.." && pwd)

rm -rf "$fixture" "$bare_remote" && mkdir -p "$fixture"
commit_at() { GIT_AUTHOR_DATE="2026-11-0$1T10:00:00" GIT_COMMITTER_DATE="2026-11-0$1T10:00:00" \
  git -C "$fixture" -c user.name="$2" commit -q --allow-empty -m "$3"; }
git -C "$fixture" init -q -b main
git -C "$fixture" config user.email dev@example.com
git -C "$fixture" config user.name Alice
git -C "$fixture" config commit.gpgsign false
commit_at 1 Alice "Initial commit"
echo "# Fixture" > "$fixture/README.md" && echo "## 0.1" > "$fixture/CHANGELOG.md"
git -C "$fixture" add README.md CHANGELOG.md
commit_at 2 Alice "Add README"
git -C "$fixture" switch -q -c hotfix
commit_at 3 Carol "Fix crash on startup"
git -C "$fixture" switch -q main
git -C "$fixture" switch -q -c feature-a
commit_at 3 Bob "Add login form"
commit_at 4 Bob "Validate login input"
git -C "$fixture" switch -q main
commit_at 4 Alice "Update dependencies"
GIT_COMMITTER_DATE="2026-11-05T10:00:00" git -C "$fixture" \
  merge -q --no-ff feature-a -m "Merge branch 'feature-a'"
git -C "$fixture" switch -q -c feature-b
commit_at 6 Dave "Sketch settings page"
commit_at 7 Dave "Persist settings"
git -C "$fixture" switch -q main
commit_at 8 Alice "Release 0.1"
git init -q --bare "$bare_remote"
git -C "$fixture" remote add origin "$bare_remote"
git -C "$fixture" push -q -u origin main feature-b
echo "Usage" >> "$fixture/README.md"
echo "- staged entry" >> "$fixture/CHANGELOG.md"
git -C "$fixture" add CHANGELOG.md
echo "- unstaged entry" >> "$fixture/CHANGELOG.md"
echo "todo" > "$fixture/memo.txt"

cargo build --quiet --manifest-path "$app_root/Cargo.toml"
wait_until() { for _ in $(seq 150); do eval "$1" && return 0; sleep 0.2; done; echo "timed out: $1" >&2; return 1; }
xdotool getmouselocation >/dev/null 2>&1 || { Xvfb :99 -screen 0 1280x800x24 >/dev/null 2>&1 & }
wait_until 'xdotool getmouselocation >/dev/null 2>&1'

"$app_root/target/debug/kalapa" "$fixture" &
app_pid=$!
trap 'kill $app_pid 2>/dev/null || true' EXIT
wait_until 'window=$(xdotool search --onlyvisible --name "^kalapa - " | head -1) && [ -n "$window" ]'
# gpui 0.2.2 はウィンドウマネージャがないと最初の MapNotify を取りこぼして描画を始めないので、map し直す。
xdotool windowunmap --sync "$window" windowmap --sync "$window"
wait_until '[ "$(import -window "$window" -format "%[fx:maxima]" info:)" != 0 ]'
sleep 1
import -window root "$work_dir/history-before-checkout.png"

eval "$(xdotool getwindowgeometry --shell "$window")"
# WORKTREES の見出しとワークツリー 1 行、BRANCHES の見出しの次から、26px ごとに feature-a, feature-b, hotfix, main と並ぶ。
xdotool mousemove $((X + 60)) $((Y + 26 * 3 + 26 + 13)) click 1
wait_until '[ "$(git -C "$fixture" branch --show-current)" = feature-b ]'
sleep 1
import -window root "$work_dir/history-after-checkout.png"

current_branch=$(git -C "$fixture" branch --show-current)
echo "current branch after click: $current_branch"
[ "$current_branch" = feature-b ]

file_status() { git -C "$fixture" status --porcelain=v1 -- "$1"; }
[ "$(git -C "$fixture" status --porcelain=v1)" = "$(printf 'MM CHANGELOG.md\n M README.md\n?? memo.txt')" ]
commit_panel_height=240
row_height=26
sidebar_width=220
changed_file_row_y() { echo $((HEIGHT - commit_panel_height + row_height * ($1 + 1) + row_height / 2)); }
xdotool mousemove $((X + 320)) $((Y + $(changed_file_row_y 1))) click 1
wait_until '[ "$(file_status README.md)" = "M  README.md" ]'
xdotool mousemove $((X + 320)) $((Y + $(changed_file_row_y 0))) click 1
wait_until '[ "$(file_status CHANGELOG.md)" = "M  CHANGELOG.md" ]'
# 描き直す前に押すと、行は前の状態 (MM) のまま git add をもう一度打つので、描き直しを待つ。
sleep 0.5
xdotool mousemove $((X + 320)) $((Y + $(changed_file_row_y 0))) click 1
wait_until '[ "$(file_status CHANGELOG.md)" = " M CHANGELOG.md" ]'
sleep 1
import -window root "$work_dir/changes-after-staging-clicks.png"
echo "status after clicking README.md once and CHANGELOG.md twice:"
git -C "$fixture" status --porcelain=v1

count_colors_in_fourth_changed_file_row() {
  import -window "$window" -crop "$((WIDTH - sidebar_width))x${row_height}+${sidebar_width}+$(($(changed_file_row_y 3) - row_height / 2))" -format "%k" info:
}
echo "idea" > "$fixture/new-idea.txt"
sleep 1
color_count_before_refocus=$(count_colors_in_fourth_changed_file_row)
echo "colors in the 4th row before refocus: $color_count_before_refocus"
[ "$color_count_before_refocus" = 1 ]
root_window=$(xdotool search --maxdepth 0 --name "")
xdotool windowfocus "$root_window"
sleep 0.5
xdotool windowfocus --sync "$window"
wait_until '[ "$(count_colors_in_fourth_changed_file_row)" -gt 1 ]'
sleep 1
import -window root "$work_dir/changes-after-refocus.png"
echo "colors in the 4th row after refocus: $(count_colors_in_fourth_changed_file_row)"

panel_padding_x=12
bottom_row_padding_y=8
input_height=30
button_width=96
button_gap=8
bottom_row_y=$((Y + HEIGHT - bottom_row_padding_y - input_height / 2))
push_button_x=$((X + WIDTH - panel_padding_x - button_width / 2))
commit_button_x=$((push_button_x - 2 * (button_width + button_gap)))
input_text_signature() {
  import -window "$window" -crop "200x$((input_height - 2))+$((sidebar_width + panel_padding_x + 2))+$((HEIGHT - bottom_row_padding_y - input_height + 1))" -format "%#" info:
}
push_button_signature() {
  import -window "$window" -crop "${button_width}x${input_height}+$((WIDTH - panel_padding_x - button_width))+$((HEIGHT - bottom_row_padding_y - input_height))" -format "%#" info:
}
up_to_date_push_button_signature=$(push_button_signature)
xdotool mousemove $((X + 320)) $bottom_row_y click 1
sleep 0.5
empty_input_signature=$(input_text_signature)
typed_message="docs: README を更新"
# xdotool はロケールが UTF-8 でないと日本語の引数を読めない。また、キーマップにない文字は空きキーに一時的に割り当て、
# 押した直後に割り当てを戻す。gpui は前のキーの描画中だと戻した後のキーマップを読んで文字を落とすので、間をあける。
LC_ALL=C.UTF-8 xdotool type --delay 300 "$typed_message"
sleep 1
import -window root "$work_dir/commit-message-typed.png"
[ "$(input_text_signature)" != "$empty_input_signature" ] || { echo "typed text is not shown" >&2; exit 1; }
xdotool mousemove $commit_button_x $bottom_row_y click 1
wait_until '[ "$(git -C "$fixture" log -1 --format=%s)" = "$typed_message" ]'
sleep 1
import -window root "$work_dir/after-commit.png"
echo "HEAD subject after Commit: $(git -C "$fixture" log -1 --format=%s)"
echo "files in HEAD: $(git -C "$fixture" show --name-only --format= HEAD | tr '\n' ' ')"
[ "$(git -C "$fixture" show --name-only --format= HEAD)" = README.md ]
[ "$(input_text_signature)" = "$empty_input_signature" ] || { echo "input is not empty after commit" >&2; exit 1; }

git_error_banner_pixel() { import -window "$window" -crop "1x1+$((sidebar_width + 4))+4" -depth 8 -format "%[hex:p{0,0}]" info:; }
head_before_failing_commit=$(git -C "$fixture" rev-parse HEAD)
xdotool mousemove $((X + 320)) $bottom_row_y click 1
xdotool type --delay 80 "nothing staged"
xdotool mousemove $commit_button_x $bottom_row_y click 1
wait_until '[ "$(git_error_banner_pixel)" = 5A1D1D ]'
sleep 1
import -window root "$work_dir/commit-with-nothing-staged.png"
[ "$(git -C "$fixture" rev-parse HEAD)" = "$head_before_failing_commit" ]
[ "$(input_text_signature)" != "$empty_input_signature" ] || { echo "a failed commit cleared the message" >&2; exit 1; }
echo "banner shown after Commit with nothing staged, message kept"
xdotool windowfocus "$root_window"
sleep 0.5
xdotool windowfocus --sync "$window"
sleep 1
[ "$(git_error_banner_pixel)" = 5A1D1D ] || { echo "returning to the window cleared the banner" >&2; exit 1; }
import -window root "$work_dir/banner-after-refocus.png"
echo "banner still shown after refocus"

[ "$(push_button_signature)" != "$up_to_date_push_button_signature" ] || { echo "push label did not count the new commit" >&2; exit 1; }
install_pre_push_hook_that_counts_calls_and_waits_two_seconds() {
  printf '#!/bin/sh\necho called >> "%s"\nsleep 2\n' "$work_dir/pre-push-calls" > "$fixture/.git/hooks/pre-push"
  chmod +x "$fixture/.git/hooks/pre-push"
  rm -f "$work_dir/pre-push-calls"
}
install_pre_push_hook_that_counts_calls_and_waits_two_seconds
xdotool mousemove $push_button_x $bottom_row_y click 1
sleep 0.5
import -window root "$work_dir/pushing.png"
xdotool click 1
wait_until '[ "$(git -C "$bare_remote" rev-parse feature-b)" = "$(git -C "$fixture" rev-parse HEAD)" ]'
xdotool mousemove $((X + 320)) $((Y + 200))
wait_until '[ "$(push_button_signature)" = "$up_to_date_push_button_signature" ]'
sleep 1
import -window root "$work_dir/after-push.png"
echo "bare remote feature-b: $(git -C "$bare_remote" rev-parse --short feature-b), local HEAD: $(git -C "$fixture" rev-parse --short HEAD)"
echo "pre-push hook calls: $(wc -l < "$work_dir/pre-push-calls")"
[ "$(wc -l < "$work_dir/pre-push-calls")" = 1 ] || { echo "a click during the push started a second push" >&2; exit 1; }
[ "$(git_error_banner_pixel)" != 5A1D1D ] || { echo "the banner stayed after a successful push" >&2; exit 1; }
echo "push label back to Push ↑0, banner cleared"
