#!/usr/bin/env bash
# kalapa をビルドし、自分で立てた Xvfb の上で <repository> を開いて、描画が始まるまで待つ。
# 使い方: launch_kalapa.sh <run-dir> <display-number> <repository>
# 起動に成功すると <run-dir>/state.env を書き、最後に "ready" で始まる行を出す。
set -euo pipefail

run_dir=$(realpath -m "${1:?run-dir を指定する}")
display_number="${2:?ディスプレイ番号を指定する (例: 91)}"
repository=$(realpath "${3:?開くリポジトリを指定する}")
repo_root=$(cd "$(dirname "$0")/../../../.." && pwd)
evidence_dir="$run_dir/evidence"
mkdir -p "$evidence_dir"
export DISPLAY=":$display_number"

wait_until() {
  for _ in $(seq 150); do eval "$1" && return 0; sleep 0.2; done
  echo "timed out: $1" >&2
  return 1
}

if [ -e "/tmp/.X11-unix/X$display_number" ] || xdotool getmouselocation >/dev/null 2>&1; then
  echo "display $DISPLAY is already in use. Pick another display number; never drive an X server this run did not start." >&2
  exit 1
fi

cargo build --quiet --manifest-path "$repo_root/Cargo.toml"

Xvfb "$DISPLAY" -screen 0 1280x800x24 >"$evidence_dir/xvfb.log" 2>&1 &
xvfb_pid=$!
# 途中で失敗しても cleanup_kalapa.sh が止められるよう、起動したらすぐ PID を書く。
cat >"$run_dir/state.env" <<EOF
RUN_DIR='$run_dir'
EVIDENCE_DIR='$evidence_dir'
REPO_ROOT='$repo_root'
REPOSITORY='$repository'
DISPLAY='$DISPLAY'
XVFB_PID=$xvfb_pid
EOF
wait_until 'xdotool getmouselocation >/dev/null 2>&1'

"$repo_root/target/debug/kalapa" "$repository" >"$evidence_dir/app-stdout.log" 2>"$evidence_dir/app-stderr.log" &
app_pid=$!
echo "APP_PID=$app_pid" >>"$run_dir/state.env"

wait_for_kalapa_window() {
  for _ in $(seq 150); do
    window=$(xdotool search --onlyvisible --name "^kalapa - " | head -1 || true)
    [ -n "$window" ] && return 0
    if ! kill -0 "$app_pid" 2>/dev/null; then
      echo "kalapa exited before opening a window. Its stderr follows. Run cleanup_kalapa.sh $run_dir next." >&2
      cat "$evidence_dir/app-stderr.log" >&2
      return 1
    fi
    sleep 0.2
  done
  echo "timed out waiting for the kalapa window. Run cleanup_kalapa.sh $run_dir next." >&2
  return 1
}
wait_for_kalapa_window
# gpui 0.2.2 はウィンドウマネージャがないと最初の MapNotify を取りこぼして描画を始めないので、map し直す。
xdotool windowunmap --sync "$window" windowmap --sync "$window"
wait_until '[ "$(import -window "$window" -format "%[fx:maxima]" info:)" != 0 ]'
eval "$(xdotool getwindowgeometry --shell "$window")"
cat >>"$run_dir/state.env" <<EOF
WINDOW=$window
WINDOW_X=$X
WINDOW_Y=$Y
WINDOW_WIDTH=$WIDTH
WINDOW_HEIGHT=$HEIGHT
EOF
echo "ready: display $DISPLAY, app pid $app_pid, window $window at ${X},${Y} size ${WIDTH}x${HEIGHT}"
