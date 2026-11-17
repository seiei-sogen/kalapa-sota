#!/usr/bin/env bash
# 検証用のリポジトリと bare リモートを <run-dir> に作る。日付と作者を固定するので、毎回同じ履歴になる。
# 使い方: create_fixture_repository.sh <run-dir>
set -euo pipefail

run_dir=$(realpath -m "${1:?run-dir を指定する}")
fixture="$run_dir/fixture-repo"
remote="$run_dir/remote.git"
rm -rf "$fixture" "$remote"
mkdir -p "$fixture"

git_in_fixture() { git -C "$fixture" "$@"; }
commit_on_day() {
  GIT_AUTHOR_DATE="2026-11-0$1T10:00:00" GIT_COMMITTER_DATE="2026-11-0$1T10:00:00" \
    git_in_fixture -c user.name="$2" commit -q --allow-empty -m "$3"
}

git_in_fixture init -q -b main
git_in_fixture config user.name Alice
git_in_fixture config user.email alice@example.com
git_in_fixture config commit.gpgsign false
commit_on_day 1 Alice "Initial commit"
printf '# Fixture\n' > "$fixture/README.md"
printf '## 0.1\n' > "$fixture/CHANGELOG.md"
printf 'one\n' > "$fixture/notes.txt"
git_in_fixture add README.md CHANGELOG.md notes.txt
commit_on_day 2 Alice "Add README"
git_in_fixture switch -q -c hotfix
commit_on_day 3 Carol "Fix crash on startup"
git_in_fixture switch -q main
git_in_fixture switch -q -c feature-a
commit_on_day 3 Bob "Add login form"
git_in_fixture switch -q main
commit_on_day 4 Alice "Update dependencies"
GIT_COMMITTER_DATE="2026-11-05T10:00:00" git_in_fixture merge -q --no-ff feature-a -m "Merge branch 'feature-a'"
git_in_fixture switch -q -c feature-b
commit_on_day 6 Dave "Sketch settings page"
git_in_fixture switch -q main

git init -q --bare "$remote"
git_in_fixture remote add origin "$remote"
git_in_fixture push -q -u origin main 2>/dev/null

printf 'Usage\n' >> "$fixture/README.md"
printf -- '- staged entry\n' >> "$fixture/CHANGELOG.md"
git_in_fixture add CHANGELOG.md
printf -- '- unstaged entry\n' >> "$fixture/CHANGELOG.md"
rm "$fixture/notes.txt"
printf 'メモ\n' > "$fixture/日本語メモ.txt"

echo "fixture: $fixture"
echo "remote: $remote"
