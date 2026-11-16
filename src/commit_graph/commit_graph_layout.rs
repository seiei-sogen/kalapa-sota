/// 履歴の 1 行に描く線。どの線もレーンを 1 本だけ持ち、そのレーンの色で描く。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommitGraphLine {
    /// 行の上端から下端まで、同じレーンをまっすぐ通る。
    PassThrough { lane: usize },
    /// 行の上端の from_lane から、この行のコミットの点へ入る。
    IntoCommit { from_lane: usize },
    /// この行のコミットの点から、行の下端の to_lane へ出る。
    OutOfCommit { to_lane: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitGraphRow {
    pub commit_lane: usize,
    pub lines: Vec<CommitGraphLine>,
}

/// rows はコミットと同じ順に 1 対 1 で並ぶ。
pub struct CommitGraphLayout {
    pub rows: Vec<CommitGraphRow>,
    pub lane_count: usize,
}
