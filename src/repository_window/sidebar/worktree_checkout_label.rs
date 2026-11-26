use crate::repository::WorktreeCheckout;

pub fn worktree_checkout_label(checkout: &WorktreeCheckout) -> String {
    match checkout {
        WorktreeCheckout::Branch { branch_name } => branch_name.clone(),
        WorktreeCheckout::DetachedHead { head_commit_id } => format!(
            "detached {}",
            head_commit_id.0.get(..7).unwrap_or(&head_commit_id.0)
        ),
        WorktreeCheckout::Bare => "bare".to_string(),
        WorktreeCheckout::UnreadableHead => "unreadable HEAD".to_string(),
    }
}
