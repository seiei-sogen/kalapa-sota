# View history

The right column lists every commit of every local branch, newest first, with a graph that gives each branch its own lane color. Branch names in the BRANCHES panel use the color of the lane their head commit sits on.

## Sub-features

- `history-list` shows one row per commit with summary, author, and date.
- `history-graph` draws lanes, merges, and forks.
- `history-branch-colors` colors each branch label like its head commit's lane.
- `history-current-branch` marks the current branch with ● and a highlighted row.

## How to get to it (user POV)

- Open a repository. The history shows at once. There is no other entry point.

## Driving it with kalapa_driver.sh

Preconditions:

- Fresh fixture, baseline from [README.md](./README.md).

- **Read the list.** Take a screenshot. Run `capture_screenshot history-initial`. The screen shows 7 rows, from `Sketch settings page` (Dave) at the top to `Initial commit` (Alice) at the bottom.
- **Match git.** Count commits. Run `git_in_repository log --all --oneline | wc -l`. It prints `7`, the same as the number of rows.
- **Graph shape.** Look at the screenshot. `Merge branch 'feature-a'` has two lines entering it, and `Fix crash on startup` sits on its own lane that joins at `Add README`.
- **Current branch.** Look at BRANCHES. `main` has ● and a highlighted row, matching `git_in_repository branch --show-current`.

## Gotchas

- The list is capped at 1000 commits, newest first. A larger repository shows only the newest 1000.
- Colors repeat after 8 lanes.
- The rows follow `git log --topo-order`, not date order. `Add login form` (day 3) sits above `Update dependencies` (day 4), because it belongs to the merged branch.
