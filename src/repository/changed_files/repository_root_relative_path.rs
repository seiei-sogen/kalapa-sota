/// git status が出すパス。どのディレクトリでリポジトリを開いても、常にリポジトリのルートからの相対パス。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryRootRelativePath(pub String);
