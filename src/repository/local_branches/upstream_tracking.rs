#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpstreamTracking {
    NoUpstream,
    UpstreamGone,
    /// 数は最後の fetch か push の時点のもの。
    Tracking {
        commits_ahead: u32,
        commits_behind: u32,
    },
}
