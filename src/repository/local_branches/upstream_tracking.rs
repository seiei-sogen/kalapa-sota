/// ローカルブランチと、それが追跡する上流ブランチとの関係。数は最後の fetch か push の時点のもの。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpstreamTracking {
    NoUpstream,
    /// 上流ブランチを設定してあるが、リモートから消えている。
    UpstreamGone,
    Tracking {
        commits_ahead: u32,
        commits_behind: u32,
    },
}
