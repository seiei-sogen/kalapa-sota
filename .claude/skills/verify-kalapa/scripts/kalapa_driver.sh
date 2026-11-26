# launch_kalapa.sh で起動した kalapa を、ユーザーと同じクリックとキー入力で操作する関数を読み込む。
# 使い方: . kalapa_driver.sh <run-dir>
# 座標はウィンドウ左上からの相対位置で、レイアウトの定数は src/appearance/repository_window_sizes.rs から読む。

kalapa_run_dir=$(realpath -m "${1:?run-dir を指定する}")
# shellcheck disable=SC1091
. "$kalapa_run_dir/state.env"
export DISPLAY
# 確かめるための git status が index.lock を取り、アプリの git add や git reset を失敗させないようにする。
export GIT_OPTIONAL_LOCKS=0

read_window_size_constant() {
  sed -n "s/^pub const $1: f32 = \([0-9]*\)\..*/\1/p" "$REPO_ROOT/src/appearance/repository_window_sizes.rs"
}
ROW_HEIGHT=$(read_window_size_constant ROW_HEIGHT)
SIDEBAR_WIDTH=$(read_window_size_constant SIDEBAR_WIDTH)
COMMIT_PANEL_HEIGHT=$(read_window_size_constant COMMIT_PANEL_HEIGHT)
COMMIT_PANEL_BUTTON_MIN_WIDTH=$(read_window_size_constant COMMIT_PANEL_BUTTON_MIN_WIDTH)
# render_commit_pull_push_row.rs の px_3, py_2, gap_2 と、入力欄の高さ。
COMMIT_ROW_PADDING_X=12
COMMIT_ROW_PADDING_Y=8
COMMIT_ROW_GAP=8
COMMIT_MESSAGE_INPUT_HEIGHT=30

wait_until() {
  for _ in $(seq 150); do eval "$1" && return 0; sleep 0.2; done
  echo "timed out: $1" >&2
  return 1
}

click_window_point() {
  xdotool mousemove $((WINDOW_X + $1)) $((WINDOW_Y + $2)) click 1
}

# 0 始まり。並び順は git worktree list と同じで、メインのワークツリーが先頭に来る。
click_worktree_row() {
  click_window_point $((SIDEBAR_WIDTH / 2)) $((ROW_HEIGHT * ($1 + 1) + ROW_HEIGHT / 2))
}

# 0 始まり。ブランチはブランチ名の順に並び、その上に WORKTREES の見出しとワークツリーの行、BRANCHES の見出しがある。
click_branch_row() {
  local worktree_count
  worktree_count=$(git -C "$REPOSITORY" worktree list --porcelain | grep -c '^worktree ')
  click_window_point $((SIDEBAR_WIDTH / 2)) $((ROW_HEIGHT * (worktree_count + 1 + $1 + 1) + ROW_HEIGHT / 2))
}

# 0 始まり。並び順は git status --porcelain と同じで、追跡していないファイルは最後に来る。
click_changed_file_row() {
  click_window_point $((SIDEBAR_WIDTH + 100)) \
    $((WINDOW_HEIGHT - COMMIT_PANEL_HEIGHT + ROW_HEIGHT * ($1 + 1) + ROW_HEIGHT / 2))
}

commit_row_center_y() {
  echo $((WINDOW_HEIGHT - COMMIT_ROW_PADDING_Y - COMMIT_MESSAGE_INPUT_HEIGHT / 2))
}

click_commit_message_input() {
  click_window_point $((SIDEBAR_WIDTH + COMMIT_ROW_PADDING_X + 40)) "$(commit_row_center_y)"
}

click_push_button() {
  click_window_point $((WINDOW_WIDTH - COMMIT_ROW_PADDING_X - COMMIT_PANEL_BUTTON_MIN_WIDTH / 2)) "$(commit_row_center_y)"
}

# Push のボタンのラベルが最小幅に収まるときだけ正しい。Push ↑12 ↓3 のように長いと、Pull は左へずれる。
click_pull_button() {
  click_window_point \
    $((WINDOW_WIDTH - COMMIT_ROW_PADDING_X - COMMIT_PANEL_BUTTON_MIN_WIDTH - COMMIT_ROW_GAP - COMMIT_PANEL_BUTTON_MIN_WIDTH / 2)) \
    "$(commit_row_center_y)"
}

# Pull と Push のボタンのラベルが最小幅に収まるときだけ正しい。Push ↑12 ↓3 のように長いと、Commit は左へずれる。
click_commit_button() {
  click_window_point \
    $((WINDOW_WIDTH - COMMIT_ROW_PADDING_X - 2 * (COMMIT_PANEL_BUTTON_MIN_WIDTH + COMMIT_ROW_GAP) - COMMIT_PANEL_BUTTON_MIN_WIDTH / 2)) \
    "$(commit_row_center_y)"
}

# xdotool は日本語をロケールで読むので UTF-8 を指定する。速く打つと、gpui がキー配置の変更に追いつかず文字を落とす。
type_text() {
  LC_ALL=C.UTF-8 xdotool type --delay 300 -- "$1"
}

press_keys() {
  xdotool key --delay 100 "$@"
}

# ウィンドウからいったん離れて戻る。kalapa は戻ったときにリポジトリを読み直す。
leave_and_return_to_window() {
  xdotool windowfocus "$(xdotool search --maxdepth 0 --name "")"
  sleep 0.5
  xdotool windowfocus --sync "$WINDOW"
}

capture_screenshot() {
  import -window "$WINDOW" "$EVIDENCE_DIR/$1.png"
  echo "$EVIDENCE_DIR/$1.png"
}

# 画面に出たものだけでなく、git に残った結果も証拠として残す。
record_git_state() {
  {
    echo "## git status --porcelain"
    git -C "$REPOSITORY" status --porcelain=v1
    echo "## git log -3 --format='%h %s'"
    git -C "$REPOSITORY" log -3 --format='%h %s'
    echo "## git branch -vv"
    git -C "$REPOSITORY" branch -vv
  } >"$EVIDENCE_DIR/$1.git.txt"
  echo "$EVIDENCE_DIR/$1.git.txt"
}

git_in_repository() {
  git -C "$REPOSITORY" "$@"
}
