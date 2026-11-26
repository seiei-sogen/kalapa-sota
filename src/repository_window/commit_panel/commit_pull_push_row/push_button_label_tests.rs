use super::push_button_label::push_button_label;
use crate::repository::UpstreamTracking::{NoUpstream, Tracking, UpstreamGone};

#[test]
fn labels_the_push_button_from_the_current_branch_upstream() {
    let tracking = |commits_ahead, commits_behind| {
        Some(Tracking {
            commits_ahead,
            commits_behind,
        })
    };
    assert_eq!(push_button_label(false, Some(NoUpstream)), "Publish");
    assert_eq!(push_button_label(false, Some(UpstreamGone)), "Publish");
    assert_eq!(push_button_label(false, tracking(0, 0)), "Push ↑0");
    assert_eq!(push_button_label(false, tracking(2, 0)), "Push ↑2");
    assert_eq!(push_button_label(false, tracking(2, 1)), "Push ↑2 ↓1");
    assert_eq!(push_button_label(false, None), "Push");
    assert_eq!(push_button_label(true, tracking(2, 0)), "Pushing…");
}
