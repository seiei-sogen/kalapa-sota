#!/usr/bin/env bash
# 起動中の kalapa が操作してよい状態かを、何も変えずに確かめる。問題が 1 つでもあれば終了コード 1。
# 使い方: doctor_kalapa.sh <run-dir>
set -uo pipefail

run_dir=$(realpath -m "${1:?run-dir を指定する}")
state_file="$run_dir/state.env"
[ -f "$state_file" ] || { echo "FAIL no $state_file: launch_kalapa.sh has not run for this run-dir"; exit 1; }
# shellcheck disable=SC1090
. "$state_file"
export DISPLAY
problems=0
check() {
  if eval "$2"; then echo "ok   $1"; else echo "FAIL $1"; problems=$((problems + 1)); fi
}

check "Xvfb started by this run is alive (pid ${XVFB_PID:-?})" \
  '[ -n "${XVFB_PID:-}" ] && [ "$(ps -o comm= -p "$XVFB_PID")" = Xvfb ]'
check "kalapa started by this run is alive (pid ${APP_PID:-?})" \
  '[ -n "${APP_PID:-}" ] && [ "$(readlink "/proc/$APP_PID/exe")" = "$REPO_ROOT/target/debug/kalapa" ]'
check "window ${WINDOW:-?} is visible and titled for $REPOSITORY" \
  '[ -n "${WINDOW:-}" ] && [ "$(xdotool getwindowname "$WINDOW" 2>/dev/null)" = "kalapa - $REPOSITORY" ]'
check "binary is newer than every source file, so it runs the checked-out code" \
  '[ -z "$(find "$REPO_ROOT/src" "$REPO_ROOT/Cargo.toml" -newer "$REPO_ROOT/target/debug/kalapa" -print -quit)" ]'
check "app stderr has no panic" \
  '! grep -q "panicked" "$EVIDENCE_DIR/app-stderr.log"'
check "window is drawn (not all black)" \
  '[ "$(import -window "$WINDOW" -format "%[fx:maxima]" info: 2>/dev/null)" != 0 ]'

[ "$problems" -eq 0 ]
