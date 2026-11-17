#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitInclusion {
    Included,
    PartlyIncluded,
    Excluded,
}
