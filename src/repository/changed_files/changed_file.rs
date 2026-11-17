use super::{ChangedFileStaging, RepositoryRootRelativePath};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangedFile {
    pub path: RepositoryRootRelativePath,
    pub staging: ChangedFileStaging,
}
