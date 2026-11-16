#!/usr/bin/env bash
# Xvfb 上でアプリを起動し、履歴グラフの画面とブランチ切り替えを確かめる。
# 使い方: scripts/verify_on_xvfb.sh <作業ディレクトリ>
# 必要なもの: git, Xvfb, xdotool, ImageMagick (import), Vulkan ドライバ (lavapipe で可)
set -euo pipefail

work_dir=$(realpath -m "${1:?作業ディレクトリを指定する}")
fixture="$work_dir/fixture-repo"
export DISPLAY=:99
app_root=$(cd "$(dirname "$0")/.." && pwd)

rm -rf "$fixture" && mkdir -p "$fixture"
commit_at() { GIT_AUTHOR_DATE="2026-11-0$1T10:00:00" GIT_COMMITTER_DATE="2026-11-0$1T10:00:00" \
  git -C "$fixture" -c user.name="$2" -c user.email=dev@example.com -c commit.gpgsign=false commit -q --allow-empty -m "$3"; }
git -C "$fixture" init -q -b main
commit_at 1 Alice "Initial commit"
commit_at 2 Alice "Add README"
git -C "$fixture" switch -q -c hotfix
commit_at 3 Carol "Fix crash on startup"
git -C "$fixture" switch -q main
git -C "$fixture" switch -q -c feature-a
commit_at 3 Bob "Add login form"
commit_at 4 Bob "Validate login input"
git -C "$fixture" switch -q main
commit_at 4 Alice "Update dependencies"
GIT_COMMITTER_DATE="2026-11-05T10:00:00" git -C "$fixture" -c user.name=Alice -c user.email=dev@example.com \
  merge -q --no-ff feature-a -m "Merge branch 'feature-a'"
git -C "$fixture" switch -q -c feature-b
commit_at 6 Dave "Sketch settings page"
commit_at 7 Dave "Persist settings"
git -C "$fixture" switch -q main
commit_at 8 Alice "Release 0.1"

cargo build --quiet --manifest-path "$app_root/Cargo.toml"
wait_until() { for _ in $(seq 150); do eval "$1" && return 0; sleep 0.2; done; echo "timed out: $1" >&2; return 1; }
pgrep -x Xvfb >/dev/null || { Xvfb :99 -screen 0 1280x800x24 >/dev/null 2>&1 & }
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
# BRANCHES 見出しの次から 26px ごとに feature-a, feature-b, hotfix, main と並ぶ。
xdotool mousemove $((X + 60)) $((Y + 26 + 26 + 13)) click 1
wait_until '[ "$(git -C "$fixture" branch --show-current)" = feature-b ]'
sleep 1
import -window root "$work_dir/history-after-checkout.png"

current_branch=$(git -C "$fixture" branch --show-current)
echo "current branch after click: $current_branch"
[ "$current_branch" = feature-b ]
