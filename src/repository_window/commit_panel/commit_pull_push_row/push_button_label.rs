use crate::repository::UpstreamTracking;

pub fn push_button_label(
    push_in_progress: bool,
    current_branch_tracking: Option<UpstreamTracking>,
) -> String {
    if push_in_progress {
        return "Pushing…".to_string();
    }
    match current_branch_tracking {
        None => "Push".to_string(),
        // 消えた上流ブランチへの push は、リモートにブランチを作り直す。
        Some(UpstreamTracking::NoUpstream | UpstreamTracking::UpstreamGone) => {
            "Publish".to_string()
        }
        Some(UpstreamTracking::Tracking {
            commits_ahead,
            commits_behind: 0,
        }) => format!("Push ↑{commits_ahead}"),
        Some(UpstreamTracking::Tracking {
            commits_ahead,
            commits_behind,
        }) => format!("Push ↑{commits_ahead} ↓{commits_behind}"),
    }
}
