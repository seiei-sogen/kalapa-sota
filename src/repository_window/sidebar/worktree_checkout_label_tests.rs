use super::worktree_checkout_label::worktree_checkout_label;
use crate::repository::{CommitId, WorktreeCheckout};

#[test]
fn labels_each_worktree_by_what_it_checks_out() {
    assert_eq!(
        worktree_checkout_label(&WorktreeCheckout::Branch {
            branch_name: "feature/login".to_string(),
        }),
        "feature/login"
    );
    assert_eq!(
        worktree_checkout_label(&WorktreeCheckout::DetachedHead {
            head_commit_id: CommitId("0123456789abcdef0123456789abcdef01234567".to_string()),
        }),
        "detached 0123456"
    );
    assert_eq!(worktree_checkout_label(&WorktreeCheckout::Bare), "bare");
}
