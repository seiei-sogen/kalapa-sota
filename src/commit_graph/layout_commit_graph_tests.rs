use super::layout_commit_graph;
use super::{CommitGraphLine, CommitGraphRow};
use crate::repository::{Commit, CommitId};
use CommitGraphLine::{IntoCommit, OutOfCommit, PassThrough};

fn commit(id: &str, parent_ids: &[&str]) -> Commit {
    Commit {
        id: CommitId(id.to_string()),
        parent_ids: parent_ids
            .iter()
            .map(|parent_id| CommitId(parent_id.to_string()))
            .collect(),
        summary: String::new(),
        author_name: String::new(),
        committed_at: String::new(),
    }
}

fn row(commit_lane: usize, lines: &[CommitGraphLine]) -> CommitGraphRow {
    CommitGraphRow {
        commit_lane,
        lines: lines.to_vec(),
    }
}

// 1 行の中で線を引く順番は見た目に関係しないので、並べ替えてから比べる。
fn assert_layout(commits: &[Commit], expected_lane_count: usize, expected_rows: &[CommitGraphRow]) {
    let layout = layout_commit_graph(commits);
    let sorted = |rows: &[CommitGraphRow]| -> Vec<CommitGraphRow> {
        rows.iter()
            .map(|graph_row| {
                let mut lines = graph_row.lines.clone();
                lines.sort();
                row(graph_row.commit_lane, &lines)
            })
            .collect()
    };
    assert_eq!(sorted(&layout.rows), sorted(expected_rows));
    assert_eq!(layout.lane_count, expected_lane_count);
}

#[test]
fn linear_history_stays_in_one_lane() {
    assert_layout(
        &[commit("c", &["b"]), commit("b", &["a"]), commit("a", &[])],
        1,
        &[
            row(0, &[OutOfCommit { to_lane: 0 }]),
            row(
                0,
                &[IntoCommit { from_lane: 0 }, OutOfCommit { to_lane: 0 }],
            ),
            row(0, &[IntoCommit { from_lane: 0 }]),
        ],
    );
}

#[test]
fn merged_branch_opens_a_second_lane_and_closes_it_at_the_fork_point() {
    assert_layout(
        &[
            commit("merge", &["main", "feature"]),
            commit("main", &["base"]),
            commit("feature", &["base"]),
            commit("base", &[]),
        ],
        2,
        &[
            row(0, &[OutOfCommit { to_lane: 0 }, OutOfCommit { to_lane: 1 }]),
            row(
                0,
                &[
                    IntoCommit { from_lane: 0 },
                    PassThrough { lane: 1 },
                    OutOfCommit { to_lane: 0 },
                ],
            ),
            row(
                1,
                &[
                    PassThrough { lane: 0 },
                    IntoCommit { from_lane: 1 },
                    OutOfCommit { to_lane: 1 },
                ],
            ),
            row(
                0,
                &[IntoCommit { from_lane: 0 }, IntoCommit { from_lane: 1 }],
            ),
        ],
    );
}

#[test]
fn octopus_merge_opens_one_lane_per_parent() {
    assert_layout(
        &[
            commit("octopus", &["p1", "p2", "p3"]),
            commit("p1", &[]),
            commit("p2", &[]),
            commit("p3", &[]),
        ],
        3,
        &[
            row(
                0,
                &[
                    OutOfCommit { to_lane: 0 },
                    OutOfCommit { to_lane: 1 },
                    OutOfCommit { to_lane: 2 },
                ],
            ),
            row(
                0,
                &[
                    IntoCommit { from_lane: 0 },
                    PassThrough { lane: 1 },
                    PassThrough { lane: 2 },
                ],
            ),
            row(1, &[IntoCommit { from_lane: 1 }, PassThrough { lane: 2 }]),
            row(2, &[IntoCommit { from_lane: 2 }]),
        ],
    );
}

#[test]
fn merge_parent_already_on_a_lane_joins_that_lane() {
    assert_layout(
        &[
            commit("feature-tip", &["feature"]),
            commit("merge", &["base", "feature"]),
            commit("feature", &["base"]),
            commit("base", &[]),
        ],
        2,
        &[
            row(0, &[OutOfCommit { to_lane: 0 }]),
            row(
                1,
                &[
                    PassThrough { lane: 0 },
                    OutOfCommit { to_lane: 1 },
                    OutOfCommit { to_lane: 0 },
                ],
            ),
            row(
                0,
                &[
                    IntoCommit { from_lane: 0 },
                    PassThrough { lane: 1 },
                    OutOfCommit { to_lane: 0 },
                ],
            ),
            row(
                0,
                &[IntoCommit { from_lane: 0 }, IntoCommit { from_lane: 1 }],
            ),
        ],
    );
}

#[test]
fn unrelated_branch_reuses_a_freed_lane() {
    assert_layout(
        &[commit("x2", &["x1"]), commit("x1", &[]), commit("y1", &[])],
        1,
        &[
            row(0, &[OutOfCommit { to_lane: 0 }]),
            row(0, &[IntoCommit { from_lane: 0 }]),
            row(0, &[]),
        ],
    );
}
