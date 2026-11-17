use crate::git_cli::GitCommandError;
use crate::repository::UpstreamTracking;

pub fn parse_upstream_tracking(
    upstream_refname: &str,
    upstream_track: &str,
) -> Result<UpstreamTracking, GitCommandError> {
    if upstream_refname.is_empty() {
        return Ok(UpstreamTracking::NoUpstream);
    }
    if upstream_track == "gone" {
        return Ok(UpstreamTracking::UpstreamGone);
    }
    let unexpected_track = || GitCommandError {
        message: format!("unexpected upstream track from git for-each-ref: {upstream_track:?}"),
    };
    let (mut commits_ahead, mut commits_behind) = (0, 0);
    for direction_and_count in upstream_track.split(", ").filter(|part| !part.is_empty()) {
        let (direction, count) = direction_and_count
            .split_once(' ')
            .ok_or_else(unexpected_track)?;
        let count: u32 = count.parse().map_err(|_| unexpected_track())?;
        match direction {
            "ahead" => commits_ahead = count,
            "behind" => commits_behind = count,
            _ => return Err(unexpected_track()),
        }
    }
    Ok(UpstreamTracking::Tracking {
        commits_ahead,
        commits_behind,
    })
}
