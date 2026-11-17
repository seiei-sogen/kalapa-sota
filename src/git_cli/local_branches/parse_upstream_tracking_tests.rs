use super::parse_upstream_tracking::parse_upstream_tracking;
use crate::repository::UpstreamTracking::{self, NoUpstream, Tracking, UpstreamGone};

const UPSTREAM_REFNAME: &str = "refs/remotes/origin/main";

fn tracking(commits_ahead: u32, commits_behind: u32) -> UpstreamTracking {
    Tracking {
        commits_ahead,
        commits_behind,
    }
}

#[test]
fn reads_each_upstream_track_that_git_prints() {
    assert_eq!(parse_upstream_tracking("", "").unwrap(), NoUpstream);
    for (track, expected) in [
        ("", tracking(0, 0)),
        ("ahead 2", tracking(2, 0)),
        ("behind 1", tracking(0, 1)),
        ("ahead 2, behind 1", tracking(2, 1)),
        ("gone", UpstreamGone),
    ] {
        assert_eq!(
            parse_upstream_tracking(UPSTREAM_REFNAME, track).unwrap(),
            expected,
            "{track:?}"
        );
    }
}

#[test]
fn rejects_an_upstream_track_it_does_not_know() {
    for track in ["diverged", "ahead two", "sideways 3"] {
        assert!(
            parse_upstream_tracking(UPSTREAM_REFNAME, track).is_err(),
            "{track:?}"
        );
    }
}
