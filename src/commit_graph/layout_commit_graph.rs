use super::{CommitGraphLayout, CommitGraphLine, CommitGraphRow};
use crate::repository::{Commit, CommitId};

/// commits は子が親より先に来る順に並んでいること。
/// レーンは途中で詰め直さない。空いたレーンは次に始まる枝が再利用する。
pub fn layout_commit_graph(commits: &[Commit]) -> CommitGraphLayout {
    // 各レーンが次に通るはずのコミット。None は空きレーン。
    let mut lanes: Vec<Option<&CommitId>> = Vec::new();
    let claim_free_lane = |lanes: &mut Vec<Option<&CommitId>>| {
        lanes.iter().position(Option::is_none).unwrap_or_else(|| {
            lanes.push(None);
            lanes.len() - 1
        })
    };
    let mut rows = Vec::with_capacity(commits.len());
    let mut lane_count = 0;

    for commit in commits {
        let commit_lane = match lanes
            .iter()
            .position(|waiting| *waiting == Some(&commit.id))
        {
            Some(lane) => lane,
            None => claim_free_lane(&mut lanes),
        };
        let mut lines = Vec::new();
        for (lane, waiting) in lanes.iter_mut().enumerate() {
            match waiting {
                Some(id) if *id == &commit.id => {
                    lines.push(CommitGraphLine::IntoCommit { from_lane: lane });
                    *waiting = None;
                }
                Some(_) => lines.push(CommitGraphLine::PassThrough { lane }),
                None => {}
            }
        }
        for (parent_index, parent_id) in commit.parent_ids.iter().enumerate() {
            let parent_lane = if parent_index == 0 {
                lanes[commit_lane] = Some(parent_id);
                commit_lane
            } else if let Some(lane) = lanes.iter().position(|waiting| *waiting == Some(parent_id))
            {
                lane
            } else {
                let lane = claim_free_lane(&mut lanes);
                lanes[lane] = Some(parent_id);
                lane
            };
            lines.push(CommitGraphLine::OutOfCommit {
                to_lane: parent_lane,
            });
        }
        lane_count = lane_count.max(lanes.len());
        rows.push(CommitGraphRow { commit_lane, lines });
    }

    CommitGraphLayout { rows, lane_count }
}
