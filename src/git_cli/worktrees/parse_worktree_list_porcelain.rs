use std::path::PathBuf;

use crate::git_cli::GitCommandError;
use crate::repository::{CommitId, Worktree, WorktreeCheckout, WorktreeRootPath};

pub fn parse_worktree_list_porcelain(output: &str) -> Result<Vec<Worktree>, GitCommandError> {
    output
        .split("\0\0")
        .filter(|record| !record.is_empty())
        .map(|record| {
            let unexpected_record = || GitCommandError {
                message: format!("unexpected git worktree list output: {record:?}"),
            };
            let attributes: Vec<(&str, &str)> = record
                .split('\0')
                .map(|attribute| attribute.split_once(' ').unwrap_or((attribute, "")))
                .collect();
            let [("worktree", root_path), other_attributes @ ..] = &attributes[..] else {
                return Err(unexpected_record());
            };
            // locked、prunable、知らない属性は、何をチェックアウトしているかに関わらないので読まない。
            let attribute_value = |name: &str| {
                other_attributes
                    .iter()
                    .find(|(attribute_name, _)| *attribute_name == name)
                    .map(|(_, value)| *value)
            };
            let checkout = if attribute_value("bare").is_some() {
                Some(WorktreeCheckout::Bare)
            } else if let Some(branch_refname) = attribute_value("branch") {
                branch_refname
                    .strip_prefix("refs/heads/")
                    .map(|branch_name| WorktreeCheckout::Branch {
                        branch_name: branch_name.to_string(),
                    })
            } else if attribute_value("detached").is_some() {
                attribute_value("HEAD").map(|head_commit_id| WorktreeCheckout::DetachedHead {
                    head_commit_id: CommitId(head_commit_id.to_string()),
                })
            } else {
                None
            }
            .ok_or_else(unexpected_record)?;
            Ok(Worktree {
                root_path: WorktreeRootPath(PathBuf::from(root_path)),
                checkout,
            })
        })
        .collect()
}
