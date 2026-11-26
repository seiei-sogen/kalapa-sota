mod parse_upstream_tracking;
mod pull_current_branch_fast_forward_only;
mod push_current_branch;
mod read_local_branches;
mod switch_local_branch;

#[cfg(test)]
mod parse_upstream_tracking_tests;
#[cfg(test)]
mod pull_current_branch_fast_forward_only_round_trip_tests;
#[cfg(test)]
mod push_current_branch_round_trip_tests;

pub use pull_current_branch_fast_forward_only::pull_current_branch_fast_forward_only;
pub use push_current_branch::push_current_branch;
pub use read_local_branches::read_local_branches;
pub use switch_local_branch::switch_local_branch;
