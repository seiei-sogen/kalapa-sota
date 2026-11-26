#!/usr/bin/env bash
# この run で起動した kalapa と Xvfb だけを止め、試験用のリポジトリを消す。<run-dir>/evidence は残す。
# 使い方: cleanup_kalapa.sh <run-dir>
set -uo pipefail

run_dir=$(realpath -m "${1:?run-dir を指定する}")
state_file="$run_dir/state.env"
if [ -f "$state_file" ]; then
  # shellcheck disable=SC1090
  . "$state_file"
  if [ -n "${APP_PID:-}" ] && [ "$(readlink "/proc/$APP_PID/exe" 2>/dev/null)" = "$REPO_ROOT/target/debug/kalapa" ]; then
    kill "$APP_PID" && echo "stopped kalapa (pid $APP_PID)"
  fi
  if [ -n "${XVFB_PID:-}" ] && [ "$(ps -o comm= -p "$XVFB_PID")" = Xvfb ]; then
    kill "$XVFB_PID" && echo "stopped Xvfb $DISPLAY (pid $XVFB_PID)"
  fi
  # 同じディスプレイ番号ですぐ起動し直せるよう、止めたプロセスが終わるまで待つ。
  for _ in $(seq 25); do
    ps -p "${APP_PID:-0},${XVFB_PID:-0}" >/dev/null || break
    sleep 0.2
  done
  rm -f "$state_file"
fi
rm -rf "$run_dir/fixture-repo" "$run_dir"/fixture-repo-* "$run_dir/remote.git"
echo "evidence kept in $run_dir/evidence"
